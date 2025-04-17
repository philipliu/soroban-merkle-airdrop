# soroban-merkle-airdrop

A simple airdrop contract using Merkle trees to distribute tokens to multiple receipients.

## Layout

```
.
├── airdrop — Airdrop contract
├── frontend — Claim UI
├── merkle — Merkle tree proof generation
```

## Proof Generation

### Usage

```bash
cargo run --bin merkle -- <receivers-file> <proofs-output-file>
```

#### Parameters

- `<receivers-file>`: Path to the receivers file.
- `<proofs-output-file>`: Path to the output file.

#### Input File Format

- The receivers file must be a JSON array of objects with the following
  structure:

```json
[
    {
        "address": "GDDJCEMJLXXEWNPCUSXOM5BV7CDH7AR54WYJWZYQUDPY655UGVI5ZX5Y",
        "amount": 100
    },
    {
        "address": "GBVHBZU6RHJ7KKM4XCVPDRGJV7LDCPKJQV3ZJMVBT26ZHGBDMWSGFHXP",
        "amount": 200
    }
]
```

#### Output

- The Merkle root hash is printed to stdout.
- The proofs file will contain a JSON array with receipient details and their
  corresponding proofs:

```json
[
    {
        "index": 0,
        "receiver": {
            "address": "GDDJCEMJLXXEWNPCUSXOM5BV7CDH7AR54WYJWZYQUDPY655UGVI5ZX5Y",
            "amount": 100
        },
        "proofs": [
            "523f4e7dbabc89ba00edee5769b83b1850a801240dfb3aba0b0c414e6cb56958",
            "6279358d9672a20c74ef5462989fd73d27a38a7b91d6b29bc5d6c4ed9ca5028f"
        ]
    }
]
```

## Contract Deployment

Deploy the contract using the following command to initialize the 
contract with funds transferred from the source account to the contract.

```bash
stellar contract deploy \
--wasm target/wasm32-unknown-unknown/release/airdrop.wasm \
--source-account alice \
--network testnet \
-- \
--root_hash <your-root-hash> \
--token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC
--funding_amount <funding-amount>\
--funding_source <alice address>
```

## Claim UI

Set up the environment using the proofs generated and the contract address
from the previous steps:
```
cp .env.example .env
```

Start the server:
```
npm install
npm run dev
```