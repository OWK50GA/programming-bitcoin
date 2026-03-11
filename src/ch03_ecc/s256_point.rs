// TODO: There are things here meant for CURVE_ORDER, not FIELD_SIZE, and vice-versa
// and in the other two related files

// TODO: Implement base58 encoding and decoding. We are a bit
// light-headed now, that is why we are not doing it
use num_bigint::{BigUint, ToBigInt, ToBigUint};
use secp256k1::constants::{CURVE_ORDER, FIELD_SIZE, GENERATOR_X, GENERATOR_Y};

use crate::{
    ch03_ecc::s256_field::{S256Field, ToS256Field},
    signature::Signature,
};
use std::{
    io::{Error, ErrorKind},
    ops::Add,
};

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
            let zero = 0_u64.to_felts256();
            if y1 == zero {
                return Ok(S256Point::infinity(a, b));
            }

            // Doubling with non-zero y: slope = (3*x1^2 + a) / (2*y1)
            let x_squared = x1.clone().pow(2.to_bigint().unwrap());
            let numerator = x_squared * 3_u64.to_felts256() + a;
            let denominator = y1 * 2_u64.to_felts256();

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
        let a = S256A.to_felts256();
        let b = S256B.to_felts256();

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
        // ECDSA verification: all arithmetic must be done modulo CURVE_ORDER (n), not FIELD_SIZE (p)
        let n = BigUint::from_bytes_be(&CURVE_ORDER);
        
        // Convert to modulo n arithmetic
        let s_inv = sig.s.element.modinv(&n).ok_or_else(|| {
            Error::new(ErrorKind::InvalidInput, "s has no inverse mod n")
        })?;
        
        let u = (&z.element * &s_inv) % &n;
        let v = (&sig.r.element * &s_inv) % &n;

        let generator = Self::generator();
        let total = (generator.scalar_mult(u) + self.scalar_mult(v))?;

        Ok(total.x.unwrap().element == sig.r.element)
    }
}

pub fn test_point() {
    let group_hex = hex::encode(FIELD_SIZE); // -> P -> Prime for the field
    let curve_hex = hex::encode(CURVE_ORDER); // N -> group order
    println!("Field Prime, p: {group_hex}");
    println!("Curve Order, n: {curve_hex}");

    let generator = S256Point::generator();

    let infty = generator.scalar_mult(BigUint::from_bytes_be(&CURVE_ORDER));
    println!("{:#?}", infty);
}
