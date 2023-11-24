use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Coin, Uint128};
use cw_storage_plus:: Item;

pub static ESCROW: Item<Escrow> = Item::new("escrows:");
pub static POOL: Item<Pool> = Item::new("pool");
pub static COLLATERALS: Item<Collateral> = Item::new("collaterals:");
pub static VAULT: Item<Vault> = Item::new("vault");
pub static LENDERS: Item<LenderInfo> = Item::new("lenders:");
pub static BORROWERS: Item<BorrowerInfo> = Item::new("borrowers:");
pub const CONFIG: Item<Config> = Item::new("config");
pub const EARNINGS: Item<Earnings> = Item::new("earnings");
// Represents the collective vault where all tokens are pooled together
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq,  Eq, JsonSchema)]
pub struct Vault {
    pub total_tokens: Uint128,
}

impl Default for Vault {
    fn default() -> Self {
        Vault {
            total_tokens: Uint128::zero(),
           
        }
    }
}

// Represents an individual lender's contribution and detailsasd
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LenderInfo {
   pub lender: Addr,
   pub amount_lent: Uint128,
   pub maturity_date: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BorrowerInfo {
   pub borrower: Addr,
   pub amount_borrowed: Uint128,
   pub maturity_date: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub token: Addr,
}

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
    ReceiveForCollateral{amount: Coin},
   
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Escrow {
    pub user: Addr,
    pub amount: Uint128,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Earnings {
    pub user: Addr,            
    pub amount_supplied: Uint128,  
    pub last_updated: u64,    
    
}
