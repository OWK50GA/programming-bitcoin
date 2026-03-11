// For the to_felts256, the order should not be an argument,
// it should be Field Size from the secp256k1 library

use std::{
    fmt::{self},
    ops::{Add, Div, Mul, Sub},
};

use num_bigint::{BigInt, BigUint, ToBigInt, ToBigUint};
use secp256k1::constants::FIELD_SIZE;

#[derive(Debug, Clone)]
pub struct S256Field {
    pub order: BigUint,
    pub element: BigUint,
}

impl Add for S256Field {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.order, rhs.order);
        let mut s = self.element + rhs.element;
        s %= self.order.clone();
        S256Field {
            order: self.order,
            element: s,
        }
    }
}

impl Sub for S256Field {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.order, rhs.order);

        let n = if rhs.element > self.element {
            let quotient = &rhs.element / &self.order;
            let scalar = quotient + 1.to_biguint().unwrap();

            let multiple = scalar * &self.order;

            multiple + &self.element - &rhs.element
        } else {
            self.element - rhs.element
        };
        S256Field {
            element: n.to_biguint().unwrap(),
            order: self.order,
        }
    }
}

impl Mul for S256Field {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.order, rhs.order);
        let r = (self.element * rhs.element) % self.order.clone();

        S256Field {
            order: self.order,
            element: r,
        }
    }
}

impl Div for S256Field {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        assert_eq!(self.order, rhs.order);
        let inv = rhs.inv().expect("Division by non-invertible element");
        self * inv
    }
}

impl PartialEq for S256Field {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order && self.element == other.element
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl fmt::Display for S256Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "S256Field_{}_{}", self.element, self.order)
    }
}

pub trait ToS256Field {
    fn to_felts256(self) -> S256Field;
}

impl ToS256Field for u8 {
    fn to_felts256(self) -> S256Field {
        let order = BigUint::from_bytes_be(&FIELD_SIZE);
        assert!(self.to_biguint().unwrap() < order);
        S256Field {
            order,
            element: self.to_biguint().unwrap(),
        }
    }
}

impl ToS256Field for u16 {
    fn to_felts256(self) -> S256Field {
        let order = BigUint::from_bytes_be(&FIELD_SIZE);
        assert!(self.to_biguint().unwrap() < order);
        S256Field {
            order,
            element: self.to_biguint().unwrap(),
        }
    }
}

impl ToS256Field for u32 {
    fn to_felts256(self) -> S256Field {
        let order = BigUint::from_bytes_be(&FIELD_SIZE);
        assert!(self.to_biguint().unwrap() < order);
        S256Field {
            order,
            element: self.to_biguint().unwrap(),
        }
    }
}

impl ToS256Field for u64 {
    fn to_felts256(self) -> S256Field {
        let order = BigUint::from_bytes_be(&FIELD_SIZE);
        assert!(self.to_biguint().unwrap() < order);
        S256Field {
            order,
            element: self.to_biguint().unwrap(),
        }
    }
}

impl S256Field {
    pub fn new(mut element: BigUint) -> S256Field {
        let p = BigUint::from_bytes_be(&FIELD_SIZE);
        if element >= p {
            element %= p.clone();
        }
        S256Field { order: p, element }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let big_bytes = BigUint::from_bytes_be(bytes);

        Self::new(big_bytes)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let big_self = &self.element;
        big_self.to_bytes_be()
    }

    pub fn repr(&self) -> String {
        format!("S256Field_{}", self.element)
    }

    pub fn equals(&self, other: &Self) -> bool {
        // Not checking for order because they have to be both FIELD_SIZE
        self.element == other.element
    }

    pub fn nequals(&self, other: &Self) -> bool {
        !self.equals(other)
    }

    pub fn geq(&self, other: &Self) -> bool {
        // Not checking for order because they have to be both FIELD_SIZE
        self.element > other.element
    }

    pub fn leq(&self, other: &Self) -> bool {
        // Not checking for order because they have to be both FIELD_SIZE
        self.element < other.element
    }

    pub fn inv(&self) -> Option<Self> {
        self.element.modinv(&self.order).map(|x| S256Field {
            order: self.order.clone(),
            element: x,
        })
    }

    pub fn pow(&self, exponent: BigInt) -> Self {
        let p = &BigUint::from_bytes_be(&FIELD_SIZE);
        if exponent
            >= BigUint::from_bytes_be(&0_u64.to_be_bytes())
                .to_bigint()
                .unwrap()
        {
            // let r = Self::mod_pow(self.element as u128, exponent as u128, p) as u64;
            let r = self
                .element
                .to_bigint()
                .unwrap()
                .modpow(&exponent, &p.to_bigint().unwrap());
            S256Field {
                order: p.clone(),
                element: r.to_biguint().unwrap(),
            }
        } else {
            // let inv = Self::mod_inv(self.element as i128, self.order as i128).expect("no inverse exists for this element");
            let inv = self
                .element
                .to_bigint()
                .unwrap()
                .modinv(&self.order.to_bigint().unwrap())
                .unwrap();
            // let r = Self::mod_pow(inv as u128, (-exponent) as u128, p) as u64;
            let r = inv.modpow(&(-exponent), &p.to_bigint().unwrap());
            S256Field {
                order: p.clone(),
                element: r.to_biguint().unwrap(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::{ToBigInt, ToBigUint};

    use super::*;

    #[test]
    fn test_new() {
        let fe = S256Field::new(3_u8.to_biguint().unwrap());
        assert_eq!(fe.element, 3_u8.to_biguint().unwrap());
        assert_eq!(fe.order, BigUint::from_bytes_be(&FIELD_SIZE));
    }

    #[test]
    fn test_repr() {
        let fe = S256Field::new(3_u8.to_biguint().unwrap());
        assert_eq!(fe.repr(), "S256Field_3");
    }

    #[test]
    fn test_equals() {
        let fe1 = S256Field::new(3_u8.to_biguint().unwrap());
        let fe2 = S256Field::new(3_u8.to_biguint().unwrap());
        let fe3 = S256Field::new(4_u8.to_biguint().unwrap());
        let fe4 = S256Field::new(5_u8.to_biguint().unwrap());

        assert!(fe1 == fe2);
        assert!(!fe1.equals(&fe3));
        assert!(!fe1.equals(&fe4));
    }

    #[test]
    fn test_nequals() {
        let fe1 = S256Field::new(3_u8.to_biguint().unwrap());
        let fe2 = S256Field::new(3_u8.to_biguint().unwrap());
        let fe3 = S256Field::new(4_u8.to_biguint().unwrap());

        assert!(!fe1.nequals(&fe2));
        assert!(fe1.nequals(&fe3));
    }

    #[test]
    fn test_add() {
        let fe1 = S256Field::new(3_u8.to_biguint().unwrap());
        let fe2 = S256Field::new(4_u8.to_biguint().unwrap());
        let result = fe1 + fe2;

        assert_eq!(result.element, 7.to_biguint().unwrap()); // 3 + 4 = 7 ≡ 0 mod 7
        assert_eq!(result.order, BigUint::from_bytes_be(&FIELD_SIZE));
    }

    #[test]
    fn test_sub() {
        let fe1 = S256Field::new(3_u8.to_biguint().unwrap());
        let fe2 = S256Field::new(4_u8.to_biguint().unwrap());
        let result = fe1 - fe2;

        assert_eq!(
            result.element,
            BigUint::from_bytes_be(&FIELD_SIZE) - 1_u8.to_biguint().unwrap()
        ); // 3 - 4 = -1 ≡ 6 mod 7
        assert_eq!(result.order, BigUint::from_bytes_be(&FIELD_SIZE));
    }

    #[test]
    fn test_mul() {
        let fe1 = S256Field::new(3_u8.to_biguint().unwrap());
        let fe2 = S256Field::new(4_u8.to_biguint().unwrap());
        let result = fe1 * fe2;

        assert_eq!(result.element, 12_u8.to_biguint().unwrap()); // 3 * 4 = 12 ≡ 5 mod 7
        assert_eq!(result.order, BigUint::from_bytes_be(&FIELD_SIZE));
    }

    #[test]
    fn test_exp() {
        let fe = S256Field::new(3_u8.to_biguint().unwrap());

        // Positive exponent
        let result1 = fe.pow(2_u8.to_bigint().unwrap());
        assert_eq!(result1.element, 9_u8.to_biguint().unwrap()); // 3^2 = 9 ≡ 2 mod 7

        // Zero exponent
        let result2 = fe.pow(0_u8.to_bigint().unwrap());
        assert_eq!(result2.element, 1_u8.to_biguint().unwrap()); // 3^0 = 1

        // Negative exponent
        let result3 = fe.pow(BigInt::from(-1));
        assert_eq!(result3.element, fe.inv().unwrap().element); // 3^-1 ≡ 5 mod 7 (since 3*5=15≡1 mod 7)
    }

    #[test]
    fn test_div() {
        let fe1 = S256Field::new(3_u8.to_biguint().unwrap());
        let fe2 = S256Field::new(4_u8.to_biguint().unwrap());
        let result = fe1.clone() / fe2.clone();

        assert_eq!(result.element, (fe1 * fe2.inv().unwrap()).element);
        assert_eq!(result.order, BigUint::from_bytes_be(&FIELD_SIZE));
    }

    #[test]
    fn test_partial_eq() {
        let fe1 = S256Field::new(3_u8.to_biguint().unwrap());
        let fe2 = S256Field::new(3_u8.to_biguint().unwrap());
        let fe3 = S256Field::new(4_u8.to_biguint().unwrap());

        assert_eq!(fe1, fe2);
        assert_ne!(fe1, fe3);
    }
}
