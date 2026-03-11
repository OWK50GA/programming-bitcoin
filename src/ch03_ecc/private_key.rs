use crate::ch03_ecc::{s256_point, signature};
use crate::s256_field::S256Field;
use hmac::{Hmac, Mac};
use num_bigint::{BigUint, ToBigUint};
use rand::{RngCore, rngs::OsRng};
use s256_point::S256Point;
use secp256k1::constants::{CURVE_ORDER};
use sha2::Sha256;
use signature::Signature;
use std::io::Error;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, Default)]
pub struct PrivateKey {
    pub secret_bytes: S256Field,
    pub point: S256Point,
}

impl PrivateKey {
    pub fn new() -> Self {
        let mut key = [0_u8; 32];
        OsRng.fill_bytes(&mut key);

        let felt = S256Field::from_bytes(&key);
        let point = S256Point::generate_point(felt.clone().element);

        PrivateKey {
            secret_bytes: felt,
            point,
        }
    }

    pub fn hex(&self) -> String {
        let secret = self.secret_bytes.element.to_bytes_be();
        hex::encode(secret)
    }

    // TODO: Implement the deterministic k algorithm

    pub fn sign(&self, z: S256Field) -> Result<Signature, Error> {
        let n = BigUint::from_bytes_be(&CURVE_ORDER);

        let k = Self::deterministic_k(self, z.clone());
        let r_point = S256Point::generate_point(k.clone().element);
        let r = r_point.x.unwrap().element.clone();

        // All arithmetic must be done modulo n (CURVE_ORDER), not p (FIELD_SIZE)
        let k_inv = k.element.modinv(&n).ok_or_else(|| {
            Error::new(std::io::ErrorKind::InvalidInput, "k has no inverse mod n")
        })?;
        
        // s = (z + r * private_key) / k mod n
        let z_mod_n = &z.element % &n;
        let private_key_mod_n = &self.secret_bytes.element % &n;
        let r_mod_n = &r % &n;
        
        let s_numerator = (z_mod_n + (r_mod_n * private_key_mod_n)) % &n;
        let mut s = (s_numerator * k_inv) % &n;

        // Use low-s value (BIP 62)
        if s > &n / 2_u64.to_biguint().unwrap() {
            s = &n - s;
        }

        Ok(Signature { 
            r: S256Field::new(r), 
            s: S256Field::new(s) 
        })
    }

    pub fn deterministic_k(&self, mut z: S256Field) -> S256Field {
        let mut k = [0_u8; 32];
        let mut v = [0_u8; 32];

        let n_field = S256Field::from_bytes(&CURVE_ORDER);

        if z.geq(&n_field) {
            z = z - n_field.clone();
        }

        let mut z_bytes = vec![];
        z_bytes.extend_from_slice(&z.to_bytes());

        let mut secret_bytes = vec![];
        secret_bytes.extend_from_slice(&self.secret_bytes.to_bytes());

        let mut hmac = HmacSha256::new_from_slice(&k).expect("Invalid key");
        hmac.update(&v);
        hmac.update(&[0_u8]);
        hmac.update(&secret_bytes);
        hmac.update(&z_bytes);

        k = hmac.finalize().into_bytes().into();

        let mut hmac = HmacSha256::new_from_slice(&k).expect("Invalid key");
        hmac.update(&v);
        v = hmac.finalize().into_bytes().into();

        let mut hmac = HmacSha256::new_from_slice(&k).expect("Invalid key");
        hmac.update(&v);
        hmac.update(&[1_u8]);
        hmac.update(&secret_bytes);
        hmac.update(&z_bytes);

        k = hmac.finalize().into_bytes().into();

        loop {
            let mut hmac = Hmac::<Sha256>::new_from_slice(&k).unwrap();
            hmac.update(&v);
            v = hmac.finalize().into_bytes().into();
            let candidate = BigUint::from_bytes_be(&v);
            if candidate >= 1u32.to_biguint().unwrap() && candidate < n_field.element {
                return S256Field::new(candidate);
            }
            let mut hmac = Hmac::<Sha256>::new_from_slice(&k).unwrap();
            hmac.update(&v);
            hmac.update(&[0]);
            k = hmac.finalize().into_bytes().into();
            let mut hmac = Hmac::<Sha256>::new_from_slice(&k).unwrap();
            hmac.update(&v);
            v = hmac.finalize().into_bytes().into();
        }
    }
}
