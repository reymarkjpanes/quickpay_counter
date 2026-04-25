use super::*;
use soroban_sdk::{Env, Address};

#[test]
fn test_create_job() {
    let env = Env::default();
    let contract_id = env.register(FreelanceEscrow, ());
    let client = Address::generate(&env);
    let freelancer = Address::generate(&env);

    let client_api = FreelanceEscrowClient::new(&env, &contract_id);

    assert_eq!(
        client_api.create_job(&client, &freelancer, &100),
        100
    );
}

#[test]
fn test_release_payment() {
    let env = Env::default();
    let contract_id = env.register(FreelanceEscrow, ());
    let client = Address::generate(&env);
    let freelancer = Address::generate(&env);

    let client_api = FreelanceEscrowClient::new(&env, &contract_id);

    client_api.create_job(&client, &freelancer, &100);

    assert_eq!(
        client_api.approve_and_release(&client),
        100
    );
}

#[test]
fn test_status_update() {
    let env = Env::default();
    let contract_id = env.register(FreelanceEscrow, ());
    let client = Address::generate(&env);
    let freelancer = Address::generate(&env);

    let client_api = FreelanceEscrowClient::new(&env, &contract_id);

    client_api.create_job(&client, &freelancer, &100);
    client_api.approve_and_release(&client);

    assert_eq!(client_api.get_status(), 1);
}