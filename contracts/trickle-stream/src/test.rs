#![cfg(test)]
use super::*;
use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::Address;

fn create_token_contract<'a>(env: &Env, admin: &Address) -> (token::Client<'a>, token::StellarAssetClient<'a>) {
    let sac = env.register_stellar_asset_contract_v2(admin.clone());
    (
        token::Client::new(env, &sac.address()),
        token::StellarAssetClient::new(env, &sac.address()),
    )
}

#[test]
fn test_create_and_full_withdraw() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    let (token_client, token_admin) = create_token_contract(&env, &admin);
    token_admin.mint(&sender, &1000);

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let stream_id = client.create_stream(
        &sender,
        &recipient,
        &token_client.address,
        &1000,
        &100, // 100 second duration
    );

    // sender's funds moved into the contract
    assert_eq!(token_client.balance(&sender), 0);
    assert_eq!(token_client.balance(&contract_id), 1000);

    // halfway through the stream, ~half should be vested
    env.ledger().with_mut(|li| li.timestamp += 50);
    let available = client.balance(&stream_id);
    assert_eq!(available, 500);

    let withdrawn = client.withdraw(&stream_id);
    assert_eq!(withdrawn, 500);
    assert_eq!(token_client.balance(&recipient), 500);

    // fast forward past the end of the stream, remaining half should vest
    env.ledger().with_mut(|li| li.timestamp += 100);
    let withdrawn2 = client.withdraw(&stream_id);
    assert_eq!(withdrawn2, 500);
    assert_eq!(token_client.balance(&recipient), 1000);
}

#[test]
fn test_cancel_mid_stream() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    let (token_client, token_admin) = create_token_contract(&env, &admin);
    token_admin.mint(&sender, &1000);

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let stream_id = client.create_stream(
        &sender,
        &recipient,
        &token_client.address,
        &1000,
        &100,
    );

    env.ledger().with_mut(|li| li.timestamp += 30);
    client.cancel(&stream_id);

    // recipient got the ~30% vested, sender got the rest back
    assert_eq!(token_client.balance(&recipient), 300);
    assert_eq!(token_client.balance(&sender), 700);
}