use std::str::FromStr;

use merkle::MerkleTree;
use stellar_xdr::curr::{Limits, ScAddress, ScMap, ScMapEntry, ScSymbol, ScVal, StringM, WriteXdr};

fn make_receiver(address: &str, amount: u64) -> Result<ScMap, ()> {
    let entries = vec![
        ScMapEntry {
            key: ScVal::Symbol(ScSymbol(StringM::from_str("address")?)),
            val: ScVal::Address(ScAddress::from_str(address)?),
        },
        ScMapEntry {
            key: ScVal::Symbol(ScSymbol(StringM::from_str("amount")?)),
            val: ScVal::U64(amount),
        },
    ]
    .into_iter();
    let map = ScMap::sorted_from_entries(entries)?;

    Ok(map)
}

fn main() -> Result<(), ()> {
    let receivers = [
        make_receiver(
            "GDDJCEMJLXXEWNPCUSXOM5BV7CDH7AR54WYJWZYQUDPY655UGVI5ZX5Y",
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

    let tree = MerkleTree::new(serialized_receivers);

    println!("{}", hex::encode(tree.root().unwrap()));

    Ok(())
}
