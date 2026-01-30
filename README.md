![license: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)

![rust: 1.93+](https://img.shields.io/badge/rust-1.93+-orange.svg)
![no-std: compatible](https://img.shields.io/badge/no--std-compatible-success.svg)
![wasm: compatible](https://img.shields.io/badge/wasm-compatible-success.svg)
![unsafe: forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)

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
    CST --> Diagnostics([Diagnostics])
    AST <-->|serialization| EST
    EST <--> JSON
    EST <--> Protobuf
    AST --> Validator
    Validator --> Diagnostics
```

## Disclaimer

Duramen is not an official Cedar project.

Cedar is a trademark of Amazon Web Services, and a member of the Cloud Native Computing Foundation.
