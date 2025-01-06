#![no_std]
pub mod models;
pub mod contracts;
pub mod utils;

use soroban_sdk::{contract, contractimpl, Env};

pub fn extend_ttl(env: &Env, contract_id: &[u8]) {
    let ttl = 60 * 60 * 24 * 7; // 7 days
    env.storage().instance().extend_ttl(ttl, ttl);
}
