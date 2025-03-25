# soroban-merkle-airdrop

## Proof Generation

### Usage

```bash
cd soroban-merkle-airdrop/merkle
cargo run -- <receivers-file> <proofs-output-file>
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
