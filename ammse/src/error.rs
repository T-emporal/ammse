use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("User has existing escrow")]
    ExistingEscrow {},

    #[error("User has no existing escrow")]
    NoExistingEscrow {},

    #[error("Escrow has not expired")]
    NotExpired {},

    #[error("Insufficent Funds")]
    InsufficientFunds{},

    #[error("Duration Not Met")]
    DurationNotMet{},
}
