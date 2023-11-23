#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{from_binary, Addr};
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Storage, Api, Querier, BankMsg, Coin};
use cw2::set_contract_version;
use cw20::Cw20ReceiveMsg;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg, Cw20HookMsg};
use crate::execute::{execute_escrow, execute_redeem, lend_to_pool, borrow_from_pool, earn_tokens_into_pool, withdraw_from_pool_for_earn};
use crate::query::{ query_escrow, query_borrow_to_pool, query_pool};

// version info for migration info
const CONTRACT_NAME: &str = "Temporal AMM Contracts";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
 
    Ok(Response::new().add_attribute("method", "instantiate")
                      .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ReceiveForCollateral(msg) => receive_cw20(deps, _env, info, msg),
        ExecuteMsg::RedeemForCollateral{} => execute_redeem(deps, _env, info.sender),
        ExecuteMsg::LendToPool(msg) =>receive_cw20_to_pool(deps, _env, info, msg),
        ExecuteMsg::BorrowFromPool(msg) =>borrow_cw20_from_pool(deps, _env, info, msg),
        ExecuteMsg::EarnToPool(msg) =>earn_to_pool(deps, _env, info, msg),
    }
}

pub fn receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::Escrow { time }) => execute_escrow(
            deps,
            env,
            Addr::unchecked(cw20_msg.sender),
            info.sender,
            cw20_msg.amount,
            time,
        ),
        Err(err) => Err(ContractError::Std(err)),
    }
}

pub fn receive_cw20_to_pool(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::Escrow { time }) => lend_to_pool(
            deps,
            env,
            Addr::unchecked(cw20_msg.sender),
            cw20_msg.amount,
            time,
        ),
        Err(err) => Err(ContractError::Std(err)),
    }
}

pub fn borrow_cw20_from_pool(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::Escrow { time }) => borrow_from_pool(
            deps,
            env,
            Addr::unchecked(cw20_msg.sender),
            cw20_msg.amount,
            time,
        ),
        Err(err) => Err(ContractError::Std(err)),
    }
}

pub fn earn_to_pool(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::Escrow { time }) => earn_tokens_into_pool(
            deps,
            env,
            Addr::unchecked(cw20_msg.sender),
            cw20_msg.amount,
        ),
        Err(err) => Err(ContractError::Std(err)),
    }
}

//function to release tokens from withdraw from earn
pub fn withdraw_for_earn(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::Escrow { time }) =>withdraw_from_pool_for_earn (
            deps,
            env,
            Addr::unchecked(cw20_msg.sender),
        ),
        Err(err) => Err(ContractError::Std(err)),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Escrow { address } => {
            to_binary(&query_escrow(deps, deps.api.addr_validate(&address)?)?)
        }
        QueryMsg::BorrowFromPool { address } => {
            to_binary(&query_borrow_to_pool(deps, deps.api.addr_validate(&address)?)?)   
        }
        QueryMsg::LendToPool { address } => {
            to_binary(&query_borrow_to_pool(deps, deps.api.addr_validate(&address)?)?)   
        }
        QueryMsg::Pool {} => to_binary(&query_pool(deps)?),
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};
    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &coins(1000, "BTC"));
        let msg = InstantiateMsg {  };
        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

    }

}
