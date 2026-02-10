![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![crates.io](https://img.shields.io/crates/v/duramen)](https://crates.io/crates/duramen)
[![documentation](https://docs.rs/duramen/badge.svg)](https://docs.rs/duramen)

![rust: 1.88+](https://img.shields.io/badge/rust-1.88+-orange.svg)
![`unsafe`: forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)
![`wasm`: compatible](https://img.shields.io/badge/wasm-compatible-success.svg)
![`no-std`: compatible](https://img.shields.io/badge/no--std-compatible-success.svg)

[![codecov](https://codecov.io/gh/DuskSystems/duramen/graph/badge.svg)](https://codecov.io/gh/DuskSystems/duramen)
[![codspeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/DuskSystems/duramen)

# `duramen`

A Cedar implementation.

> [!WARNING]
> Not ready for use.

## Architecture

```mermaid
flowchart TD
    Source -->|lexing| Tokens
    Tokens -->|parsing| CST
    CST -->|lowering| AST
    CST -.-> Diagnostics([Diagnostics])
    AST -.-> Diagnostics
    AST --> Validator
    Validator -.-> Diagnostics
    Validator --> Evaluator
    Evaluator -.-> Diagnostics
    Evaluator -.-> Decision([Decision])
```

## Disclaimer

Duramen is not an official Cedar project.

Cedar is a trademark of Amazon Web Services, and a member of the Cloud Native Computing Foundation.

## License

`duramen` is licensed under the terms of both the [MIT License](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).
