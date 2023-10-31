use cosmwasm_std::{to_binary, Addr, CosmosMsg, DepsMut, Env, Response, Uint128, WasmMsg};
use cw20::Cw20ExecuteMsg;

use crate::error::ContractError;
use crate::state::{ESCROW, VAULT, LENDERS, CONFIG, Escrow, LenderInfo, EARNINGS};

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

    if ESCROW.may_load(deps.storage)?.is_some() {
        return Err(ContractError::ExistingEscrow {});
    }

    let escrow: Escrow = Escrow {
        user: user.clone(),
        amount,
        time: env.block.time.seconds() + time,
    };

    ESCROW.save(deps.storage, &escrow)?;

    Ok(Response::default().add_attribute("action", "escrow"))
}

pub fn execute_redeem(deps: DepsMut, env: Env, user: Addr) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let escrow = ESCROW.may_load(deps.storage)?;
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

    ESCROW.remove(deps.storage);

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
    LENDERS.save(deps.storage, &lender_info)?;

    Ok(Response::default().add_attribute("action", "lend"))
}

// Release tokens back to the lender when the duration ends
pub fn release_from_pool(
    deps: DepsMut,
    env: Env,
    lender: Addr
) -> Result<Response, ContractError> {
    let lender_info = LENDERS.load(deps.storage)?;

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
    LENDERS.remove(deps.storage);

    Ok(Response::default().add_attribute("action", "release"))
}

pub fn earn(
    deps: DepsMut,
    _env: Env,  
    user: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // First, we need to add the user's tokens to the vault.
    let mut vault = VAULT.load(deps.storage)?;
    vault.total_tokens += amount;
    VAULT.save(deps.storage, &vault)?;

    // Then, record the user's contribution to a different mapping than the LENDERS one, since the logic is a bit different.
    let mut user_earnings = EARNINGS.load(deps.storage)?;
    user_earnings.amount_supplied += amount;
    user_earnings.user = user;
    EARNINGS.save(deps.storage, &user_earnings)?;

    Ok(Response::default().add_attribute("action", "earn"))
}

pub fn withdraw_for_earn(
    deps: DepsMut,
    _env: Env,
    user: Addr,
) -> Result<Response, ContractError> {
    let user_earnings = EARNINGS.load(deps.storage)?;

    // Here, compute the actual amount the user can withdraw. This can be based on various factors.
    // TODO:    
    let amount_to_withdraw = user_earnings.amount_supplied;

    // Ensure the vault has enough funds.
    let mut vault = VAULT.load(deps.storage)?;
    if vault.total_tokens < amount_to_withdraw {
        return Err(ContractError::InsufficientFunds {});
    }

    // Update the vault's total tokens.
    vault.total_tokens -= amount_to_withdraw;
    VAULT.save(deps.storage, &vault)?;

    // Reset the user's earnings to 0.
    EARNINGS.remove(deps.storage);

    Ok(Response::default().add_attribute("action", "withdraw"))
}





/*
#[test]
fn test_redeem_from_escrow() {
    let mut deps = setup_dependencies();
    let user = mock_address("user");
    let token = mock_address("token");

    // mock initialization and previous escrow action...

    // Redeem before duration should fail
    let res = execute_redeem(deps.as_mut(), mock_env(), user.clone());
    assert_eq!(res.err(), Some(ContractError::NotExpired {}));

    // Simulate passage of time
    let mut mock_env = mock_env();
    mock_env.block.time = mock_env.block.time.plus_seconds(3601);  // 1 hour + 1 second

    // Redeem after duration should succeed
    let res = execute_redeem(deps.as_mut(), mock_env, user.clone()).unwrap();
    assert_eq!(res.attributes, vec![attr("action", "redeem")]);
}

#[test]
fn test_lend_to_pool() {
    let mut deps = setup_dependencies();
    let lender = mock_address("lender");

    let amount = Uint128::from(100u128);
    let duration = 3600;  // 1 hour for simplicity

    let res = lend_to_pool(deps.as_mut(), mock_env(), lender.clone(), amount, duration).unwrap();

    assert_eq!(res.attributes, vec![attr("action", "lend")]);

    let vault = VAULT.load(&deps.storage).unwrap();
    assert_eq!(vault.total_tokens, amount);

    let lender_info = LENDERS.load(&deps.storage, &lender).unwrap();
    assert_eq!(lender_info.amount_lent, amount);
}

#[test]
fn test_release_from_pool() {
    let mut deps = setup_dependencies();
    let lender = mock_address("lender");

    // mock initialization and previous lend action...

    // Release before duration should fail
    let res = release_from_pool(deps.as_mut(), mock_env(), lender.clone());
    assert_eq!(res.err(), Some(ContractError::DurationNotMet {}));

    // Simulate passage of time
    let mut mock_env = mock_env();
    mock_env.block.time = mock_env.block.time.plus_seconds(3601);  // 1 hour + 1 second

    // Release after duration should succeed
    let res = release_from_pool(deps.as_mut(), mock_env, lender.clone()).unwrap();
    assert_eq!(res.attributes, vec![attr("action", "release")]);
}
 */