use std::{fmt::Error};
use std::{io::{BufRead}};
use serde::Serialize;
use sha2::{Sha256, Digest};

use crate::{tx_input::TxIn, tx_output::TxOut};

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    pub version: u32,
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub locktime: u32,
    pub testnet: bool,
    // pub id: String,
    // pub hash: [u8; 32]
}

pub trait Decodable: Sized {
    fn consensus_decode<R: BufRead>(r: &mut R) -> Result<Self, Error>;
}

// Reads the number, and then the index from where to begin the next read
pub fn decode_varint(data: &[u8], index: usize) -> (u64, usize) {
    let i = data[index];

    match i {
        0xfd => {
            let start = index + 1;
            let to_read = data[start..=start + 1].try_into().unwrap();
            (u16::from_le_bytes(to_read) as u64, start + 2)
        },
        0xfe => {
            let start = index + 1;
            let to_read = data[start..=start + 3].try_into().unwrap();
            (u32::from_le_bytes(to_read) as u64, start + 4)
        },
        0xff => {
            let start = index + 1;
            let to_read = data[start..=start + 7].try_into().unwrap();
            (u64::from_le_bytes(to_read), start + 8)
        },
        _ =>  {
            (i as u64, index + 1)
        }
    }
}

pub fn encode_varint(number: u64) -> Vec<u8> {
    match number {
        0..=0xfc => (number as u8).to_le_bytes().to_vec(),
        0xfd..=0xFFFF => {
            let mut bytes = vec![0xfd];
            bytes.extend_from_slice(&(number as u16).to_le_bytes());
            bytes
        },
        0x10000..=0xFFFFFFFF => {
            let mut bytes = vec![0xfe];
            bytes.extend_from_slice(&(number as u32).to_le_bytes());
            bytes
        },
        _ => {
            let mut bytes = vec![0xff];
            bytes.extend_from_slice(&number.to_le_bytes());
            bytes
        }
    }
}

impl Transaction {
    pub fn new(version: u32, inputs: Vec<TxIn>, outputs: Vec<TxOut>, locktime: u32, testnet: bool) -> Transaction {
        Transaction { version, inputs, outputs, locktime, testnet }
    }

    // pub fn repr(&self) -> Result<String, Error> {
    //     let mut tx_ins: String = String::new();
    //     for txin in &self.inputs {
    //         // tx_ins.push_str(hex::encode(txin).as_str());
    //     }

    //     let mut tx_outs: String = String::new();
    //     for txout in &self.outputs {
    //         // tx_outs.push_str(hex::encode(txout).as_str());
    //     }

    //     Ok(format!("tx_version: {} tx_ins: {} tx_outs: {} locktime {}", &self.version, tx_ins, tx_outs, &self.locktime))
    // }

    // pub fn id(&self) -> String {
    //     hex::encode(self.hash())
    // }

    // pub fn hash(&self) -> Vec<u8> {
    //     let serialized = self.serialize();
    //     let mut hasher = Sha256::new();
    //     hasher.update(serialized);
    //     let hash1 = hasher.finalize();

    //     let mut hasher = Sha256::new();
    //     hasher.update(hash1);
    //     hasher.finalize().try_into().unwrap().reverse()
    // }

    pub fn parse(serialization: &[u8]) -> String {
        let mut index = 0;

        let version_bytes: [u8; 4] = serialization[index..index+4].try_into().unwrap();
        let version = u32::from_le_bytes(version_bytes);

        // Check out the stream thing on page 115
        index += 4;

        let (input_count, new_index) = decode_varint(&serialization, index);

        index = new_index;

        let mut inputs = Vec::new();

        for _ in 0..input_count {
            let (input, displacement) = TxIn::parse(&serialization, index);
            inputs.push(input);
            index += displacement;
        }

        let (output_count, new_index) = decode_varint(&serialization, index);
        index = new_index;

        let mut outputs = Vec::new();

        for _ in 0..output_count {
            let (output, new_index) = TxOut::parse(&serialization, index);
            outputs.push(output);        
            index = new_index
        }

        let locktime_bytes = serialization[index..index+4].try_into().unwrap();
        let locktime = u32::from_le_bytes(locktime_bytes);


        let transaction = Transaction {
            inputs,
            version,
            outputs,
            locktime,
            testnet: true
        };

        serde_json::to_string_pretty(&transaction).unwrap()
    }
}

// impl Decodable for Transaction {
//     fn consensus_decode<R: BufRead>(r: &mut R) -> Result<Self, Error> {
        
//     }
// }