use std::fmt;

use crate::s256_field::S256Field;

#[derive(Debug, Clone)]
pub struct Signature {
    pub r: S256Field,
    pub s: S256Field,
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Signature({:?},{:?})", self.r, self.s)
    }
}

impl Signature {
    pub fn new(r: S256Field, s: S256Field) -> Self {
        Signature { r, s }
    }
}
