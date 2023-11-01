#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{from_binary, Addr};
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Storage, Api, Querier, BankMsg, Coin};
use cw2::set_contract_version;
use cw2::Cw20ReceiveMsg;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg, Cw20HookMsg};
use crate::state::{State, STATE, ESCROWS, POOL, COLLATERALS};
use crate::execute::{execute_escrow, execute_redeem, lend_to_pool};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:ammse";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddToEscrow { amount } => {
            // Add tokens to escrow and increase liquidity in pool
            let mut escrow = ESCROWS.load(deps.storage)?;
            escrow.funds.amount += amount;
            escrow.owner = info.sender.clone();
            ESCROWS.save(deps.storage, &escrow)?;

            let mut pool = POOL.load(deps.storage)?;
            pool.liquidity.amount += amount;
            POOL.save(deps.storage, &pool)?;

            Ok(Response::new().add_attributes(vec![
                ("action", "add_to_escrow"),
                ("owner", &info.sender.to_string()),
                ("amount", &amount.to_string()),
            ]))
        }

        ExecuteMsg::BorrowFromPool { amount } => {
            // Borrow tokens from pool and decrease liquidity
            let mut pool = POOL.load(deps.storage)?;
            if pool.liquidity.amount < amount.amount {
              //  return Err(StdError::generic_err("Not enough liquidity in pool"));  TODO :: ADD Error handling
            }
            pool.liquidity.amount -= amount;
            POOL.save(deps.storage, &pool)?;

            Ok(Response::new().add_message(BankMsg::Send {
                to_address: info.sender.clone().into_string(),
                amount: vec![Coin { denom: "inj".to_string(), amount.amount }],
            }))
        }
        ExecuteMsg::AddCollateral { amount } => {
            // Add tokens as collateral
            receive_cw20(deps, _env, info, msg);
            let mut collateral = COLLATERALS.load(deps.storage)?;
            collateral.amount = amount;
            collateral.owner = info.sender.clone();
            COLLATERALS.save(deps.storage, &collateral)?;

            Ok(Response::new().add_attributes(vec![
                ("action", "add_collateral"),
                ("owner", &info.sender.to_string()),
                ("amount", &amount.to_string()),
            ]))
        }
        ExecuteMsg::ReceiveForCollateral(msg) => receive_cw20(deps, _env, info, msg),
        ExecuteMsg::RedeemForCollateral{} => execute_redeem(deps, _env, info.sender),
        ExecuteMsg::LendToPool(msg) =>receive_cw20_to_pool(deps, _env, info, msg)
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
// this is a helper to move the tokens, so the business logic is easy to read
fn send_tokens(to_address: Addr, amount: Vec<Coin>, action: &str) -> Response {
    Response::new()
        .add_message(BankMsg::Send {
            to_address: to_address.clone().into(),
            amount,
        })
        .add_attribute("action", action)
        .add_attribute("to", to_address)
}
pub mod execute {
    use super::*;

    pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.count += 1;
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "increment"))
    }

    pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }
            state.count = count;
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("action", "reset"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query::count(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetCountResponse { count: state.count })
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

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
