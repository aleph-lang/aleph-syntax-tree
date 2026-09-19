# aleph-syntax-tree

Shared intermediate representation (IR) for the Aleph compiler pipeline.
All parsers produce an `AlephTree`, all generators consume one.

## Installation

```toml
[dependencies]
aleph-syntax-tree = "0.2"
```

## Key nodes

- Literals: `Int`, `Float`, `Bool`, `String`, `Bytes`, `Complex`, `HexLiteral`
- Structures: `Tuple`, `Array`, `Record`
- Control flow: `If`, `While`, `Match`, `Stmts`, `Break`, `Continue`
- Bindings: `Let`, `LetRec`
- Operations: `Add`, `Sub`, `Mul`, `Div`, `Mod`, `And`, `Or`, `Not`, `Eq`, `LE`
- Functions: `App`, `Return`
- COBOL constructs: `ProcedureDivision`, `Perform`, `Accept`, `Display`
- Type & effect layer (v0.2, Aleph-Next): `Typed`, `TypeDef`, `WithEffects` — see `types::Type`, `effects::Effect`

## Type, effect and content-hash layer (v0.2+)

Additive foundation for the planned Aleph-Next successor language — existing parsers/generators pinned to `"0.1"` are unaffected.

- `types::Type` — a small static type system (primitives, `List`, `Tuple`, `Record`, `Sum`, `Fun`, `Var`) attached to a subtree via `AlephTree::Typed`, or declared with `AlephTree::TypeDef`.
- `effects::Effect` — a function's declared effect row (`Pure`, `Io`, `Net`, `Mut`, `Act`), attached via `AlephTree::WithEffects`.
- `hash::content_hash` / `content_hash_hex` — deterministic `blake3` structural identity for any `AlephTree` node, independent of variable names or position in a file. The basis for future content-addressed storage; not itself a storage layer.

## Related

- Parsers: [`aleparser`](https://github.com/aleph-lang/aleparser), [`js_parser`](https://github.com/aleph-lang/jsparser), [`ale_python_parser`](https://github.com/aleph-lang/pythonparser), `cobolparser`, `adaparser`, `pliparser`, `forthparser`
- Generators: [`alegen`](https://github.com/aleph-lang/alegen), [`pythongen`](https://github.com/aleph-lang/pythongen), [`erlanggen`](https://crates.io/crates/erlanggen), [`elixirgen`](https://crates.io/crates/elixirgen), [`gleamgen`](https://crates.io/crates/gleamgen)
- Compiler: [`alephc`](https://github.com/aleph-lang/aleph)
