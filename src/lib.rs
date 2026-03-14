use std::error::Error;

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

use crate::transaction::Transaction;

pub fn decode(transaction_hex: &str) -> Result<String, Box<dyn Error>> {
    let tx_bytes = hex::decode(transaction_hex).map_err(|e| format!("Hex decode error: {}", e))?;
    // let transaction = Transaction::consensus_decode(&mut tx_bytes.as_slice());
    Ok(serde_json::to_string_pretty(&Transaction::parse(&tx_bytes))?)
    // serde_json::to_string_pretty(&transaction)

    // println!("Transaction: {}", json_transaction);
    // Ok(())
}