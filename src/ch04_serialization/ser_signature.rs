use std::{
    fmt,
    io::{Error, ErrorKind},
};

use num_bigint::BigUint;

use crate::ser_s256_field::S256Field;

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

    pub fn to_der(&self) -> Vec<u8> {
        // to_bytes() returns big-endian; strip any leading 0x00 that BigUint may emit
        let mut r_bin = self.r.to_bytes();
        r_bin = r_bin.strip_prefix(&[0_u8]).unwrap_or(&r_bin).to_vec();

        // DER integers are signed: if the high bit is set, prepend 0x00 so it isn't
        // misread as a negative number.
        if r_bin[0] & 0x80 != 0 {
            r_bin.insert(0, 0);
        }

        // 0x02 = INTEGER tag, followed by length, then the bytes
        let mut result = vec![2, r_bin.len() as u8];

        result.extend_from_slice(&r_bin);
        let mut s_bin = self.s.to_bytes();
        s_bin = s_bin.strip_prefix(&[0]).unwrap_or(&s_bin).to_vec();

        if s_bin[0] & 0x80 != 0 {
            s_bin.insert(0, 0);
        }

        result.extend_from_slice(&[2, s_bin.len() as u8]);
        result.extend_from_slice(&s_bin);

        // 0x30 = SEQUENCE tag; der[1] is the total payload length (not including these 2 bytes)
        let mut der = vec![0x30, result.len() as u8];
        der.extend_from_slice(&result);

        der
    }

    pub fn from_der(der: &[u8]) -> Result<Self, Error> {
        if der.len() < 8 {
            return Err(Error::new(ErrorKind::InvalidData, "DER is too short"));
        }

        // Expect sequence tag 0x30
        if der[0] != 0x30 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "DER does not start with sequence tag 30",
            ));
        }

        // check signature length
        let total_len = der[1] as usize;
        if total_len != der.len().saturating_sub(2) {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "DER length mismatch: total_len {} != payload {}",
                    total_len,
                    der.len() - 2
                ),
            ));
        }

        // parse r
        let mut index = 2;
        if index >= der.len() || der[index] != 0x02 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Expected INTEGER (0x02) for r",
            ));
        }
        index += 1;

        if index >= der.len() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Unexpected end parsing r length",
            ));
        }

        let r_len = der[index] as usize;
        index += 1;

        if index + r_len > der.len() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "r length exceeds DER size",
            ));
        }

        let r_bin = &der[index..index + r_len];
        index += r_len;

        // parse s now
        if index >= der.len() || der[index] != 0x02 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Expected INTEGER (0x02) for s",
            ));
        }
        index += 1;

        if index >= der.len() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Unexpected end parsing s length",
            ));
        }

        let s_len = der[index] as usize;
        index += 1;

        if index + s_len > der.len() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "s length exceeds DER size",
            ));
        }

        let s_bin = &der[index..index + s_len];
        index += s_len;

        if index != der.len() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Extra data after DER signature",
            ));
        }

        let r_big = BigUint::from_bytes_be(r_bin);
        let s_big = BigUint::from_bytes_be(s_bin);

        let r_field = S256Field::new(r_big);
        let s_field = S256Field::new(s_big);

        Ok(Self::new(r_field, s_field))
    }
}
