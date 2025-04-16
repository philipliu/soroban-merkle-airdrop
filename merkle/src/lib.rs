use std::collections::HashMap;

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerkleTree {
    layers: Vec<Vec<[u8; 32]>>,
    leaf_indicies: HashMap<[u8; 32], usize>,
}

impl MerkleTree {
    pub fn new<T: AsRef<[u8]>, I: IntoIterator<Item = T>>(data: I) -> Self {
        let mut digests = data
            .into_iter()
            .map(|d| sha256(d.as_ref()))
            .collect::<Vec<_>>();

        digests.sort();

        let mut leaf_indicies = HashMap::new();
        for (i, leaf) in digests.iter().enumerate() {
            leaf_indicies.insert(*leaf, i);
        }

        let mut layers = vec![digests];
        let mut level: usize = 0;

        while layers[level].len() > 1 {
            let current_layer = &layers[level];
            let mut next_layer = Vec::new();

            for i in (0..current_layer.len()).step_by(2) {
                if i + 1 < current_layer.len() {
                    let a = current_layer[i];
                    let b = current_layer[i + 1];
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

    pub fn get_proof<T: AsRef<[u8]>>(&self, data: T) -> Option<Vec<[u8; 32]>> {
        let hash = sha256(data.as_ref());

        self.leaf_indicies.get(&hash)?;
        let mut idx = *self.leaf_indicies.get(&hash).unwrap();

        let mut proof = Vec::new();
        for l in 0..self.layers.len() - 1 {
            let current_layer = &self.layers[l];

            let sibling_idx = if idx % 2 == 0 {
                if idx + 1 < current_layer.len() {
                    idx + 1
                } else {
                    idx
                }
            } else {
                idx - 1
            };

            if sibling_idx != idx {
                proof.push(current_layer[sibling_idx])
            }

            idx /= 2;
        }

        Some(proof)
    }
}

fn merge(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let mut digest = Sha256::new();
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

fn sha256<T: AsRef<[u8]>>(data: T) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(data);
    let result = digest.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);

    hash
}

pub fn verify<T: AsRef<[u8]>>(root: &[u8; 32], data: T, proof: &[[u8; 32]]) -> bool {
    let mut hash = sha256(data.as_ref());

    for p in proof {
        hash = merge(&hash, p);
    }

    hash.eq(root)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_construction() {
        let a = b"address 1";
        let b = b"address 2";
        let c = b"address 3";

        let tree = MerkleTree::new([a, b, c]);
        let proof_a = tree.get_proof(a).unwrap();
        let proof_b = tree.get_proof(b).unwrap();
        let proof_c = tree.get_proof(c).unwrap();

        assert!(tree.root().is_some());
        assert!(!proof_a.is_empty());
        assert!(!proof_b.is_empty());
        assert!(!proof_c.is_empty());
    }

    #[test]
    fn test_get_proof_inclusion() {
        let a = b"address 1";
        let b = b"address 2";

        let tree = MerkleTree::new([a, b]);
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

        let tree = MerkleTree::new([a, b]);
        let proof_c = tree.get_proof(c);

        assert!(proof_c.is_none());
    }

    #[test]
    fn test_verify_bad_proof() {
        let a = b"address 1";
        let b = b"address 2";
        let c = b"address 3";
        let tree_1 = MerkleTree::new([a, b, c]);

        let d = b"address 4";
        let e = b"address 5";
        let f = b"address 6";
        let tree_2 = MerkleTree::new([d, e, f]);

        let tree_1_root = tree_1.root().unwrap();

        assert!(!verify(&tree_1_root, d, &tree_2.get_proof(d).unwrap()));
        assert!(!verify(&tree_1_root, d, &tree_2.get_proof(e).unwrap()));
        assert!(!verify(&tree_1_root, d, &tree_2.get_proof(f).unwrap()));
    }
}
