use serde::{Deserialize, Serialize};

/// Static type attached to a subtree via `AlephTree::Typed`, or to a
/// `TypeDef`'s variant fields. Deliberately small: enough to type-check
/// records, sum types, and function signatures (see the Aleph-Next spec)
/// without committing yet to full parametric polymorphism.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Bytes,
    Unit,
    List { elem: Box<Type> },
    Tuple { elems: Vec<Type> },
    Record { fields: Vec<(String, Type)> },
    Sum { variants: Vec<(String, Vec<Type>)> },
    Fun { params: Vec<Type>, ret: Box<Type> },
    Var { name: String },
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
                ("Circle".to_string(), vec![Type::Float]),
                ("Rect".to_string(), vec![Type::Float, Type::Float]),
            ],
        };
        let json = serde_json::to_string(&ty).unwrap();
        let back: Type = serde_json::from_str(&json).unwrap();
        assert_eq!(ty, back);
    }
}
