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

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::MockStorage;
    use cosmwasm_std::{StdError, Storage};

    #[test]
    fn test_set_contract_version() {
        let mut storage = MockStorage::new();
        let contract_name = "my_contract";
        let contract_version = "1.0.0";

        // Call the set_contract_version function
        let result = set_contract_version(&mut storage, contract_name, contract_version);
        assert!(result.is_ok());

        // Retrieve and assert the saved data
        let saved_version: ContractVersion = CONTRACT.load(&storage).unwrap();
        assert_eq!(saved_version.contract, contract_name);
        assert_eq!(saved_version.version, contract_version);
    }
}