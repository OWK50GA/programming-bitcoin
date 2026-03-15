use std::io::Error;

use serde::{Serialize, Serializer, ser::SerializeStruct};
use sha2::{Digest, Sha256};

use crate::{decode_varint, encode_varint, transaction::Transaction, tx_fetcher::TxFetcher};

#[derive(Debug, Clone, Copy)]
pub struct TxId(pub [u8; 32]);

impl TxId {
    pub fn from_hash(bytes: [u8; 32]) -> Self {
        TxId(bytes)
    }

    pub fn from_raw_transaction(tx: Vec<u8>) -> Result<TxId, Error> {
        let mut hasher = Sha256::new();
        hasher.update(tx);
        let hash1 = hasher.finalize();

        let mut hasher = Sha256::new();
        hasher.update(hash1);
        let hash2 = hasher.finalize();

        Ok(TxId(hash2.into()))
    }
}

#[derive(Debug, Clone)]
pub struct TxIn {
    // pub tx_id: [u8; 32],
    pub tx_id: TxId,
    pub output_index: u32,
    pub script_sig: String,
    pub sequence: u32,
}

impl TxIn {
    pub fn new(tx_id: [u8; 32], prev_index: u32, script_sig: String, sequence: u32) -> Self {
        TxIn {
            tx_id: TxId(tx_id),
            output_index: prev_index,
            script_sig,
            sequence,
        }
    }

    pub fn repr(&self) -> String {
        format!("{}:{}", hex::encode(self.tx_id.0), self.output_index)
    }

    pub fn parse(data: &[u8], mut index: usize) -> (Self, usize) {
        let mut displacement = 0;
        let previous_tx_id: [u8; 32] = data[index..index + 32].try_into().unwrap();

        index += 32;
        displacement += 32;

        let output_index_bytes = &data[index..index + 4].try_into().unwrap();
        let output_index = u32::from_le_bytes(*output_index_bytes);

        index += 4;
        displacement += 4;

        let start_index = index;
        let (script_len, new_index) = decode_varint(data, index);
        let varint_size = new_index - start_index;
        index = new_index;
        displacement += varint_size;
        let script_buf = &data[index..index + (script_len as usize)];
        let script_sig = hex::encode(script_buf);

        index += script_len as usize;
        displacement += script_len as usize;

        let seq_bytes = &data[index..index + 4].try_into().unwrap();
        let sequence = u32::from_le_bytes(*seq_bytes);

        displacement += 4;

        (
            TxIn {
                tx_id: TxId(previous_tx_id),
                output_index,
                script_sig,
                sequence,
            },
            displacement,
        )
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        let tx_id_bytes = &self.tx_id.0;
        let output_index_bytes = &self.output_index.to_le_bytes();
        let script_sig_bytes = hex::decode(&self.script_sig).unwrap();
        let compact_size_len = encode_varint(script_sig_bytes.len() as u64);
        let sequence_bytes = &self.sequence.to_le_bytes();

        result.extend_from_slice(tx_id_bytes);
        result.extend_from_slice(output_index_bytes);
        result.extend_from_slice(&compact_size_len);
        result.extend_from_slice(&script_sig_bytes);
        result.extend_from_slice(sequence_bytes);

        result
    }

    pub async fn fetch(&self, testnet: bool) -> Transaction {
        TxFetcher::fetch(&hex::encode(self.tx_id.0), testnet, true, &mut None)
            .await
            .unwrap()
    }

    pub async fn value(&self, testnet: bool) -> u64 {
        let tx_outs = self.fetch(testnet).await.outputs;

        let index = self.output_index;
        tx_outs[index as usize].amount
    }

    pub async fn script_pubkey(&self, testnet: bool) -> String {
        let tx_outs = self.fetch(testnet).await.outputs;

        let index = self.output_index;
        let script_pubkey = &tx_outs[index as usize].script_pubkey;

        String::from(script_pubkey)
    }
}

impl Serialize for TxId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut bytes = self.0;
        bytes.reverse();
        serializer.serialize_str(&hex::encode(bytes))
    }
}

impl Serialize for TxIn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut txin = serializer.serialize_struct("TxIn", 4)?;
        txin.serialize_field("txid", &self.tx_id)?;
        txin.serialize_field("output_index", &self.output_index)?;
        txin.serialize_field("script_sig", &self.script_sig)?;
        txin.serialize_field("sequence", &self.sequence)?;

        txin.end()
    }
}
