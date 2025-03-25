use std::str::FromStr;

use merkle::MerkleTree;
use stellar_xdr::curr::{
    Int128Parts, Limits, ScAddress, ScMap, ScMapEntry, ScSymbol, ScVal, StringM, WriteXdr,
};

fn make_receiver(address: &str, amount: i128) -> Result<ScVal, ()> {
    let i128parts = Int128Parts {
        hi: (amount >> 64) as i64,
        lo: amount as u64,
    };
    let entries = vec![
        ScMapEntry {
            key: ScVal::Symbol(ScSymbol(StringM::from_str("address")?)),
            val: ScVal::Address(ScAddress::from_str(address)?),
        },
        ScMapEntry {
            key: ScVal::Symbol(ScSymbol(StringM::from_str("amount")?)),
            val: ScVal::I128(i128parts),
        },
    ]
    .into_iter();
    let map = ScMap::sorted_from_entries(entries)?;

    Ok(ScVal::Map(Some(map)))
}

fn main() -> Result<(), ()> {
    let receivers = [
        make_receiver(
            "CAASCQKVVBSLREPEUGPOTQZ4BC2NDBY2MW7B2LGIGFUPIY4Z3XUZRVTX",
            100,
        )?,
        make_receiver(
            "GAXEVPVZ7VHRYPKMVGTOCO6WVFBSKQHNN2J2BMOS445X362SXECXKG2W",
            100,
        )?,
        make_receiver(
            "GBDRLGGZQ7JZWVHRISYCQR22UV7ALV2KHH4RRVMWKHS7YEZJEUINQCV6",
            100,
        )?,
    ];

    let serialized_receivers: Vec<Vec<u8>> = receivers
        .iter()
        .map(|receiver| receiver.to_xdr(Limits::none()).unwrap())
        .collect();

    let tree = MerkleTree::new(serialized_receivers.clone());

    println!("root: {}", hex::encode(tree.root().unwrap()));
    println!();

    for (i, receiver) in serialized_receivers.iter().enumerate() {
        println!("Proof for receiver {}:", i);
        if let Some(proof) = tree.get_proof(receiver) {
            for chunk in &proof {
                println!("  {}", hex::encode(chunk));
            }
        } else {
            println!("  No proof generated!");
        }
    }

    Ok(())
}
