use alloy_primitives::{Address, U256};
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("Insufficient balance")]
    InsufficientBalance,
    #[error("Insufficient allowance")]
    InsufficientAllowance,
}

#[derive(Clone)]
pub struct SeerToken {
    balances: Arc<Mutex<std::collections::HashMap<Address, U256>>>,
    allowances: Arc<Mutex<std::collections::HashMap<(Address, Address), U256>>>,
    total_supply: Arc<Mutex<U256>>,
}

impl SeerToken {
    pub fn new() -> Self {
        SeerToken {
            balances: Arc::new(Mutex::new(std::collections::HashMap::new())),
            allowances: Arc::new(Mutex::new(std::collections::HashMap::new())),
            total_supply: Arc::new(Mutex::new(U256::zero())),
        }
    }

    pub fn balance_of(&self, owner: Address) -> U256 {
        *self
            .balances
            .lock()
            .unwrap()
            .get(&owner)
            .unwrap_or(&U256::zero())
    }

    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        *self
            .allowances
            .lock()
            .unwrap()
            .get(&(owner, spender))
            .unwrap_or(&U256::zero())
    }

    pub fn total_supply(&self) -> U256 {
        *self.total_supply.lock().unwrap()
    }

    pub fn mint(&self, to: Address, amount: U256) {
        let mut balances = self.balances.lock().unwrap();
        let balance = balances.entry(to).or_insert(U256::zero());
        *balance += amount;

        let mut total_supply = self.total_supply.lock().unwrap();
        *total_supply += amount;
    }

    pub fn burn(&self, from: Address, amount: U256) -> Result<(), TokenError> {
        let mut balances = self.balances.lock().unwrap();
        let balance = balances
            .get_mut(&from)
            .ok_or(TokenError::InsufficientBalance)?;
        if *balance < amount {
            return Err(TokenError::InsufficientBalance);
        }
        *balance -= amount;

        let mut total_supply = self.total_supply.lock().unwrap();
        *total_supply -= amount;
        Ok(())
    }

    pub fn transfer(&self, from: Address, to: Address, amount: U256) -> Result<(), TokenError> {
        let mut balances = self.balances.lock().unwrap();
        let balance = balances
            .get_mut(&from)
            .ok_or(TokenError::InsufficientBalance)?;
        if *balance < amount {
            return Err(TokenError::InsufficientBalance);
        }
        *balance -= amount;
        *balances.entry(to).or_insert(U256::zero()) += amount;
        Ok(())
    }

    pub fn approve(&self, owner: Address, spender: Address, amount: U256) {
        let mut allowances = self.allowances.lock().unwrap();
        allowances.insert((owner, spender), amount);
    }

    pub fn transfer_from(
        &self,
        owner: Address,
        spender: Address,
        to: Address,
        amount: U256,
    ) -> Result<(), TokenError> {
        let mut allowances = self.allowances.lock().unwrap();
        let allowance = allowances
            .get_mut(&(owner, spender))
            .ok_or(TokenError::InsufficientAllowance)?;
        if *allowance < amount {
            return Err(TokenError::InsufficientAllowance);
        }
        *allowance -= amount;
        self.transfer(owner, to, amount)
    }
}
