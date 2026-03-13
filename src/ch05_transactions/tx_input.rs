use serde::Serialize;

use crate::transaction::decode_varint;

#[derive(Debug, Clone, Serialize)]
pub struct TxIn {
    pub tx_id: [u8; 32],
    pub output_index: u32,
    pub script_sig: String,
    pub sequence: u32
}

impl TxIn {
    pub fn new(tx_id: [u8; 32], prev_index: u32, script_sig: String, sequence: u32) -> Self {
        TxIn { tx_id, output_index: prev_index, script_sig, sequence }
    }

    pub fn repr(&self) -> String {
        format!("{}:{}", hex::encode(self.tx_id), self.output_index)
    }

    pub fn parse(data: &[u8], mut index: usize) -> (Self, usize) {
        let mut displacement = 0;
        let previous_tx_id: [u8; 32] = data[index..index+32].try_into().unwrap();

        index += 32;
        displacement += 32;

        let output_index_bytes = &data[index..index+4].try_into().unwrap();
        let output_index = u32::from_le_bytes(*output_index_bytes);

        index += 4;
        displacement += 4;

        let start_index = index;
        let (script_len, new_index) = decode_varint(&data, index);
        let varint_size = new_index - start_index;
        index = new_index;
        displacement += varint_size;
        let script_buf = &data[index..index+(script_len as usize)];
        let script_sig = hex::encode(script_buf);

        index += script_len as usize;
        displacement += script_len as usize;

        let seq_bytes = &data[index..index+4].try_into().unwrap();
        let sequence = u32::from_le_bytes(*seq_bytes);

        displacement += 4;

        (TxIn { tx_id: previous_tx_id, output_index, script_sig, sequence }, displacement)
    }
}