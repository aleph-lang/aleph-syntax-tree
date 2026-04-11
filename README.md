# aleph-syntax-tree

Shared intermediate representation (IR) for the Aleph compiler pipeline.
All parsers produce an `AlephTree`, all generators consume one.

## Installation

```toml
[dependencies]
aleph-syntax-tree = "0.1"
```

## Key nodes

- Literals: `Int`, `Float`, `Bool`, `String`, `Bytes`, `Complex`, `HexLiteral`
- Structures: `Tuple`, `Array`, `Record`
- Control flow: `If`, `While`, `Match`, `Stmts`, `Break`, `Continue`
- Bindings: `Let`, `LetRec`
- Operations: `Add`, `Sub`, `Mul`, `Div`, `Mod`, `And`, `Or`, `Not`, `Eq`, `LE`
- Functions: `App`, `Return`
- COBOL constructs: `ProcedureDivision`, `Perform`, `Accept`, `Display`

## Related

- Parsers: [`aleparser`](https://github.com/aleph-lang/aleparser), [`js_parser`](https://github.com/aleph-lang/jsparser), [`ale_python_parser`](https://github.com/aleph-lang/pythonparser), `cobolparser`, `adaparser`, `pliparser`, `forthparser`
- Generators: [`alegen`](https://github.com/aleph-lang/alegen), [`pythongen`](https://github.com/aleph-lang/pythongen), [`erlanggen`](https://crates.io/crates/erlanggen), [`elixirgen`](https://crates.io/crates/elixirgen), [`gleamgen`](https://crates.io/crates/gleamgen)
- Compiler: [`alephc`](https://github.com/aleph-lang/aleph)
