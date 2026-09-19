use serde::{Deserialize, Serialize};

/// Static type attached to a subtree via `AlephTree::Typed`, or to a
/// `TypeDef`'s variant fields. Deliberately small: enough to type-check
/// records, sum types, and function signatures (see the Aleph-Next spec)
/// without committing yet to full parametric polymorphism.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Type {
    /// 64-bit signed integer.
    Int,
    /// 64-bit floating point number.
    Float,
    /// Boolean.
    Bool,
    /// UTF-8 text.
    String,
    /// Raw byte sequence.
    Bytes,
    /// The single-valued "nothing" type.
    // `#[default]` here exists only so `AlephTree`'s `strum::EnumString`
    // derive can build a placeholder `Type` value for its `FromStr` path
    // (which requires every struct-like variant's fields to implement
    // `Default`). It is not a sentinel for "no type annotation" — use
    // `Option<Type>` for that. Don't let `Type::default()` leak into
    // real type-checking logic.
    #[default]
    Unit,
    /// Homogeneous list of `elem`.
    List { elem: Box<Type> },
    /// Fixed-size heterogeneous tuple.
    Tuple { elems: Vec<Type> },
    /// Named-field record (struct).
    Record { fields: Vec<RecordField> },
    /// Tagged union of named variants (sum/enum type).
    Sum { variants: Vec<Variant> },
    /// Function signature: positional parameter types and a return type.
    Fun { params: Vec<Type>, ret: Box<Type> },
    /// A type variable, referenced by name (for as-yet-unresolved/generic types).
    Var { name: String },
}

/// One named field of a `Type::Record`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordField {
    pub name: String,
    pub ty: Type,
}

/// One named variant of a `Type::Sum` (e.g. `Circle` in `Circle(radius: Float)`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Variant {
    pub name: String,
    pub fields: Vec<Type>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_round_trips_through_json() {
        let ty = Type::Int;
        let json = serde_json::to_string(&ty).unwrap();
        let back: Type = serde_json::from_str(&json).unwrap();
        assert_eq!(ty, back);
    }

    #[test]
    fn function_type_round_trips_through_json() {
        let ty = Type::Fun {
            params: vec![Type::Int, Type::String],
            ret: Box::new(Type::Bool),
        };
        let json = serde_json::to_string(&ty).unwrap();
        let back: Type = serde_json::from_str(&json).unwrap();
        assert_eq!(ty, back);
    }

    #[test]
    fn sum_type_round_trips_through_json() {
        let ty = Type::Sum {
            variants: vec![
                Variant { name: "Circle".to_string(), fields: vec![Type::Float] },
                Variant { name: "Rect".to_string(), fields: vec![Type::Float, Type::Float] },
            ],
        };
        let json = serde_json::to_string(&ty).unwrap();
        let back: Type = serde_json::from_str(&json).unwrap();
        assert_eq!(ty, back);
    }

    #[test]
    fn struct_like_variant_wire_format_is_locked() {
        let ty = Type::List { elem: Box::new(Type::Int) };
        assert_eq!(
            serde_json::to_string(&ty).unwrap(),
            r#"{"type":"List","elem":{"type":"Int"}}"#
        );
    }

    #[test]
    fn record_wire_format_is_locked() {
        let ty = Type::Record {
            fields: vec![RecordField { name: "x".to_string(), ty: Type::Int }],
        };
        assert_eq!(
            serde_json::to_string(&ty).unwrap(),
            r#"{"type":"Record","fields":[{"name":"x","ty":{"type":"Int"}}]}"#
        );
    }

    #[test]
    fn var_round_trips_through_json() {
        let ty = Type::Var { name: "a".to_string() };
        let json = serde_json::to_string(&ty).unwrap();
        let back: Type = serde_json::from_str(&json).unwrap();
        assert_eq!(ty, back);
    }

    #[test]
    fn nested_composite_type_round_trips_through_json() {
        let ty = Type::List {
            elem: Box::new(Type::Record {
                fields: vec![RecordField { name: "x".to_string(), ty: Type::Float }],
            }),
        };
        let json = serde_json::to_string(&ty).unwrap();
        let back: Type = serde_json::from_str(&json).unwrap();
        assert_eq!(ty, back);
    }
}
