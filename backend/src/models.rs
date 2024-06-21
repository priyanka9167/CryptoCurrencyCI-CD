use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: PgPool
}


#[derive(Debug, FromRow, Serialize)]
pub struct BitcoinData {
    pub name: String,
    pub bitcoin_height: i32,
    pub timestamp: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockDataFromDB{
    pub blockchain: String,
    pub block_number: i64,
    pub total_transaction: i64,
    pub gas_used: String,
    pub miner: String,
    pub time: DateTime<Utc>,
    pub difficulty: String,
    pub transactions: Vec<TransactionDataFromDB>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionDataFromDB{
    pub transaction_hash: String,
    pub time: DateTime<Utc>,
    pub from_address: Option<String>, // Change to Option<String>
    pub to_address: Option<String>, // Change to Option<String>
    pub value: Option<String>,
    pub gas: Option<String>,
    pub gas_price: Option<String>,
}


#[derive(Debug,FromRow)]
pub struct BitcoinDataFromDB {
    pub name: String,
    pub bitcoin_height: u64,
    pub timestamp: i64,
}

#[derive(Debug, Serialize,Deserialize)]
pub struct EthPrice {
    pub ethereum: EthUsd,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct EthUsd {
    pub usd: f64,

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block{
    pub blockchain: String,
    pub block_number: i64,
    pub total_transaction: i64,
    pub gas_used: String,
    pub miner: String,
    pub time: DateTime<Utc>,
    pub difficulty: String,
    pub transactions: Option<Vec<BlockchaninTransaction>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockchaninTransaction{
    pub transaction_hash: String,
    pub time: DateTime<Utc>,
    pub from_address: Option<String>, // Change to Option<String>
    pub to_address: Option<String>, // Change to Option<String>
    pub value: Option<String>,
    pub gas: Option<String>,
    pub gas_price: Option<String>,
}


// Define a struct to deserialize the JSON response
#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub name: String,
    pub height: u64,
    pub time: String,
}
