use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Uint128, Decimal, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {
    // pub admin: Option<String>,
    // pub base_interest_rate: Decimal,
    // pub fee_percentage: Decimal,
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
    //AddToEscrow { amount : Coin }, 
    //AddCollateral { amount : Coin},
    ReceiveForCollateral(Cw20ReceiveMsg),
    RedeemForCollateral{},
    BorrowFromPool ( Cw20ReceiveMsg ),
    LendToPool(Cw20ReceiveMsg),
    EarnToPool(Cw20ReceiveMsg),
    LendToPoolV2{lender:Addr, amount: Uint128, duration:u64 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(EscrowResponse)]
    Escrow { address: String },
    #[returns(LenderPoolResponse)]
    LendToPool { address: String },
    #[returns(BorrowerPoolResponse)]
    BorrowFromPool { address: String },
    #[returns(Pool)]
    Pool {},
}

#[cw_serde]
pub enum Cw20HookMsg {
    Escrow { time: u64 },
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