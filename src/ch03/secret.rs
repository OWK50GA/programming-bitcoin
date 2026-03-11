use std::io::Error;

use crate::{s256_field::S256Field, s256_point::S256Point, signature::Signature};
use num_bigint::{BigUint, ToBigUint};
use rand::{RngCore, rngs::OsRng};
use secp256k1::constants::FIELD_SIZE;
use sha2::Sha256;
use hmac::{Hmac, Mac};

type HmacSha256 = Hmac<Sha256>;

pub struct PrivateKey {
    pub secret_bytes: S256Field,
    pub point: S256Point
}

impl PrivateKey {
    pub fn new() -> Self {
        let mut key = [0_u8; 32];
        OsRng.fill_bytes(&mut key);

        let felt = S256Field::from_bytes(&key);
        let point = S256Point::generate_point(felt.clone().element);

        PrivateKey { secret_bytes: felt, point }
    }

    pub fn hex(&self) -> String {
        let secret = self.secret_bytes.element.to_bytes_be();
        let hex_string = hex::encode(secret);

        hex_string
    }

    // TODO: Implement the deterministic k algorithm

    pub fn sign(self, z: S256Field) -> Result<Signature, Error> {
        let big_n = BigUint::from_bytes_be(&FIELD_SIZE);
        
        let mut k_bytes = [0_u8; 32];
        OsRng.fill_bytes(&mut k_bytes);

        let k = Self::deterministic_k(&self, z.clone());
        let r = S256Point::generate_point(k.clone().element).x.unwrap();

        let k_inv = k.inv().unwrap();
        let mut s = (z + r.clone() * self.secret_bytes) * k_inv;

        if s.element > &big_n/2.to_biguint().unwrap() {
            s = S256Field::new(big_n - s.element);
        }

        Ok(Signature { r, s })
    }

    pub fn deterministic_k(&self, mut z: S256Field) -> S256Field {
        let mut k = [0_u8; 32];
        let mut v = [0_u8; 32];

        let n_field = S256Field::from_bytes(&FIELD_SIZE);

        if z.geq(&n_field) {
            z = z - n_field.clone();
        }

        let mut z_bytes = [0_u8; 32];
        z_bytes.copy_from_slice(&z.to_bytes());

        let mut secret_bytes = [0_u8; 32];
        secret_bytes.copy_from_slice(&self.secret_bytes.to_bytes());

        let mut hmac = HmacSha256::new_from_slice(&k).expect("Invalid key");
        hmac.update(&v);
        hmac.update(&[0_u8]);
        hmac.update(&secret_bytes);
        hmac.update(&z_bytes);

        k = hmac.finalize().into_bytes().try_into().unwrap();

        let mut hmac = HmacSha256::new_from_slice(&k).expect("Invalid key");
        hmac.update(&v);
        v = hmac.finalize().into_bytes().try_into().unwrap();
        
        let mut hmac = HmacSha256::new_from_slice(&k).expect("Invalid key");
        hmac.update(&v);
        hmac.update(&[1_u8]);
        hmac.update(&secret_bytes);
        hmac.update(&z_bytes);

        k = hmac.finalize().into_bytes().try_into().unwrap();

        loop {
            let mut hmac = Hmac::<Sha256>::new_from_slice(&k).unwrap();
            hmac.update(&v);
            v = hmac.finalize().into_bytes().try_into().unwrap();
            let candidate = BigUint::from_bytes_be(&v);
            if candidate >= 1u32.to_biguint().unwrap() && candidate < n_field.element {
                return S256Field::new(candidate);
            }
            let mut hmac = Hmac::<Sha256>::new_from_slice(&k).unwrap();
            hmac.update(&v);
            hmac.update(&[0]);
            k = hmac.finalize().into_bytes().try_into().unwrap();
            let mut hmac = Hmac::<Sha256>::new_from_slice(&k).unwrap();
            hmac.update(&v);
            v = hmac.finalize().into_bytes().try_into().unwrap();
        }

    }
}