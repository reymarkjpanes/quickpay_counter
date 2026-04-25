#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short, Address};

#[contract]
pub struct FreelanceEscrow;

const AMOUNT: Symbol = symbol_short!("AMT");
const CLIENT: Symbol = symbol_short!("CLI");
const FREELANCER: Symbol = symbol_short!("FRL");
const STATUS: Symbol = symbol_short!("STA");
const TOTAL: Symbol = symbol_short!("TOT");

#[contractimpl]
impl FreelanceEscrow {

    pub fn create_job(
        env: Env,
        client: Address,
        freelancer: Address,
        amount: u32
    ) -> u32 {

        env.storage().instance().set(&AMOUNT, &amount);
        env.storage().instance().set(&CLIENT, &client);
        env.storage().instance().set(&FREELANCER, &freelancer);
        env.storage().instance().set(&STATUS, &0u32);

        amount
    }

    pub fn approve_and_release(env: Env, caller: Address) -> u32 {

        let client: Address = env.storage().instance().get(&CLIENT).unwrap();
        let freelancer: Address = env.storage().instance().get(&FREELANCER).unwrap();
        let amount: u32 = env.storage().instance().get(&AMOUNT).unwrap_or(0);
        let status: u32 = env.storage().instance().get(&STATUS).unwrap_or(0);

        if caller != client {
            panic!("Only client can approve payment");
        }

        if status == 1 {
            panic!("Already released");
        }

        env.storage().instance().set(&STATUS, &1u32);

        let mut total: u32 = env.storage().instance().get(&TOTAL).unwrap_or(0);
        total += amount;
        env.storage().instance().set(&TOTAL, &total);

        amount
    }

    pub fn get_status(env: Env) -> u32 {
        env.storage().instance().get(&STATUS).unwrap_or(0)
    }

    pub fn get_total(env: Env) -> u32 {
        env.storage().instance().get(&TOTAL).unwrap_or(0)
    }

    pub fn get_job(env: Env) -> (Address, Address, u32) {
        let client: Address = env.storage().instance().get(&CLIENT).unwrap();
        let freelancer: Address = env.storage().instance().get(&FREELANCER).unwrap();
        let amount: u32 = env.storage().instance().get(&AMOUNT).unwrap_or(0);

        (client, freelancer, amount)
    }
}