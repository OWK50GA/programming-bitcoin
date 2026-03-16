use std::{error::Error, io::Error as IoError};

mod ch01_finite_fields;
pub use ch01_finite_fields::*;

mod ch02_elliptic_curves;
pub use ch02_elliptic_curves::*;

mod ch03_ecc;
pub use ch03_ecc::*;

mod ch04_serialization;
pub use ch04_serialization::*;

mod ch05_transactions;
pub use ch05_transactions::*;

mod ch06_script;
pub use ch06_script::*;

use crate::{ch06_script::script::Script, transaction::Transaction};

pub fn decode_transaction(transaction_hex: &str) -> Result<String, Box<dyn Error>> {
    let tx_bytes = hex::decode(transaction_hex).map_err(|e| format!("Hex decode error: {}", e))?;

    Ok(serde_json::to_string_pretty(&Transaction::parse(
        &tx_bytes,
    ))?)
}

// Reads the number, and then the index from where to begin the next read
pub fn decode_varint(data: &[u8], index: usize) -> (u64, usize) {
    let i = data[index];

    match i {
        0xfd => {
            let start = index + 1;
            let to_read = data[start..=start + 1].try_into().unwrap();
            (u16::from_le_bytes(to_read) as u64, start + 2)
        }
        0xfe => {
            let start = index + 1;
            let to_read = data[start..=start + 3].try_into().unwrap();
            (u32::from_le_bytes(to_read) as u64, start + 4)
        }
        0xff => {
            let start = index + 1;
            let to_read = data[start..=start + 7].try_into().unwrap();
            (u64::from_le_bytes(to_read), start + 8)
        }
        _ => (i as u64, index + 1),
    }
}

pub fn encode_varint(number: u64) -> Vec<u8> {
    match number {
        0..=0xfc => (number as u8).to_le_bytes().to_vec(),
        0xfd..=0xFFFF => {
            let mut bytes = vec![0xfd];
            bytes.extend_from_slice(&(number as u16).to_le_bytes());
            bytes
        }
        0x10000..=0xFFFFFFFF => {
            let mut bytes = vec![0xfe];
            bytes.extend_from_slice(&(number as u32).to_le_bytes());
            bytes
        }
        _ => {
            let mut bytes = vec![0xff];
            bytes.extend_from_slice(&number.to_le_bytes());
            bytes
        }
    }
}

pub fn parse_opcodes(codes: Vec<u8>) -> Result<Script, IoError> {
    // Send in the codes without length prefix
    let mut prefixed = encode_varint(codes.len() as u64);
    prefixed.extend_from_slice(&codes);
    Script::parse(&prefixed)
}
