#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, token, xdr::ToXdr, Address, BytesN, Env,
    Vec,
};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    RootHash,
    TokenAddress,
    Claimed(Address),
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
    pub address: Address,
    pub amount: i128,
}

#[contract]
pub struct AirdropContract;

#[contractimpl]
impl AirdropContract {
    pub fn __constructor(env: Env, root_hash: BytesN<32>, token: Address) {
        env.storage().instance().set(&DataKey::RootHash, &root_hash);
        env.storage().instance().set(&DataKey::TokenAddress, &token);
    }

    pub fn claim(
        env: Env,
        receiver: Address,
        amount: i128,
        proof: Vec<BytesN<32>>,
    ) -> Result<(), Error> {
        if env
            .storage()
            .instance()
            .get::<_, ()>(&DataKey::Claimed(receiver.clone()))
            .is_some()
        {
            return Err(Error::AlreadyClaimed);
        }

        let data = Receiver {
            address: receiver.clone(),
            amount,
        };

        let mut hash = env.crypto().sha256(&data.to_xdr(&env));

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
            let combined = BytesN::from_array(&env, &combined_array);
            hash = env.crypto().sha256(&combined.into());
        }

        let root = env
            .storage()
            .instance()
            .get::<_, BytesN<32>>(&DataKey::RootHash)
            .unwrap();

        if !root.eq(&hash.to_bytes()) {
            return Err(Error::InvalidProof);
        }

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

        env.storage()
            .instance()
            .set(&DataKey::Claimed(receiver), &());

        Ok(())
    }
}

mod test;
