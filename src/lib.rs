// Find all our documentation at https://docs.near.org
use near_contract_standards::storage_management::StorageBalance;
use near_sdk::json_types::U128;
use near_sdk::{ext_contract, near, AccountId, NearToken, PanicOnDefault};

// Define the contract structure
#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {}

#[ext_contract(ft_contract)]
trait FT {
    fn ft_transfer(&self, receiver_id: AccountId, amount: U128);
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance;
}

// Implement the contract structure
#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {}
    }
    #[payable]
    pub fn batch_transfer(&mut self, token_contract: AccountId, transfers: Vec<(AccountId, U128)>) {
        for (recipient, amount) in transfers {
            ft_contract::ext(token_contract.clone())
                .with_attached_deposit(NearToken::from_yoctonear(1_250_000_000_000_000_000_000))
                // .with_static_gas(Gas::from_gas(5_500_000_000_000))
                .storage_deposit(Some(recipient.clone()), Some(true))
                .then(
                    ft_contract::ext(token_contract.clone())
                        .with_attached_deposit(NearToken::from_yoctonear(1))
                        // .with_static_gas(Gas::from_gas(10_000_000_000_000))
                        .ft_transfer(recipient, amount),
                );
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;
}
