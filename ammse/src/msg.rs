use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

#[cw_serde]
pub enum ExecuteMsg {
    //Increment {},
    //Reset { count: i32 },
    //AddToEscrow { amount : Coin }, 
    //AddCollateral { amount : Coin},
    ReceiveForCollateral(Cw20ReceiveMsg),
    RedeemForCollateral{},
    BorrowFromPool ( Cw20ReceiveMsg ),
    LendToPool(Cw20ReceiveMsg),
    EarnToPool(Cw20ReceiveMsg),
}

#[cw_serde]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    Config {},
    Escrow { address: String },
    LendToPool { address: String },
    BorrowFromPool { address: String },
    Pool {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}

pub enum HandleMsg {
    Lend {
        unit: f64,
    },
    Borrow {
        unit: f64,
    },
}

#[cw_serde]
pub enum Cw20HookMsg {
    Escrow { time: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct EscrowResponse {
    pub amount: Uint128,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct LenderPoolResponse {
    pub amount_lent: Uint128,
    pub maturity_date: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct BorrowerPoolResponse {
    pub amount_borrowed: Uint128,
    pub maturity_date: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Pool {
    pub liquidity: Coin,  // total tokens in the pool
}