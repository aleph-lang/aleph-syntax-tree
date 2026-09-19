use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// What a function is declared to be able to do. Attached to a subtree via
/// `AlephTree::WithEffects`. A function whose effect row is `[Pure]` cannot
/// call one whose row includes `Io`, `Net`, `Mut`, or `Act` — that check is
/// the type/effect checker's job, not this crate's; this crate only defines
/// the vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Effect {
    /// No observable side effect.
    Pure,
    /// Reads or writes outside the program (files, stdout, environment).
    Io,
    /// Network access.
    Net,
    /// Mutates shared/external state in place.
    Mut,
    /// Invokes the cognitive runtime (`Intend`/`Suggest`/`Act`/`Remember`/`Perceive`).
    Act,
}

/// A function's declared effect row. `BTreeSet`, not `Vec`: duplicates are
/// meaningless here, and — unlike `HashSet` — iteration order is fixed, so
/// two structurally-identical effect rows always serialize to identical
/// bytes (required for deterministic content hashing elsewhere in this crate).
pub type EffectSet = BTreeSet<Effect>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_and_round_trips_through_json() {
        let effects: EffectSet = EffectSet::from([Effect::Io, Effect::Act]);
        let json = serde_json::to_string(&effects).unwrap();
        let back: EffectSet = serde_json::from_str(&json).unwrap();
        assert_eq!(effects, back);
    }

    #[test]
    fn json_wire_format_is_locked() {
        // Guards against an accidental #[serde(rename_all = ...)] or
        // re-tagging change silently breaking non-Rust consumers.
        assert_eq!(serde_json::to_string(&Effect::Io).unwrap(), "\"Io\"");
        assert_eq!(serde_json::to_string(&Effect::Act).unwrap(), "\"Act\"");
    }

    #[test]
    fn duplicates_collapse_and_serialization_order_is_stable() {
        let a: EffectSet = EffectSet::from([Effect::Act, Effect::Io, Effect::Io]);
        let b: EffectSet = EffectSet::from([Effect::Io, Effect::Act]);
        assert_eq!(a, b);
        assert_eq!(serde_json::to_string(&a).unwrap(), serde_json::to_string(&b).unwrap());
    }
}
