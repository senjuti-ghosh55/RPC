#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contract]
pub struct LegacyBridgeContract;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    BankMapping(String), // Bank ID maps to Stellar Address
}

#[contractimpl]
impl LegacyBridgeContract {
    /// Register a mapping from a legacy bank ID to a Stellar address.
    pub fn register_mapping(env: Env, bank_id: String, stellar_address: Address) {
        let key = DataKey::BankMapping(bank_id);
        env.storage().instance().set(&key, &stellar_address);
    }

    /// Retrieve the Stellar address associated with a given bank ID.
    pub fn get_mapping(env: Env, bank_id: String) -> Option<Address> {
        let key = DataKey::BankMapping(bank_id);
        env.storage().instance().get(&key)
    }
}
