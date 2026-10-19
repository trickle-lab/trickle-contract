#![cfg(test)]
use super::*;
use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::Address;

fn create_token_contract<'a>(
    env: &Env,
    admin: &Address,
) -> (token::Client<'a>, token::StellarAssetClient<'a>) {
    let sac = env.register_stellar_asset_contract_v2(admin.clone());
    (
        token::Client::new(env, &sac.address()),
        token::StellarAssetClient::new(env, &sac.address()),
    )
}

/// Registers the Trickle contract and a test token, and gives `sender` `funds` of that token.
/// Returns (client, token, sender, recipient, contract address).
fn setup<'a>(
    env: &Env,
    funds: i128,
) -> (ContractClient<'a>, token::Client<'a>, Address, Address, Address) {
    env.mock_all_auths();
    let admin = Address::generate(env);
    let sender = Address::generate(env);
    let recipient = Address::generate(env);
    let (token_client, token_admin) = create_token_contract(env, &admin);
    token_admin.mint(&sender, &funds);
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(env, &contract_id);
    (client, token_client, sender, recipient, contract_id)
}

fn advance(env: &Env, seconds: u64) {
    env.ledger().with_mut(|li| li.timestamp += seconds);
}

#[test]
fn test_create_and_full_withdraw() {
    let env = Env::default();
    let (client, token_client, sender, recipient, contract_id) = setup(&env, 1000);

    let stream_id = client.create_stream(&sender, &recipient, &token_client.address, &1000, &100);

    // sender's funds moved into the contract
    assert_eq!(token_client.balance(&sender), 0);
    assert_eq!(token_client.balance(&contract_id), 1000);

    // halfway through, half is vested
    advance(&env, 50);
    assert_eq!(client.balance(&stream_id), 500);
    assert_eq!(client.withdraw(&stream_id), 500);
    assert_eq!(token_client.balance(&recipient), 500);

    // past the end, the remaining half vests
    advance(&env, 100);
    assert_eq!(client.withdraw(&stream_id), 500);
    assert_eq!(token_client.balance(&recipient), 1000);
    assert_eq!(token_client.balance(&contract_id), 0);
}

#[test]
fn test_cancel_mid_stream() {
    let env = Env::default();
    let (client, token_client, sender, recipient, _) = setup(&env, 1000);

    let stream_id = client.create_stream(&sender, &recipient, &token_client.address, &1000, &100);

    advance(&env, 30);
    client.cancel(&stream_id);

    // recipient got the 30% that vested, sender got the rest back
    assert_eq!(token_client.balance(&recipient), 300);
    assert_eq!(token_client.balance(&sender), 700);
}

#[test]
fn test_partial_withdraw_then_cancel() {
    let env = Env::default();
    let (client, token_client, sender, recipient, contract_id) = setup(&env, 1000);

    let stream_id = client.create_stream(&sender, &recipient, &token_client.address, &1000, &100);

    advance(&env, 50);
    assert_eq!(client.withdraw(&stream_id), 500);

    // 80% has vested by now; 500 was already withdrawn, so 300 is paid out on cancel
    advance(&env, 30);
    client.cancel(&stream_id);

    assert_eq!(token_client.balance(&recipient), 800);
    assert_eq!(token_client.balance(&sender), 200);
    assert_eq!(token_client.balance(&contract_id), 0);
}

#[test]
fn test_balance_starts_at_zero() {
    let env = Env::default();
    let (client, token_client, sender, recipient, _) = setup(&env, 1000);

    let stream_id = client.create_stream(&sender, &recipient, &token_client.address, &1000, &100);

    assert_eq!(client.balance(&stream_id), 0);
    assert_eq!(client.withdraw(&stream_id), 0);
    assert_eq!(token_client.balance(&recipient), 0);
}

#[test]
fn test_stream_ids_increment() {
    let env = Env::default();
    let (client, token_client, sender, recipient, _) = setup(&env, 2000);

    let first = client.create_stream(&sender, &recipient, &token_client.address, &1000, &100);
    let second = client.create_stream(&sender, &recipient, &token_client.address, &1000, &100);

    assert_eq!(first, 0);
    assert_eq!(second, 1);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_rejects_zero_amount() {
    let env = Env::default();
    let (client, token_client, sender, recipient, _) = setup(&env, 1000);
    client.create_stream(&sender, &recipient, &token_client.address, &0, &100);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_rejects_negative_amount() {
    let env = Env::default();
    let (client, token_client, sender, recipient, _) = setup(&env, 1000);
    client.create_stream(&sender, &recipient, &token_client.address, &-5, &100);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_rejects_zero_duration() {
    let env = Env::default();
    let (client, token_client, sender, recipient, _) = setup(&env, 1000);
    client.create_stream(&sender, &recipient, &token_client.address, &1000, &0);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_unknown_stream() {
    let env = Env::default();
    let (client, _, _, _, _) = setup(&env, 1000);
    client.balance(&99);
}
