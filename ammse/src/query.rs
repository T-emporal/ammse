use cosmwasm_std::{Addr, Deps, StdError, StdResult};

use crate::msg::{ EscrowResponse, LenderPoolResponse, BorrowerPoolResponse, Pool};
use crate::state::{CONFIG, ESCROW, LENDERS, BORROWERS, POOL};

pub fn query_escrow(deps: Deps, user: Addr) -> StdResult<EscrowResponse> {
    let escrow = ESCROW.may_load(deps.storage)?;

    if escrow.is_none()  {
        return Err(StdError::generic_err("No escrow found"));
    }
    let escrows = escrow.unwrap();

    if escrows.user != user {
        return Err(StdError::generic_err("Escrow not found for user"));
    }   
        
    Ok(EscrowResponse {
        amount: escrows.amount,
        time: escrows.time,
    })
}

pub fn query_lend_to_pool(deps: Deps, user: Addr) -> StdResult<LenderPoolResponse> {
    let config = CONFIG.load(deps.storage)?;
    let lenderers = LENDERS.may_load(deps.storage)?;

    if lenderers.is_none() {
        return Err(StdError::generic_err("No Lenders found"));
    }
    let lender = lenderers.unwrap();

    if lender.lender != user {
        return Err(StdError::generic_err("Lend Tokens not found for user"));
    }

   
   Ok(LenderPoolResponse {
        amount_lent: lender.amount_lent,
        maturity_date: lender.maturity_date,
    })
}

pub fn query_borrow_to_pool(deps: Deps, user: Addr) -> StdResult<BorrowerPoolResponse> {
    let config = CONFIG.load(deps.storage)?;
    let borrowers = BORROWERS.may_load(deps.storage)?;

    if borrowers.is_none() {
        return Err(StdError::generic_err("No Lenders found"));
    }
    let borrower = borrowers.unwrap();

    if borrower.borrower != user {
        return Err(StdError::generic_err("Lend Tokens not found for user"));
    }
    
   Ok(BorrowerPoolResponse {
        amount_borrowed: borrower.amount_borrowed,
        maturity_date: borrower.maturity_date,
    })
}

pub fn query_pool(deps: Deps) -> StdResult<Pool> {
  
    let pools = POOL.load(deps.storage)?;
    let pool = pools.liquidity;

    Ok(Pool {
        liquidity: pool,
    })
}