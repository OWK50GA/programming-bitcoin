use serde::Serialize;

use crate::{decode_varint, encode_varint};

#[derive(Debug, Clone, Serialize)]
pub struct TxOut {
    pub amount: u64,
    pub script_pubkey: String,
}

impl TxOut {
    pub fn new(amount: u64, script_pubkey: String) -> Self {
        Self {
            amount,
            script_pubkey,
        }
    }

    pub fn repr(&self) -> String {
        format!("{}:{}", &self.amount, hex::encode(&self.script_pubkey))
    }

    pub fn parse(data: &[u8], mut index: usize) -> (Self, usize) {
        let amount_bytes = data[index..index + 8].try_into().unwrap();
        let amount = u64::from_le_bytes(amount_bytes);

        index += 8;

        let (script_pubkey_len, new_index) = decode_varint(data, index);

        index = new_index;
        let script_pubkey_buf = &data[index..index + (script_pubkey_len as usize)];
        let script_pubkey = hex::encode(script_pubkey_buf);

        index += script_pubkey_len as usize;

        (
            Self {
                amount,
                script_pubkey,
            },
            index,
        )
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        let amount_bytes = self.amount.to_le_bytes();
        let script_pubkey_bytes = hex::decode(&self.script_pubkey).unwrap();
        let compact_size_len = encode_varint(script_pubkey_bytes.len() as u64);

        result.extend_from_slice(&amount_bytes);
        result.extend_from_slice(&compact_size_len);
        result.extend_from_slice(&script_pubkey_bytes);

        result
    }
}
