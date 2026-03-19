use serde::Serialize;
use serde::ser::SerializeStruct;
use sha2::{Digest, Sha256};

use crate::{
    decode_varint, encode_varint,
    tx_input::{TxId, TxIn},
    tx_output::TxOut,
};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub version: u32,
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub locktime: u32,
    pub testnet: bool,
}

impl Serialize for Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut tx = serializer.serialize_struct("Transaction", 5)?;
        tx.serialize_field("transaction_id", &self.compute_tx_id())?;
        tx.serialize_field("version", &self.version)?;
        tx.serialize_field("inputs", &self.inputs)?;
        tx.serialize_field("outputs", &self.outputs)?;
        tx.serialize_field("locktime", &self.locktime)?;
        tx.serialize_field("testnet", &self.testnet)?;

        tx.end()
    }
}

impl Transaction {
    pub fn new(
        version: u32,
        inputs: Vec<TxIn>,
        outputs: Vec<TxOut>,
        locktime: u32,
        testnet: bool,
    ) -> Transaction {
        Transaction {
            version,
            inputs,
            outputs,
            locktime,
            testnet,
        }
    }

    pub fn compute_tx_id(&self) -> TxId {
        let tx = hex::decode(self.serialize()).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(tx);
        let hash1 = hasher.finalize();

        let mut hasher = Sha256::new();
        hasher.update(hash1);
        let hash2 = hasher.finalize();

        TxId(hash2.into())
    }

    pub fn parse(serialization: &[u8]) -> Transaction {
        let mut index = 0;

        // Version is 4 bytes, little-endian
        let version_bytes: [u8; 4] = serialization[index..index + 4].try_into().unwrap();
        let version = u32::from_le_bytes(version_bytes);

        // Check out the stream thing on page 115
        index += 4;

        // decode_varint returns (value, new_absolute_index) — assign with = not +=
        let (input_count, new_index) = decode_varint(serialization, index);

        index = new_index;

        let mut inputs = Vec::new();

        for _ in 0..input_count {
            // TxIn::parse returns (TxIn, displacement) where displacement is bytes consumed,
            // NOT an absolute index — so use += here, not =
            let (input, displacement) = TxIn::parse(serialization, index);
            inputs.push(input);
            index += displacement;
        }

        let (output_count, new_index) = decode_varint(serialization, index);
        index = new_index;

        let mut outputs = Vec::new();

        for _ in 0..output_count {
            // TxOut::parse returns (TxOut, new_absolute_index) — assign with = not +=
            let (output, new_index) = TxOut::parse(serialization, index);
            outputs.push(output);
            index = new_index
        }

        let locktime_bytes = serialization[index..index + 4].try_into().unwrap();
        let locktime = u32::from_le_bytes(locktime_bytes);

        Transaction {
            inputs,
            version,
            outputs,
            locktime,
            testnet: true,
        }

        // serde_json::to_string_pretty(&transaction).unwrap()
    }

    pub fn serialize(&self) -> String {
        let mut result = Vec::new();

        let version_bytes = &self.version.to_le_bytes();
        result.extend_from_slice(version_bytes);

        let inputs_len = self.inputs.len();
        let inputs_len_bytes = encode_varint(inputs_len as u64);
        result.extend_from_slice(&inputs_len_bytes);

        for txin in &self.inputs {
            let ser = txin.serialize();
            result.extend_from_slice(&ser);
        }

        let outputs_len = self.outputs.len();
        let outputs_len_bytes = encode_varint(outputs_len as u64);
        result.extend_from_slice(&outputs_len_bytes);

        for txout in &self.outputs {
            let ser = txout.serialize();
            result.extend_from_slice(&ser);
        }

        let locktime_bytes = &self.locktime.to_le_bytes();
        result.extend_from_slice(locktime_bytes);

        hex::encode(result)
    }

    pub async fn get_tx_fee(&self) -> u64 {
        let mut input_amount = 0;
        let mut output_amount = 0;

        for txin in &self.inputs {
            let input_value = txin.value(true).await;
            input_amount += input_value;
        }

        for txout in &self.outputs {
            let output_val = txout.amount;
            output_amount += output_val;
        }

        input_amount - output_amount
    }
}

// impl Decodable for Transaction {
//     fn consensus_decode<R: BufRead>(r: &mut R) -> Result<Self, Error> {

//     }
// }
