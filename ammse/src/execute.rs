use cosmwasm_std::{to_binary, Addr, CosmosMsg, DepsMut, Env, Response, Uint128, WasmMsg};
use cw20::Cw20ExecuteMsg;

use crate::error::ContractError;
use crate::state::{Escrow, CONFIG, ESCROW};

pub fn execute_escrow(
    deps: DepsMut,
    env: Env,
    user: Addr,
    token: Addr,
    amount: Uint128,
    time: u64,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if config.token != token {
        return Err(ContractError::Unauthorized {});
    }

    if ESCROW.may_load(deps.storage, &user)?.is_some() {
        return Err(ContractError::ExistingEscrow {});
    }

    let escrow: Escrow = Escrow {
        user: user.clone(),
        amount,
        time: env.block.time.seconds() + time,
    };

    ESCROW.save(deps.storage, &user, &escrow)?;

    Ok(Response::default().add_attribute("action", "escrow"))
}

pub fn execute_redeem(deps: DepsMut, env: Env, user: Addr) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let escrow = ESCROW.may_load(deps.storage, &user)?;
    if escrow.is_none() {
        return Err(ContractError::NoExistingEscrow {});
    }

    let escrow = escrow.unwrap();
    if escrow.time > env.block.time.seconds() {
        return Err(ContractError::NotExpired {});
    }

    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.token.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: user.to_string(),
            amount: escrow.amount,
        })?,
        funds: vec![],
    });

    ESCROW.remove(deps.storage, &user);

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "redeem"))
}


// Lender lends tokens to the collective vault
pub fn lend_to_pool(
    deps: DepsMut,
    env: Env,
    lender: Addr,
    amount: Uint128,
    duration: u64
) -> Result<Response, ContractError> {
    let mut vault = VAULT.load(deps.storage)?;
    vault.total_tokens += amount;
    VAULT.save(deps.storage, &vault)?;

    let lender_info = LenderInfo {
        lender: lender.clone(),
        amount_lent: amount,
        maturity_date: env.block.time.seconds() + duration,
    };
    LENDERS.save(deps.storage, &lender, &lender_info)?;

    Ok(Response::default().add_attribute("action", "lend"))
}

// Release tokens back to the lender when the duration ends
pub fn release_from_pool(
    deps: DepsMut,
    env: Env,
    lender: Addr
) -> Result<Response, ContractError> {
    let lender_info = LENDERS.load(deps.storage, &lender)?;

    if env.block.time.seconds() < lender_info.maturity_date {
        return Err(ContractError::DurationNotMet {});
    }

    let mut vault = VAULT.load(deps.storage)?;
    if vault.total_tokens < lender_info.amount_lent {
        return Err(ContractError::InsufficientFunds {});
    }
    vault.total_tokens -= lender_info.amount_lent;
    VAULT.save(deps.storage, &vault)?;

    // Remove the lender's information after releasing the tokens
    LENDERS.remove(deps.storage, &lender);

    Ok(Response::default().add_attribute("action", "release"))
}
