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

#[derive(Clone, Debug)]
#[contracttype]
pub struct Receiver {
    pub address: Address,
    pub amount: i128,
}

type Proof = Vec<BytesN<32>>;

#[contract]
pub struct AirdropContract;

#[contractimpl]
impl AirdropContract {
    pub fn __constructor(env: Env, root_hash: BytesN<32>, token: Address) {
        env.storage().instance().set(&DataKey::RootHash, &root_hash);
        env.storage().instance().set(&DataKey::TokenAddress, &token);
    }

    pub fn claim(env: Env, receiver: Receiver, proof: Proof) -> Result<(), Error> {
        if let Some(_) = env
            .storage()
            .instance()
            .get::<_, ()>(&DataKey::Claimed(receiver.clone().address))
        {
            return Err(Error::AlreadyClaimed);
        }

        let mut hash = env.crypto().keccak256(&receiver.clone().to_xdr(&env));
        hash = env.crypto().keccak256(&hash.into());

        for p in proof {
            let a = hash.to_array();
            let b = p.to_array();

            let combined: BytesN<64>;
            if a < b {
                let combined_vec = [a, b].concat();
                let combined_array: [u8; 64] = combined_vec.try_into().expect("Invalid length");
                combined = BytesN::from_array(&env, &combined_array);
            } else {
                let combined_vec = [a, b].concat();
                let combined_array: [u8; 64] = combined_vec.try_into().expect("Invalid length");
                combined = BytesN::from_array(&env, &combined_array);
            }
            hash = env.crypto().keccak256(&combined.into());
            hash = env.crypto().keccak256(&hash.into());
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
            &receiver.address,
            &receiver.amount,
        );

        env.storage()
            .instance()
            .set(&DataKey::Claimed(receiver.address), &());

        Ok(())
    }
}

mod test;
