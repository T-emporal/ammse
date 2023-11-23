

use cosmwasm_std::{to_binary, Addr, CosmosMsg, DepsMut, Env, Response, Uint128, WasmMsg, BankMsg, Coin, attr};
use cw20::Cw20ExecuteMsg;

use crate::error::ContractError;
use crate::state::{ESCROW, VAULT, LENDERS, CONFIG, Escrow, LenderInfo, EARNINGS, BorrowerInfo, BORROWERS, Vault};

pub fn execute_escrow(
    deps: DepsMut,
    env: Env,
    user: Addr,
    token: Addr,
    amount: Uint128,
    time: u64,
) -> Result<Response, ContractError> {
    // let config = CONFIG.load(deps.storage)?;
    // if config.token != token {
    //     return Err(ContractError::Unauthorized {});
    // }

    if ESCROW.may_load(deps.storage)?.is_some() {
        return Err(ContractError::ExistingEscrow {});
    }

    let escrow: Escrow = Escrow {
        user: user.clone(),
        amount,
        time: env.block.time.seconds() + time,
    };

    ESCROW.save(deps.storage, &escrow)?;

    Ok(Response::new().add_attribute("action", "escrow"))
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
    let mut vault = match VAULT.load(deps.storage) {
        Ok(v) => v,
        Err(_) => Vault::default(), // Use the default if not present in storage
    };
    
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

pub fn borrow_from_pool(
    deps: DepsMut,
    env: Env,
    borrower: Addr,
    amount: Uint128,
    duration: u64
) -> Result<Response, ContractError> {
    let mut vault = match VAULT.load(deps.storage) {
        Ok(v) => v,
        Err(_) => Vault::default(), // Use the default if not present in storage
    };
    if vault.total_tokens < amount {
        return Err(ContractError::InsufficientFunds {});
    }
    vault.total_tokens -= amount;
    VAULT.save(deps.storage, &vault)?;

    let borrower_info = BorrowerInfo {
        borrower: borrower.clone(),
        amount_borrowed: amount,
        maturity_date: env.block.time.seconds() + duration,
    };
    BORROWERS.save(deps.storage, &borrower_info)?;

    Ok(Response::default().add_attribute("action", "borrow"))
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

pub fn earn_tokens_into_pool(
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

pub fn withdraw_from_pool_for_earn(
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

    Ok(Response::default().add_attribute("action", "withdraw for earn"))
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

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Timestamp};

    #[test]
    fn test_execute_escrow() {
        let mut deps = mock_dependencies();

        {
            let env = mock_env();
            let info = mock_info("user_addr", &coins(1000, "token"));
            let user = Addr::unchecked("user_addr");
            let token = Addr::unchecked("token");
            let amount = Uint128::new(500);
            let time = 60u64; // 1 minute

            // Attempt to execute escrow
            let res = execute_escrow(deps.as_mut(), env, user, token, amount, time);
            
            assert_eq!(res.unwrap().attributes,vec![attr("action", "escrow")]);

        }

        // Test Case 2: Existing Escrow
        {
            let mut deps = mock_dependencies();
        
            // Setup - Manually create an existing escrow
            let existing_escrow = Escrow {
                user: Addr::unchecked("existing_user"),
                amount: Uint128::new(1000),
                time: 12345, // Some block time
            };
            ESCROW.save(deps.as_mut().storage, &existing_escrow).unwrap();
    
            // Now call execute_escrow and expect an error
            let env = mock_env();
            let user = Addr::unchecked("new_user");
            let token = Addr::unchecked("token");
            let amount = Uint128::new(500);
            let time = 60u64;
    
            let result = execute_escrow(deps.as_mut(), env, user, token, amount, time);
            assert_eq!(result.unwrap_err().to_string(),"User has existing escrow" );

        }
          
    }

    #[test]
    fn test_successful_lend_to_pool() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let lender = Addr::unchecked("lender_address");
        let amount = Uint128::new(500);
        let duration = 60u64; // Duration in seconds

        // Call the lend_to_pool function
        let res = lend_to_pool(deps.as_mut(), env.clone(), lender.clone(), amount, duration).unwrap();

        // Assert the response is as expected
        assert_eq!(res.attributes, vec![attr("action", "lend")]);

        // Assert the vault state is updated correctly
        let vault = VAULT.load(deps.as_ref().storage).unwrap();
        assert_eq!(vault.total_tokens, amount); // or `initial_vault.total_tokens + amount` if initial state is set

        // Assert the lender info is saved correctly
        let lender_info = LENDERS.load(deps.as_ref().storage).unwrap();
        assert_eq!(lender_info.lender, lender);
        assert_eq!(lender_info.amount_lent, amount);
        assert_eq!(lender_info.maturity_date, env.block.time.seconds() + duration);
    }

    #[test]
    fn test_successful_borrow_from_pool() {
        let mut deps = mock_dependencies();

        // Setup initial vault state
        let initial_vault = Vault { total_tokens: Uint128::new(1000) };
        VAULT.save(deps.as_mut().storage, &initial_vault).unwrap();

        let env = mock_env();
        let borrower = Addr::unchecked("borrower_address");
        let amount = Uint128::new(500);
        let duration = 60u64; // Duration in seconds

        // Call the borrow_from_pool function
        let res = borrow_from_pool(deps.as_mut(), env.clone(), borrower.clone(), amount, duration).unwrap();

        // Assert the response is as expected
        assert_eq!(res.attributes, vec![attr("action", "borrow")]);

        // Assert the vault state is updated correctly
        let vault = VAULT.load(deps.as_ref().storage).unwrap();
        assert_eq!(vault.total_tokens, Uint128::new(500)); // 1000 - 500

        // Assert the borrower info is saved correctly
        let borrower_info = BORROWERS.load(deps.as_ref().storage).unwrap();
        assert_eq!(borrower_info.borrower, borrower);
        assert_eq!(borrower_info.amount_borrowed, amount);
        assert_eq!(borrower_info.maturity_date, env.block.time.seconds() + duration);
    }

    #[test]
    fn test_insufficient_funds_borrow_from_pool() {
        let mut deps = mock_dependencies();

        // Setup initial vault state with insufficient funds
        let initial_vault = Vault { total_tokens: Uint128::new(300) };
        VAULT.save(deps.as_mut().storage, &initial_vault).unwrap();

        let env = mock_env();
        let borrower = Addr::unchecked("borrower_address");
        let amount = Uint128::new(500); // More than what's in the vault
        let duration = 60u64;

        // Call the borrow_from_pool function
        let result = borrow_from_pool(deps.as_mut(), env, borrower, amount, duration);

        // Check for InsufficientFunds error
        assert!(matches!(result, Err(ContractError::InsufficientFunds {})));
    }

    #[test]
    fn test_successful_release_from_pool() {
        let mut deps = mock_dependencies();

        // Setup lender info with a past maturity date
        let lender_info = LenderInfo {
            lender: Addr::unchecked("lender_address"),
            amount_lent: Uint128::new(500),
            maturity_date: 1, // Past date
        };
        LENDERS.save(deps.as_mut().storage, &lender_info).unwrap();

        // Setup initial vault state
        let initial_vault = Vault { total_tokens: Uint128::new(1000) };
        VAULT.save(deps.as_mut().storage, &initial_vault).unwrap();

        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(2); // Current time after maturity date

        // Call the release_from_pool function
        let res = release_from_pool(deps.as_mut(), env, lender_info.lender.clone()).unwrap();

        // Assert the response and storage updates
        assert_eq!(res.attributes, vec![attr("action", "release")]);

        let vault = VAULT.load(deps.as_ref().storage).unwrap();
        assert_eq!(vault.total_tokens, Uint128::new(500)); // 1000 - 500

        // Check if lender's info is removed
        assert!(LENDERS.load(deps.as_ref().storage).is_err());
    }

    #[test]
    fn test_release_attempt_before_maturity() {
        let mut deps = mock_dependencies();
        
        let lender_info = LenderInfo {
            lender: Addr::unchecked("lender_address"),
            amount_lent: Uint128::new(500),
            maturity_date: 2, // Past date
        };
        LENDERS.save(deps.as_mut().storage, &lender_info).unwrap();

        // Setup initial vault state
        let initial_vault = Vault { total_tokens: Uint128::new(1000) };
        VAULT.save(deps.as_mut().storage, &initial_vault).unwrap();

        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(1); // Before the maturity date

        // Attempt to release funds before maturity
        let result = release_from_pool(deps.as_mut(), env, lender_info.lender);

        // Check for DurationNotMet error
        assert_eq!(result.unwrap_err().to_string(),"Duration Not Met");
    }

    #[test]
    fn test_insufficient_funds_release_from_pool() {
        let mut deps = mock_dependencies();
        
        let lender_info = LenderInfo {
            lender: Addr::unchecked("lender_address"),
            amount_lent: Uint128::new(500),
            maturity_date: 1, // Past date
        };
        LENDERS.save(deps.as_mut().storage, &lender_info).unwrap();

        // Setup vault with insufficient funds
        let initial_vault = Vault { total_tokens: Uint128::new(300) };
        VAULT.save(deps.as_mut().storage, &initial_vault).unwrap();

        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(2); // After the maturity date

        // Attempt to release more funds than available
        let result = release_from_pool(deps.as_mut(), env, lender_info.lender);

        // Check for InsufficientFunds error
        assert_eq!(result.unwrap_err().to_string(),"Insufficent Funds");
    }

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