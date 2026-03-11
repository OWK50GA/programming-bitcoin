// Finite Fields

use std::{
    fmt::{self},
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct FieldElement {
    pub order: u64,
    pub element: u64,
}

impl Add for FieldElement {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.order, rhs.order);
        let mut s = self.element + rhs.element;
        s %= self.order;
        FieldElement {
            order: self.order,
            element: s,
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.order, rhs.order);
        let n = (self.element as i128 - rhs.element as i128).rem_euclid(self.order as i128) as u64;
        FieldElement {
            element: n,
            order: self.order,
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.order, rhs.order);
        let r = ((self.element as u128 * rhs.element as u128) % self.order as u128) as u64;

        FieldElement {
            order: self.order,
            element: r,
        }
    }
}

impl Div for FieldElement {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        assert_eq!(self.order, rhs.order);
        let inv = rhs.inv().expect("Division by non-invertible element");
        self * inv
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order && self.element == other.element
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement_{}_{}", self.element, self.order)
    }
}

pub trait ToFieldElement {
    fn to_felt(self, order: u64) -> FieldElement;
}

impl ToFieldElement for u8 {
    fn to_felt(self, order: u64) -> FieldElement {
        assert!((self as u64) < order);
        FieldElement {
            order,
            element: self as u64,
        }
    }
}

impl ToFieldElement for u16 {
    fn to_felt(self, order: u64) -> FieldElement {
        assert!((self as u64) < order);
        FieldElement {
            order,
            element: self as u64,
        }
    }
}

impl ToFieldElement for u32 {
    fn to_felt(self, order: u64) -> FieldElement {
        assert!((self as u64) < order);
        FieldElement {
            order,
            element: self as u64,
        }
    }
}

impl ToFieldElement for u64 {
    fn to_felt(self, order: u64) -> FieldElement {
        assert!(self < order);
        FieldElement {
            order,
            element: self,
        }
    }
}

impl FieldElement {
    pub fn new(element: u64, order: u64) -> FieldElement {
        assert!(element < order, "Element must be less than order");
        FieldElement { order, element }
    }

    pub fn repr(&self) -> String {
        format!("FieldElement_{}_{}", self.element, self.order)
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.element == other.element && self.order == other.order
    }

    pub fn nequals(&self, other: &Self) -> bool {
        !self.equals(other)
    }

    pub fn geq(&self, other: &Self) -> bool {
        self.element > other.element && self.order == other.order
    }

    pub fn leq(&self, other: &Self) -> bool {
        self.element < other.element && self.order == other.order
    }

    fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
        if b == 0 {
            (a.abs(), a.signum(), 0)
        } else {
            let (g, x1, y1) = Self::extended_gcd(b, a % b);
            (g, y1, x1 - (a / b) * y1)
        }
    }

    fn mod_inv(a: i128, m: i128) -> Option<i128> {
        let (g, x, _) = Self::extended_gcd(a.rem_euclid(m), m);
        if g != 1 { None } else { Some(x.rem_euclid(m)) }
    }

    fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
        base %= modulus;
        let mut res = 1;
        while exp > 0 {
            if (exp & 1) == 1 {
                res = (res * base) % modulus;
            }
            base = (base * base) % modulus;
            exp >>= 1;
        }

        res
    }

    pub fn inv(&self) -> Option<Self> {
        Self::mod_inv(self.element as i128, self.order as i128).map(|x| FieldElement {
            order: self.order,
            element: x as u64,
        })
    }

    pub fn pow(&self, exponent: i128) -> Self {
        let p = self.order as u128;
        if exponent >= 0 {
            let r = Self::mod_pow(self.element as u128, exponent as u128, p) as u64;
            FieldElement {
                order: self.order,
                element: r,
            }
        } else {
            let inv = Self::mod_inv(self.element as i128, self.order as i128)
                .expect("no inverse exists for this element");
            let r = Self::mod_pow(inv as u128, (-exponent) as u128, p) as u64;
            FieldElement {
                order: self.order,
                element: r,
            }
        }
    }

    pub fn reduce(element: i128, order: u64) -> u64 {
        let mut result: u64 = 0;
        if element < order as i128 && element >= 0 {
            result = element as u64;
        } else if element >= order as i128 {
            result = (element % order as i128) as u64;
        } else if element < 0 {
            let quotient = element.abs() / order as i128;
            result = (((order as i128) * (quotient + 1) + element) % order as i128) as u64;
            // result = ((order as i128 + element) % order as i128) as u64;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let fe = FieldElement::new(3, 7);
        assert_eq!(fe.element, 3);
        assert_eq!(fe.order, 7);
    }

    #[test]
    #[should_panic(expected = "Element must be less than order")]
    fn test_new_invalid_element() {
        FieldElement::new(7, 7);
    }

    #[test]
    fn test_repr() {
        let fe = FieldElement::new(3, 7);
        assert_eq!(fe.repr(), "FieldElement_3_7");
    }

    #[test]
    fn test_equals() {
        let fe1 = FieldElement::new(3, 7);
        let fe2 = FieldElement::new(3, 7);
        let fe3 = FieldElement::new(4, 7);
        let fe4 = FieldElement::new(3, 8);

        assert!(fe1.equals(&fe2));
        assert!(!fe1.equals(&fe3));
        assert!(!fe1.equals(&fe4));
    }

    #[test]
    fn test_nequals() {
        let fe1 = FieldElement::new(3, 7);
        let fe2 = FieldElement::new(3, 7);
        let fe3 = FieldElement::new(4, 7);

        assert!(!fe1.nequals(&fe2));
        assert!(fe1.nequals(&fe3));
    }

    #[test]
    fn test_reduce() {
        // Positive within range
        assert_eq!(FieldElement::reduce(5, 7), 5);

        // Positive overflow
        assert_eq!(FieldElement::reduce(10, 7), 3);

        // Zero
        assert_eq!(FieldElement::reduce(0, 7), 0);

        // Negative
        assert_eq!(FieldElement::reduce(-3, 7), 4);

        // Negative multiple
        assert_eq!(FieldElement::reduce(-10, 7), 4);
    }

    #[test]
    fn test_add() {
        let fe1 = FieldElement::new(3, 7);
        let fe2 = FieldElement::new(4, 7);
        let result = fe1 + fe2;

        assert_eq!(result.element, 0); // 3 + 4 = 7 ≡ 0 mod 7
        assert_eq!(result.order, 7);
    }

    #[test]
    fn test_sub() {
        let fe1 = FieldElement::new(3, 7);
        let fe2 = FieldElement::new(4, 7);
        let result = fe1 - fe2;

        assert_eq!(result.element, 6); // 3 - 4 = -1 ≡ 6 mod 7
        assert_eq!(result.order, 7);
    }

    #[test]
    fn test_mul() {
        let fe1 = FieldElement::new(3, 7);
        let fe2 = FieldElement::new(4, 7);
        let result = fe1 * fe2;

        assert_eq!(result.element, 5); // 3 * 4 = 12 ≡ 5 mod 7
        assert_eq!(result.order, 7);
    }

    #[test]
    fn test_exp() {
        let fe = FieldElement::new(3, 7);

        // Positive exponent
        let result1 = fe.pow(2);
        assert_eq!(result1.element, 2); // 3^2 = 9 ≡ 2 mod 7

        // Zero exponent
        let result2 = fe.pow(0);
        assert_eq!(result2.element, 1); // 3^0 = 1

        // Negative exponent
        let result3 = fe.pow(-1);
        assert_eq!(result3.element, 5); // 3^-1 ≡ 5 mod 7 (since 3*5=15≡1 mod 7)
    }

    #[test]
    fn test_div() {
        let fe1 = FieldElement::new(3, 7);
        let fe2 = FieldElement::new(4, 7);
        let result = fe1 / fe2;

        assert_eq!(result.element, 6);
        assert_eq!(result.order, 7);
    }

    #[test]
    fn test_partial_eq() {
        let fe1 = FieldElement::new(3, 7);
        let fe2 = FieldElement::new(3, 7);
        let fe3 = FieldElement::new(4, 7);

        assert_eq!(fe1, fe2);
        assert_ne!(fe1, fe3);
    }
}
