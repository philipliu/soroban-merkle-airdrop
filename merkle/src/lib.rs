#![allow(dead_code)]

use sha3::{Digest, Keccak256};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct MerkleTree {
    layers: Vec<Vec<[u8; 32]>>,
    leaf_indicies: HashMap<[u8; 32], usize>,
}

impl MerkleTree {
    pub fn new(data: &[&[u8]]) -> Self {
        let mut digest = data
            .iter()
            .map(|address| {
                let mut hasher = Keccak256::new();
                hasher.update(address);
                let result = hasher.finalize();
                let mut hash = [0u8; 32];
                hash.copy_from_slice(&result);
                hash
            })
            .collect::<Vec<_>>();

        digest.sort();

        let mut leaf_indicies = HashMap::new();
        for (i, leaf) in digest.iter().enumerate() {
            leaf_indicies.insert(*leaf, i);
        }

        let mut layers = vec![digest];
        let mut level: usize = 0;

        while layers[level].len() > 1 {
            let current_layer = &layers[level];
            let mut next_layer = Vec::new();

            for i in (0..current_layer.len()).step_by(2) {
                if i + 1 < current_layer.len() {
                    let a = current_layer[0];
                    let b = current_layer[1];
                    let parent = merge(&a, &b);
                    next_layer.push(parent);
                } else {
                    next_layer.push(current_layer[i]);
                }
            }

            layers.push(next_layer);
            level += 1;
        }

        Self {
            layers,
            leaf_indicies,
        }
    }

    pub fn root(&self) -> Option<[u8; 32]> {
        if self.layers.is_empty() || self.layers.last()?.is_empty() {
            return None;
        }
        self.layers.last().map(|l| l[0])
    }

    pub fn get_proof(&self, data: &[u8]) -> Option<Vec<[u8; 32]>> {
        let mut digest = Keccak256::new();
        digest.update(data);
        let result = digest.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);

        if self.leaf_indicies.get(&hash).is_none() {
            return None;
        };
        let idx = *self.leaf_indicies.get(&hash).unwrap();

        let mut proof = Vec::new();
        for l in 0..self.layers.len() - 1 {
            let current_layer = &self.layers[l];

            let sibling_idx = if idx % 2 == 0 {
                if idx + 1 < current_layer.len() { idx + 1 } else { idx }
            } else {
                idx - 1
            };

            if sibling_idx != idx {
                proof.push(current_layer[sibling_idx])
            }
        }

        Some(proof)
    }
}

fn merge(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let mut digest = Keccak256::new();
    if a < b {
        digest.update(a);
        digest.update(b);
    } else {
        digest.update(b);
        digest.update(a);
    }
    let result = digest.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);

    hash
}

pub fn verify(root: &[u8; 32], data: &[u8], proof: &[[u8; 32]]) -> bool {
    let mut digest = Keccak256::new();
    digest.update(data);
    let result = digest.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    
    for p in proof {
        hash = merge(&hash, p);
    }

    return hash.eq(root);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_construction() {
        let a = b"address 1";
        let b = b"address 2";

        let tree = MerkleTree::new(&[a, b]);

        assert!(tree.root().is_some());
    }

    #[test]
    fn test_get_proof_inclusion() {
        let a = b"address 1";
        let b = b"address 2";

        let tree = MerkleTree::new(&[a, b]);
        let proof_a = tree.get_proof(a).unwrap();
        let proof_b = tree.get_proof(a).unwrap();

        assert!(proof_a.len() == 1);
        assert!(verify(&tree.root().unwrap(), a, &proof_a));

        assert!(proof_b.len() == 1);
        assert!(verify(&tree.root().unwrap(), a, &proof_b));
 
    }

    #[test]
    fn test_get_proof_exclusion() {
        let a = b"address 1";
        let b = b"address 2";
        let c = b"address 3";

        let tree = MerkleTree::new(&[a, b]);
        let proof_c = tree.get_proof(c);

        assert!(proof_c.is_none());
    }

    #[test]
    fn test_verify_bad_proof() {
        let a = b"address 1";
        let b = b"address 2";
        let c = b"address 3";
        let tree_1 = MerkleTree::new(&[a, b, c]);

        let d = b"address 4";
        let e = b"address 5";
        let f = b"address 6";
        let tree_2 = MerkleTree::new(&[d, e, f]);

        let tree_1_root = tree_1.root().unwrap();

        assert!(!verify(&tree_1_root, d, &tree_2.get_proof(d).unwrap()));
        assert!(!verify(&tree_1_root, d, &tree_2.get_proof(e).unwrap()));
        assert!(!verify(&tree_1_root, d, &tree_2.get_proof(f).unwrap()));
    }
}
