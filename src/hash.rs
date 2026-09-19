use crate::syntax::AlephTree;

pub type NodeHash = [u8; 32];

/// Deterministic content hash of an `AlephTree` node. Two structurally
/// identical trees — same shape, same field values, regardless of how or
/// where they were constructed — always hash to the same value; this is
/// the identity Aleph-Next's content-addressed storage keys definitions by,
/// not their name or their position in a file.
///
/// Deliberately not stored as a field on any `AlephTree` node: hashing a
/// node that contains its own hash would be self-referential. Callers that
/// need a "this definition, with its hash" pairing should keep the hash
/// alongside the tree (e.g. `(NodeHash, AlephTree)`) rather than inside it.
pub fn content_hash(node: &AlephTree) -> NodeHash {
    let canonical = serde_json::to_vec(node).expect("AlephTree always serializes");
    *blake3::hash(&canonical).as_bytes()
}

/// Hex-encoded form of [`content_hash`] — 64 lowercase hex characters.
pub fn content_hash_hex(node: &AlephTree) -> String {
    blake3::hash(&serde_json::to_vec(node).expect("AlephTree always serializes"))
        .to_hex()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::AlephTree;

    fn sample() -> AlephTree {
        AlephTree::LetRec {
            name: "square".to_string(),
            args: vec![Box::new(AlephTree::Ident { value: "n".to_string() })],
            body: Box::new(AlephTree::Mul {
                number_expr1: Box::new(AlephTree::Ident { value: "n".to_string() }),
                number_expr2: Box::new(AlephTree::Ident { value: "n".to_string() }),
            }),
        }
    }

    #[test]
    fn same_structural_tree_hashes_identically() {
        assert_eq!(content_hash(&sample()), content_hash(&sample()));
    }

    #[test]
    fn different_tree_hashes_differently() {
        let other = match sample() {
            AlephTree::LetRec { args, body, .. } => AlephTree::LetRec {
                name: "cube".to_string(),
                args,
                body,
            },
            _ => unreachable!(),
        };
        assert_ne!(content_hash(&sample()), content_hash(&other));
    }

    #[test]
    fn hash_is_32_bytes() {
        assert_eq!(content_hash(&sample()).len(), 32);
    }

    #[test]
    fn hex_form_is_64_lowercase_hex_chars() {
        let hex = content_hash_hex(&sample());
        assert_eq!(hex.len(), 64);
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
    }
}
