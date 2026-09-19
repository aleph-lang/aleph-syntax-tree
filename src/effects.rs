use serde::{Deserialize, Serialize};

/// What a function is declared to be able to do. Attached to a subtree via
/// `AlephTree::WithEffects`. A function whose effect row is `[Pure]` cannot
/// call one whose row includes `Io`, `Net`, `Mut`, or `Act` — that check is
/// the type/effect checker's job, not this crate's; this crate only defines
/// the vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Effect {
    Pure,
    Io,
    Net,
    Mut,
    Act,
}

pub type EffectSet = Vec<Effect>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_and_round_trips_through_json() {
        let effects: EffectSet = vec![Effect::Io, Effect::Act];
        let json = serde_json::to_string(&effects).unwrap();
        let back: EffectSet = serde_json::from_str(&json).unwrap();
        assert_eq!(effects, back);
    }

    #[test]
    fn pure_is_distinct_from_every_other_effect() {
        for e in [Effect::Io, Effect::Net, Effect::Mut, Effect::Act] {
            assert_ne!(Effect::Pure, e);
        }
    }
}
