#![cfg(test)]

use super::*;
use soroban_sdk::token::TokenClient;
use soroban_sdk::Address;
use soroban_sdk::{testutils::Address as _, vec, Env};
use token::StellarAssetClient as TokenAdminClient;

fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let sac = e.register_stellar_asset_contract_v2(admin.clone());
    (
        token::Client::new(e, &sac.address()),
        token::StellarAssetClient::new(e, &sac.address()),
    )
}

fn make_args(env: &Env, hash: &str, token: Address) -> (BytesN<32>, Address) {
    let mut hash_bytes = [0u8; 32];
    hex::decode_to_slice(hash, &mut hash_bytes).unwrap();

    let root_hash = BytesN::from_array(env, &hash_bytes);

    (root_hash, token)
}

#[test]
fn test_valid_claim() {
    let env = Env::default();
    env.mock_all_auths();

    let token_admin = Address::generate(&env);
    let (token, token_admin_client) = create_token_contract(&env, &token_admin);

    let args = make_args(
        &env,
        "8943b9ea17c82021714e46d047234e52db5fa43f25a427fbb80831f1a384c340",
        token.address.clone(),
    );
    let contract_id = env.register(AirdropContract {}, args);
    let client = AirdropContractClient::new(&env, &contract_id);

    token_admin_client.mint(&contract_id, &1000);

    let receiver = Address::from_str(
        &env,
        "CAASCQKVVBSLREPEUGPOTQZ4BC2NDBY2MW7B2LGIGFUPIY4Z3XUZRVTX",
    );
    let amount = 100;
    let proofs = vec![
        &env,
        BytesN::from_array(
            &env,
            &hex::decode("a5a8655f4b3f68e556a2e7edcf8fd44863ab22bad99cfc6b14d8bdff943e7833")
                .unwrap()
                .try_into()
                .unwrap(),
        ),
    ];

    client.claim(&receiver, &amount, &proofs);
    assert_eq!(token.balance(&receiver), 100);
    assert_eq!(token.balance(&contract_id), 900);
    assert!(env.auths().is_empty());
}

#[test]
#[should_panic]
fn test_double_claim() {
    let env = Env::default();
    env.mock_all_auths();

    let token_admin = Address::generate(&env);
    let (token, token_admin_client) = create_token_contract(&env, &token_admin);

    let args = make_args(
        &env,
        "8943b9ea17c82021714e46d047234e52db5fa43f25a427fbb80831f1a384c340",
        token.address.clone(),
    );
    let contract_id = env.register(AirdropContract {}, args);
    let client = AirdropContractClient::new(&env, &contract_id);

    token_admin_client.mint(&contract_id, &1000);

    let receiver = Address::from_str(
        &env,
        "CAASCQKVVBSLREPEUGPOTQZ4BC2NDBY2MW7B2LGIGFUPIY4Z3XUZRVTX",
    );
    let amount: i128 = 100;
    let proofs = vec![
        &env,
        BytesN::from_array(
            &env,
            &hex::decode("a5a8655f4b3f68e556a2e7edcf8fd44863ab22bad99cfc6b14d8bdff943e7833")
                .unwrap()
                .try_into()
                .unwrap(),
        ),
    ];

    client.claim(&receiver, &amount, &proofs);
    client.claim(&receiver, &amount, &proofs);
}

#[test]
#[should_panic]
fn test_bad_claim() {
    let env = Env::default();
    env.mock_all_auths();

    let token_admin = Address::generate(&env);
    let (token, token_admin_client) = create_token_contract(&env, &token_admin);

    let args = make_args(
        &env,
        "8943b9ea17c82021714e46d047234e52db5fa43f25a427fbb80831f1a384c340",
        token.address.clone(),
    );
    let contract_id = env.register(AirdropContract {}, args);
    let client = AirdropContractClient::new(&env, &contract_id);

    token_admin_client.mint(&contract_id, &1000);

    let receiver = Address::from_str(
        &env,
        "CAASCQKVVBSLREPEUGPOTQZ4BC2NDBY2MW7B2LGIGFUPIY4Z3XUZRVTX",
    );
    let amount = 100000; // This is a different amount
    let proofs = vec![
        &env,
        BytesN::from_array(
            &env,
            &hex::decode("a5a8655f4b3f68e556a2e7edcf8fd44863ab22bad99cfc6b14d8bdff943e7833")
                .unwrap()
                .try_into()
                .unwrap(),
        ),
    ];

    client.claim(&receiver, &amount, &proofs);
}
