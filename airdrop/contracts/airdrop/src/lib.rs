#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, log, token, xdr::ToXdr, Address, BytesN, Env, Vec
};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    RootHash,
    TokenAddress,
    Claimed(u32)
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyClaimed = 1,
    InvalidProof = 2,
}

#[contracttype]
#[derive(Clone, Debug)]
struct Receiver {
    pub index: u32,
    pub address: Address,
    pub amount: i128,
}

#[contract]
pub struct AirdropContract;

#[contractimpl]
impl AirdropContract {
    pub fn __constructor(
        env: Env,
        root_hash: BytesN<32>,
        token: Address,
        funding_amount: i128,
        funding_source: Address,
    ) {
        env.storage().instance().set(&DataKey::RootHash, &root_hash);
        env.storage().instance().set(&DataKey::TokenAddress, &token);
        token::TokenClient::new(&env, &token).transfer(
            &funding_source,
            &env.current_contract_address(),
            &funding_amount,
        );
    }

    fn is_claimed(env: &Env, index: u32) -> bool {
        let chunk_index = index / 128;
        let bit_index = index % 128;
        let key = DataKey::Claimed(chunk_index as u32);
        let chunk = env
            .storage()
            .persistent()
            .get::<_, u128>(&key).unwrap_or(0);

        (chunk >> bit_index) & 1 == 1
    }
    
    fn set_claimed(env: &Env, index: u32) {
        let chunk_index = index / 128;
        let bit_index = index % 128;
        let key = DataKey::Claimed(chunk_index as u32);
        let mut chunk = env
            .storage()
            .persistent()
            .get::<_, u128>(&key).unwrap_or(0);

        chunk |= 1 << bit_index;
        env.storage().persistent().set(&key, &chunk);
    }

    pub fn claim(
        env: Env,
        index: u32,
        receiver: Address,
        amount: i128,
        proof: Vec<BytesN<32>>,
    ) -> Result<(), Error> {
        if Self::is_claimed(&env, index) {
            return Err(Error::AlreadyClaimed);
        }

        let data = Receiver {
            index,
            address: receiver.clone(),
            amount,
        };

        let mut hash = env.crypto().keccak256(&data.to_xdr(&env));

        for p in proof {
            let a = hash.to_array();
            let b = p.to_array();

            let combined_array: [u8; 64] = if a < b {
                let mut arr = [0u8; 64];
                arr[..32].copy_from_slice(&a);
                arr[32..].copy_from_slice(&b);
                arr
            } else {
                let mut arr = [0u8; 64];
                arr[..32].copy_from_slice(&b);
                arr[32..].copy_from_slice(&a);
                arr
            };
            log!(&env, "hashed");
            let combined = BytesN::from_array(&env, &combined_array);
            hash = env.crypto().keccak256(&combined.into());
        }

        let root = env
            .storage()
            .instance()
            .get::<_, BytesN<32>>(&DataKey::RootHash)
            .unwrap();

        // if !root.eq(&hash.to_bytes()) {
        //     return Err(Error::InvalidProof);
        // }

        let token = env
            .storage()
            .instance()
            .get::<_, Address>(&DataKey::TokenAddress)
            .unwrap();

        token::TokenClient::new(&env, &token).transfer(
            &env.current_contract_address(),
            &receiver,
            &amount,
        );

        Self::set_claimed(&env, index);

        Ok(())
    }
}

mod test;
