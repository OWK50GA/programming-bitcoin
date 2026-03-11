use num_bigint::{BigUint, ToBigInt, ToBigUint};
use secp256k1::constants::{FIELD_SIZE, GENERATOR_X, GENERATOR_Y};

use crate::ch04::ch04_signature::Signature;
use crate::ch04::secret::PrivateKey;
use crate::ch04::s256_field::{S256Field, ToS256Field};
use std::{
    io::{Error, ErrorKind},
    ops::Add,
};
use ripemd::{Ripemd160, Digest as RipemdDigest};
// use crate::ch02::ex02::Point;

#[derive(Debug, Clone)]
pub struct S256Point {
    pub a: S256Field,
    pub b: S256Field,
    pub x: Option<S256Field>,
    pub y: Option<S256Field>, // Option because of the point at infinity
}

impl Add for S256Point {
    type Output = Result<Self, Error>;
    fn add(self, rhs: Self) -> Self::Output {
        let order = BigUint::from_bytes_be(&FIELD_SIZE);
        if self.a != rhs.a || self.b != rhs.b {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Points are not on the same curve",
            ));
        }

        if self.x.is_none() {
            return Ok(rhs);
        }
        if rhs.x.is_none() {
            return Ok(self);
        }

        // let mut slope = 0_u64.to_felts256(self.a.order);
        let (x1, y1) = (self.x.clone().unwrap(), self.y.clone().unwrap());
        let (x2, y2) = (rhs.x.clone().unwrap(), rhs.y.clone().unwrap());
        let a = self.a.clone();
        let b = self.b.clone();

        // If x1 == x2 and y1 != y2 => P + (-P) = O
        let slope = if x1 == x2 {
            // If y1 != y2 (i.e. y1 == -y2 mod p) -> P + (-P) = O
            if y1 != y2 {
                return Ok(S256Point::infinity(a, b));
            }

            // Now y1 == y2 -> could be doubling. If y1 == 0 => tangent vertical => O
            let zero = 0_u64.to_felts256(order.clone());
            if y1 == zero {
                return Ok(S256Point::infinity(a, b));
            }

            // Doubling with non-zero y: slope = (3*x1^2 + a) / (2*y1)
            let x_squared = x1.clone().pow(2.to_bigint().unwrap());
            let numerator = x_squared * 3_u64.to_felts256(order.clone()) + a;
            let denominator = y1 * 2_u64.to_felts256(order.clone());

            // denominator should not be zero here, but double-check to avoid panic
            if denominator.element == 0.to_biguint().unwrap() {
                return Ok(S256Point::infinity(self.a, self.b));
            }
            numerator / denominator
        } else {
            // General addition case: slope = (y2 - y1) / (x2 - x1)
            let change_y = y2 - y1;
            let change_x = x2.clone() - x1.clone();

            // if change_x == 0 we are in x1 == x2 branch above, so here we expect non-zero
            if change_x.element == 0.to_biguint().unwrap() {
                return Ok(S256Point::infinity(self.a, self.b));
            }
            change_y / change_x
        };

        let x3 = slope.pow(2.to_bigint().unwrap()) - x1.clone() - rhs.x.unwrap();

        // if self.eq(rhs) {
        //     x3 = slope.pow(2) - (2_u64.to_felts256(self.a.order) * self.x.unwrap());
        // }
        let y3 = slope * (self.x.unwrap() - x3.clone()) - self.y.unwrap();

        Ok(S256Point {
            a: self.a,
            b: self.b,
            x: Some(x3),
            y: Some(y3),
        })
    }
}

const S256A: u64 = 0;
const S256B: u64 = 7;

impl S256Point {
    fn get_coefs() -> (S256Field, S256Field) {
        let a = S256A.to_felts256(BigUint::from_bytes_be(&FIELD_SIZE));
        let b = S256B.to_felts256(BigUint::from_bytes_be(&FIELD_SIZE));

        (a, b)
    }

    pub fn new(x: Option<S256Field>, y: Option<S256Field>) -> Result<Self, Error> {
        let (a, b) = Self::get_coefs();
        if x.is_none() && y.is_none() {
            return Ok(S256Point {
                a,
                b,
                x: None,
                y: None,
            });
        }

        if x.is_none() || y.is_none() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "x and y must be both some or both none",
            ));
        }

        let x_unwrapped = x.unwrap();
        let y_unwrapped = y.unwrap();

        let y_squared = y_unwrapped.clone().pow(2.to_bigint().unwrap());
        let x_cubed = x_unwrapped.clone().pow(3.to_bigint().unwrap());
        let a_x = a.clone() * x_unwrapped.clone();
        let right_side = x_cubed + a_x + b.clone();

        if y_squared != right_side {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "Point ({:?}, {:?}) does not satisfy y^2 = x^3 + {:?}*x + {:?}",
                    x_unwrapped, y_unwrapped, a, b
                ),
            ));
        }

        Ok(S256Point {
            a,
            b,
            x: Some(x_unwrapped),
            y: Some(y_unwrapped),
        })
    }

    pub fn generator() -> S256Point {
        let gx = GENERATOR_X;
        let gy = GENERATOR_Y;

        let (a, b) = Self::get_coefs();

        println!("x: {}", hex::encode(gx));
        let x = S256Field::new(BigUint::from_bytes_be(&gx));
        let y = S256Field::new(BigUint::from_bytes_be(&gy));

        S256Point {
            a,
            b,
            x: Some(x),
            y: Some(y),
        }
    }

    pub fn infinity(a: S256Field, b: S256Field) -> Self {
        S256Point {
            a,
            b,
            x: None,
            y: None,
        }
    }

    pub fn eq(&self, other: Self) -> bool {
        if self.a == other.a && self.b == other.b {
            return false;
        }

        match (self.x.clone(), self.y.clone(), other.x, other.y) {
            (None, None, None, None) => true, // both infinity
            (Some(sx), Some(sy), Some(ox), Some(oy)) => sx == ox && sy == oy,
            _ => false, // one is infinity, the other isn't
        }
    }

    pub fn neq(&self, other: Self) -> bool {
        !self.eq(other)
    }

    pub fn is_valid_point(point: Self) -> Result<bool, Error> {
        if point.x.is_none() && point.y.is_none() {
            return Ok(true);
        }
        let y = point.y.unwrap();
        let x = point.x.unwrap();

        let y_squared = y.pow(2.to_bigint().unwrap());
        let x_cubed = x.pow(3.to_bigint().unwrap());
        let a_x = point.a * x;

        let right_side = x_cubed + a_x + point.b;

        Ok(y_squared == right_side)
    }

    pub fn scalar_mult(&self, scalar: BigUint) -> Self {
        let mut coef = scalar;
        let mut current = self.clone();
        let mut result = Self::infinity(self.a.clone(), self.b.clone());

        while coef > 0.to_biguint().unwrap() {
            if (coef.clone() & 1.to_biguint().unwrap()) == 1.to_biguint().unwrap() {
                result = (result + current.clone()).unwrap();
            }
            current = (current.clone() + current).unwrap();
            coef >>= 1;
        }

        result
    }

    pub fn generate_point(scalar: BigUint) -> Self {
        let generator = Self::generator();
        generator.scalar_mult(scalar)
    }

    pub fn verify_sig(&self, z: S256Field, sig: Signature) -> Result<bool, Error> {
        let u = z / sig.s.clone();
        let v = sig.r.clone() / sig.s.clone();

        let generator = Self::generator();
        let total = (generator.scalar_mult(u.element) + self.scalar_mult(v.element))?;

        Ok(total.x.unwrap().element == sig.r.element)
    }

    pub fn sec(&self, compressed: bool) -> Vec<u8> {
        let x = self.x.as_ref().unwrap();
        let y = self.y.as_ref().unwrap();

        let y_parity = &y.element % 2.to_biguint().unwrap();

        let x_bytes = x.element.to_bytes_be();
        if compressed {
            let mut sec_format_key = [0_u8; 33];
            let parity_byte = if y_parity == 0.to_biguint().unwrap() {
                [2_u8]
            } else {
                [3_u8]
            };
            sec_format_key.copy_from_slice(&parity_byte);

            sec_format_key.copy_from_slice(&x_bytes);

            sec_format_key.to_vec()
        } else {
            let mut sec_format_key = [0_u8; 65];
            sec_format_key.copy_from_slice(&[4_u8]);
            let y_bytes = y.element.to_bytes_be();

            sec_format_key.copy_from_slice(&x_bytes);
            sec_format_key.copy_from_slice(&y_bytes);

            sec_format_key.to_vec()
        }
    }

    pub fn parse(&self, sec_bin: Vec<u8>) -> Self {
        // returns a Point object from a SEC binary (not hex)
        let p = S256Field::from_bytes(&FIELD_SIZE);
        if sec_bin[0] == 4 {
            let x = &sec_bin[1..33];
            let y = &sec_bin[33..];

            let x_int = S256Field::from_bytes(x);
            let y_int = S256Field::from_bytes(y);

            return Self::new(Some(x_int), Some(y_int)).unwrap();
        }

        let is_even = sec_bin[0] == 2;
        let x = S256Field::from_bytes(&sec_bin[1..]);
        let alpha = x.pow(3.to_bigint().unwrap()) + S256B.to_felts256(x.clone().order);

        let beta = alpha.sqrt();

        let (even_beta, odd_beta) =
            if beta.clone().element % 2.to_biguint().unwrap() == 0.to_biguint().unwrap() {
                (beta.clone(), p - beta.clone())
            } else {
                (p - beta.clone(), beta)
            };

        if is_even {
            Self::new(Some(x.clone()), Some(even_beta)).unwrap()
        } else {
            Self::new(Some(x), Some(odd_beta)).unwrap()
        }
    }

    fn hash160(s: &[u8]) -> Vec<u8> {
        let mut hasher = Ripemd160::new();
        hasher.update(s);

        hasher.finalize().to_vec()
    }

    fn point_hash160(&self, compressed: bool) -> Vec<u8> {
        Self::hash160(&self.sec(compressed))
    }

    pub fn address(&self, compressed: bool, testnet: bool) -> String {
        let h160 = self.point_hash160(compressed);

        let prefix: [u8; 1] = if testnet {
            [0x6f]
        } else {
            [0x00]
        };

        let mut encode_string = vec![];
        encode_string.extend_from_slice(&prefix);
        encode_string.extend_from_slice(&h160);
        PrivateKey::encode_base58_checksum(&encode_string)
    }
}   
