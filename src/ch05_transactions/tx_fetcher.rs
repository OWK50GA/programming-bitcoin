use crate::transaction::Transaction;
use reqwest;
use std::{collections::HashMap, io::Cursor};

#[derive(Debug, Clone, Copy)]
pub struct TxFetcher {}

impl TxFetcher {
    pub fn get_url(testnet: bool) -> String {
        if testnet {
            "http://testnet.programmingbitcoin.com/".to_string()
        } else {
            "http://mainnet.programmingbitcoin.com/".to_string()
        }
    }

    pub async fn fetch(
        tx_id: &str,
        testnet: bool,
        fresh: bool,
        cache: &mut Option<HashMap<String, Transaction>>,
    ) -> Result<Transaction, reqwest::Error> {
        if fresh || !cache.clone().unwrap().contains_key(tx_id) {
            let url = format!("{}/tx/{}.hex", Self::get_url(testnet), tx_id);
            let response = reqwest::get(&url).await?;
            let text = response.text().await?;
            let raw = hex::decode(text.trim()).unwrap();

            let tx = Transaction::parse(&Cursor::new(raw).into_inner());
            cache.clone().unwrap().insert(tx_id.to_string(), tx.clone());
            Ok(tx)
        } else {
            Ok(cache.clone().unwrap()[tx_id].clone())
        }
    }
}
