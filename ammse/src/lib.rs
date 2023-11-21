pub mod contract;
mod error;
pub mod integration_tests;
pub mod msg;
pub mod state;
pub mod execute;
use cosmwasm_std::{Storage, StdResult};
use cw2::{ContractVersion, CONTRACT};

pub use crate::error::ContractError;
pub mod query;


pub fn set_contract_version<T: Into<String>, U: Into<String>>(
    store: &mut dyn Storage,
    name: T,
    version: U,
) -> StdResult<()> {
    let val = ContractVersion {
        contract: name.into(),
        version: version.into(),
    };
    CONTRACT.save(store, &val)
}