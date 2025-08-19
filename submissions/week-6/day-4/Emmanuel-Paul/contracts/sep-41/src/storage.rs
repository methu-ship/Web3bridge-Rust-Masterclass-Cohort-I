use soroban_sdk::{contracttype, symbol_short, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug)]
pub struct AllowanceData {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[contracttype]
pub enum StorageKey {
    Balance(Address),
    Allowance(Address, Address),
    Metadata,
    Admin,
}

pub struct Storage;
