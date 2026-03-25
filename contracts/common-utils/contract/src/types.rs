// contracts/common-utils/src/types.rs

use soroban_sdk::{contracttype, Address, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct PriceFeed {
    pub asset: symbol_short,
    pub price: i128,
    pub timestamp: u64,
    pub confidence: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct ScoreUpdate {
    pub user: Address,
    pub score: u32,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct FraudAlert {
    pub user: Address,
    pub fraud_type: symbol_short,
    pub severity: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct MarketData {
    pub volatility: u32,
    pub correlation: u32,
}