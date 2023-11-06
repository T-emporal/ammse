use cosmwasm_schema::{cw_serde, QueryResponses};
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
    AddToEscrow { amount : Coin },
    BorrowFromPool { amount : Coin },
    AddCollateral { amount : Coin},
    ReceiveForCollateral(Cw20ReceiveMsg),
    RedeemForCollateral{},
    LendToPool(Cw20ReceiveMsg),
    EarnToPool(Cw20ReceiveMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
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
