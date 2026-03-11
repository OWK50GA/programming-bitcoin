use std::fmt;

use crate::ch04_field::S256Field;

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

    pub fn der(&self) -> Vec<u8> {
        let mut r_bin = self.r.to_bytes();
        r_bin = r_bin.strip_prefix(&[0_u8]).unwrap_or(&r_bin).to_vec();

        if r_bin[0] & 0x80 != 0 {
            r_bin.to_vec().insert(0, 0);
        }

        let mut result = vec![2, r_bin.len() as u8];

        result.extend_from_slice(&r_bin);
        let mut s_bin = self.s.to_bytes();
        s_bin = s_bin.strip_prefix(&[0]).unwrap_or(&s_bin).to_vec();

        if s_bin[0] & 0x80 != 0 {
            s_bin.insert(0, 0);
        }

        result.extend_from_slice(&[2, s_bin.len() as u8]);
        result.extend_from_slice(&s_bin);

        let mut der = vec![0x30, result.len() as u8];
        der.extend_from_slice(&result);

        der
    }
}
