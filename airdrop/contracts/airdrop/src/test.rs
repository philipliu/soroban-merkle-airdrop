#![cfg(test)]

extern crate std;

use super::*;
use soroban_sdk::token::TokenClient;
use soroban_sdk::Address;
use soroban_sdk::{testutils::Address as _, vec, Env};
use std::println;
use token::StellarAssetClient as TokenAdminClient;

fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let sac = e.register_stellar_asset_contract_v2(admin.clone());
    (
        token::Client::new(e, &sac.address()),
        token::StellarAssetClient::new(e, &sac.address()),
    )
}

fn make_args(
    env: &Env,
    hash: &str,
    token: Address,
    funding_amount: i128,
    funding_source: Address,
) -> (BytesN<32>, Address, i128, Address) {
    let mut hash_bytes = [0u8; 32];
    hex::decode_to_slice(hash, &mut hash_bytes).unwrap();

    let root_hash = BytesN::from_array(env, &hash_bytes);

    (root_hash, token, funding_amount, funding_source)
}

fn hex_to_bytes(env: &Env, hex_str: &str) -> BytesN<32> {
    let hash_bytes = hex::decode(hex_str).unwrap().try_into().unwrap();
    BytesN::from_array(env, &hash_bytes)
}

#[test]
fn test_valid_claim() {
    let env = Env::default();
    env.mock_all_auths_allowing_non_root_auth();
    env.cost_estimate().budget().reset_default();

    let token_admin = Address::generate(&env);
    let (token, token_admin_client) = create_token_contract(&env, &token_admin);
    token_admin_client.mint(&token_admin_client.address, &1000);

    let constructor_args = make_args(
        &env,
        "11932105f1a4d0092e87cead3a543da5afd8adcff63f9a8ceb6c5db3c8135722",
        token.address.clone(),
        1000,
        token_admin_client.address.clone(),
    );

    let contract_id = env.register(AirdropContract {}, constructor_args);
    let client = AirdropContractClient::new(&env, &contract_id);

    let receiver = Address::from_str(
        &env,
        "CAASCQKVVBSLREPEUGPOTQZ4BC2NDBY2MW7B2LGIGFUPIY4Z3XUZRVTX",
    );
    let amount = 100;
    let proofs = vec![
        &env,
        hex_to_bytes(
            &env,
            "fc0d9c2f46c1e910bd3af8665318714c7c97486d2a206f96236c6e7e50c080d7",
        ),
        hex_to_bytes(
            &env,
            "c83f7b26055572e5e84c78ec4d4f45b85b71698951077baafe195279c1f30be4",
        ),
    ];

    client.claim(&3_u32, &receiver, &amount, &proofs);
    println!("{}", env.cost_estimate().budget());

    assert_eq!(token.balance(&receiver), 100);
    assert_eq!(token.balance(&contract_id), 900);
    assert!(env.auths().is_empty());
}

#[test]
#[should_panic]
fn test_double_claim() {
    let env: Env = Env::default();
    env.mock_all_auths_allowing_non_root_auth();

    let token_admin = Address::generate(&env);
    let (token, token_admin_client) = create_token_contract(&env, &token_admin);
    token_admin_client.mint(&token_admin_client.address, &1000);

    let args = make_args(
        &env,
        "11932105f1a4d0092e87cead3a543da5afd8adcff63f9a8ceb6c5db3c8135722",
        token.address.clone(),
        1000,
        token_admin_client.address.clone(),
    );
    let contract_id = env.register(AirdropContract {}, args);
    let client = AirdropContractClient::new(&env, &contract_id);

    let receiver = Address::from_str(
        &env,
        "CAASCQKVVBSLREPEUGPOTQZ4BC2NDBY2MW7B2LGIGFUPIY4Z3XUZRVTX",
    );
    let amount: i128 = 100;
    let proofs = vec![
        &env,
        hex_to_bytes(
            &env,
            "fc0d9c2f46c1e910bd3af8665318714c7c97486d2a206f96236c6e7e50c080d7",
        ),
        hex_to_bytes(
            &env,
            "c83f7b26055572e5e84c78ec4d4f45b85b71698951077baafe195279c1f30be4",
        ),
    ];

    client.claim(&3_u32, &receiver, &amount, &proofs);
    client.claim(&3_u32, &receiver, &amount, &proofs);
}

#[test]
#[should_panic]
fn test_bad_claim() {
    let env = Env::default();
    env.mock_all_auths_allowing_non_root_auth();

    let token_admin = Address::generate(&env);
    let (token, token_admin_client) = create_token_contract(&env, &token_admin);
    token_admin_client.mint(&token_admin_client.address, &1000);

    let args = make_args(
        &env,
        "11932105f1a4d0092e87cead3a543da5afd8adcff63f9a8ceb6c5db3c8135722",
        token.address.clone(),
        1000,
        token_admin_client.address.clone(),
    );
    let contract_id = env.register(AirdropContract {}, args);
    let client = AirdropContractClient::new(&env, &contract_id);

    let receiver = Address::from_str(
        &env,
        "CAASCQKVVBSLREPEUGPOTQZ4BC2NDBY2MW7B2LGIGFUPIY4Z3XUZRVTX",
    );
    let amount = 100000; // This is a different amount
    let proofs = vec![
        &env,
        hex_to_bytes(
            &env,
            "fc0d9c2f46c1e910bd3af8665318714c7c97486d2a206f96236c6e7e50c080d7",
        ),
        hex_to_bytes(
            &env,
            "c83f7b26055572e5e84c78ec4d4f45b85b71698951077baafe195279c1f30be4",
        ),
    ];

    client.claim(&3_u32, &receiver, &amount, &proofs);
}

#[test]
fn test_large_claim() {
    let env: Env = Env::default();
    env.mock_all_auths_allowing_non_root_auth();

    let token_admin = Address::generate(&env);
    let (token, token_admin_client) = create_token_contract(&env, &token_admin);
    token_admin_client.mint(&token_admin_client.address, &1000);

    let args = make_args(
        &env,
        "11932105f1a4d0092e87cead3a543da5afd8adcff63f9a8ceb6c5db3c8135722",
        token.address.clone(),
        1000,
        token_admin_client.address.clone(),
    );
    let contract_id = env.register(AirdropContract {}, args);
    let client = AirdropContractClient::new(&env, &contract_id);

    let receiver = Address::from_str(
        &env,
        "CAASCQKVVBSLREPEUGPOTQZ4BC2NDBY2MW7B2LGIGFUPIY4Z3XUZRVTX",
    );
    let amount: i128 = 100;
    let proofs = vec![
        &env,
        hex_to_bytes(
            &env,
            "fc0d9c2f46c1e910bd3af8665318714c7c97486d2a206f96236c6e7e50c080d7",
        ),
        hex_to_bytes(
            &env,
            "c83f7b26055572e5e84c78ec4d4f45b85b71698951077baafe195279c1f30be4",
        ),
        hex_to_bytes(
            &env,
            "fc0d9c2f46c1e910bd3af8665318714c7c97486d2a206f96236c6e7e50c080d7",
        ),
        hex_to_bytes(
            &env,
            "c83f7b26055572e5e84c78ec4d4f45b85b71698951077baafe195279c1f30be4",
        ),
        hex_to_bytes(
            &env,
            "fc0d9c2f46c1e910bd3af8665318714c7c97486d2a206f96236c6e7e50c080d7",
        ),
        hex_to_bytes(
            &env,
            "c83f7b26055572e5e84c78ec4d4f45b85b71698951077baafe195279c1f30be4",
        ),
        hex_to_bytes(
            &env,
            "fc0d9c2f46c1e910bd3af8665318714c7c97486d2a206f96236c6e7e50c080d7",
        ),
        hex_to_bytes(
            &env,
            "c83f7b26055572e5e84c78ec4d4f45b85b71698951077baafe195279c1f30be4",
        ),
        hex_to_bytes(
            &env,
            "fc0d9c2f46c1e910bd3af8665318714c7c97486d2a206f96236c6e7e50c080d7",
        ),
        hex_to_bytes(
            &env,
            "c83f7b26055572e5e84c78ec4d4f45b85b71698951077baafe195279c1f30be4",
        ),
        hex_to_bytes(
            &env,
            "fc0d9c2f46c1e910bd3af8665318714c7c97486d2a206f96236c6e7e50c080d7",
        ),
        hex_to_bytes(
            &env,
            "c83f7b26055572e5e84c78ec4d4f45b85b71698951077baafe195279c1f30be4",
        ),
        hex_to_bytes(
            &env,
            "fc0d9c2f46c1e910bd3af8665318714c7c97486d2a206f96236c6e7e50c080d7",
        ),
        hex_to_bytes(
            &env,
            "c83f7b26055572e5e84c78ec4d4f45b85b71698951077baafe195279c1f30be4",
        ),
        hex_to_bytes(
            &env,
            "fc0d9c2f46c1e910bd3af8665318714c7c97486d2a206f96236c6e7e50c080d7",
        ),
        hex_to_bytes(
            &env,
            "c83f7b26055572e5e84c78ec4d4f45b85b71698951077baafe195279c1f30be4",
        ),
        hex_to_bytes(
            &env,
            "fc0d9c2f46c1e910bd3af8665318714c7c97486d2a206f96236c6e7e50c080d7",
        ),
        hex_to_bytes(
            &env,
            "c83f7b26055572e5e84c78ec4d4f45b85b71698951077baafe195279c1f30be4",
        ),
        hex_to_bytes(
            &env,
            "fc0d9c2f46c1e910bd3af8665318714c7c97486d2a206f96236c6e7e50c080d7",
        ),
        hex_to_bytes(
            &env,
            "c83f7b26055572e5e84c78ec4d4f45b85b71698951077baafe195279c1f30be4",
        ),
    ];

    let _ = client.try_claim(&3_u32, &receiver, &amount, &proofs);
    println!("{}", env.cost_estimate().budget());
}
