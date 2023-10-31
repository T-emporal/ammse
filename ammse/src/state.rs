use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, Uint128};
use cw_storage_plus:: Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");
pub static ESCROW: Item<Escrow> = Item::new("escrows:");
pub static POOL: Item<Pool> = Item::new("pool");
pub static COLLATERALS: Item<Collateral> = Item::new("collaterals:");
pub static VAULT: Item<Vault> = Item::new("vault");
pub static LENDERS: Item<LenderInfo> = Item::new("lenders:");
pub const CONFIG: Item<Config> = Item::new("config");

//pub static ASSETS: Map<Addr, Asset> = Map::new("assets");
//
//pub struct Asset {
//    pub owner: Addr,    // Address of the owner.
//    pub unit: f64,               // Amount/Units of the asset.
//    pub interest: f64,           // Calculated interest for this asset.
//}
//
// Represents the collective vault where all tokens are pooled together
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Vault {
    pub total_tokens: Uint128,
}

// Represents an individual lender's contribution and detailsasd
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LenderInfo {
   pub lender: Addr,
   pub amount_lent: Uint128,
   pub maturity_date: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub token: Addr,
}


//#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
//pub struct Escrow {
//    pub owner: Addr,
//    pub funds: Coin,  // amount of tokens stored
//}
//
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Pool {
    pub liquidity: Coin,  // total tokens in the pool
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Collateral {
    pub owner: Addr,
    pub amount: Coin,
}

// Execute messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddToEscrow { amount: Coin },
    BorrowFromPool { amount: Coin },
    AddCollateral { amount: Coin },
    // ... you can add more actions if needed
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Escrow {
    pub user: Addr,
    pub amount: Uint128,
    pub time: u64,
}