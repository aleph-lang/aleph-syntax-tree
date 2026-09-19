use crate::syntax::AlephTree;

/// A `blake3` digest identifying an `AlephTree` node by structural content,
/// not by name or position. See `content_hash`.
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
    blake3::Hash::from(content_hash(node)).to_hex().to_string()
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

    #[test]
    fn tree_with_extra_arg_hashes_differently() {
        let with_extra_arg = AlephTree::LetRec {
            name: "square".to_string(),
            args: vec![
                Box::new(AlephTree::Ident { value: "n".to_string() }),
                Box::new(AlephTree::Ident { value: "unused".to_string() }),
            ],
            body: Box::new(AlephTree::Mul {
                number_expr1: Box::new(AlephTree::Ident { value: "n".to_string() }),
                number_expr2: Box::new(AlephTree::Ident { value: "n".to_string() }),
            }),
        };
        assert_ne!(content_hash(&sample()), content_hash(&with_extra_arg));
    }

    #[test]
    fn with_effects_hash_is_independent_of_effect_insertion_order() {
        use crate::effects::{Effect, EffectSet};

        let a = AlephTree::WithEffects {
            inner: Box::new(sample()),
            effects: EffectSet::from([Effect::Io, Effect::Net]),
        };
        let b = AlephTree::WithEffects {
            inner: Box::new(sample()),
            effects: EffectSet::from([Effect::Net, Effect::Io]),
        };
        assert_eq!(content_hash(&a), content_hash(&b));
    }

    #[test]
    fn known_input_hashes_to_a_pinned_value() {
        // Run this once, read the actual output, and hardcode it here — this
        // is a regression guard: if this test ever needs to change, that's a
        // signal the hash algorithm or the canonical serialization changed,
        // which for a content-addressing primitive should never happen silently.
        assert_eq!(
            content_hash_hex(&sample()),
            "1c5065803a4fbd2950297d8a3aa98b16f4dfc878e96a1492d462b6fee7f3c257"
        );
    }
}
