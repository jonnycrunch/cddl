# cddl-rs

[![crates.io](https://img.shields.io/crates/v/cddl.svg)](https://crates.io/crates/cddl) [![docs.rs](https://docs.rs/cddl/badge.svg)](https://docs.rs/cddl) [![Build Status](https://dev.azure.com/anweiss/anweiss/_apis/build/status/cddl-CI?branchName=master)](https://dev.azure.com/anweiss/anweiss/_build/latest?definitionId=1&branchName=master) [![codecov](https://codecov.io/gh/anweiss/cddl/branch/master/graph/badge.svg)](https://codecov.io/gh/anweiss/cddl)

> This library is very much experimental and is being developed as a personal learning exercise for getting acquainted with Rust and about parsing in general. It is far from complete. There are likely more performant and stable libraries out there for parsing CDDL. This one should not be used in production in any form or fashion.

A Rust implementation of the Concise data definition language (CDDL). CDDL is an IETF standard that "proposes a notational convention to express CBOR and JSON data structures." As of 2019-06-12, it is published as RFC 8610 (Proposed Standard) at https://tools.ietf.org/html/rfc8610.

This library includes a handwritten parser and lexer for CDDL and is heavily inspired by Thorsten Ball's book ["Writing An Interpretor In Go"](https://interpreterbook.com/). The AST has been built to closely match the rules defined by the ABNF grammar in [Appendix B.](https://tools.ietf.org/html/rfc8610#appendix-B) of the spec.

I'm currently only focused on using CDDL as a means for validating JSON data, and as such, work is being done to build a JSON validation component into the library and to distribute a CLI binary. An extremely basic REPL is included as well, with plans to compile for use in the browser with WebAssembly. Furthermore, there are a number of improvements to error handling and validation that need to be made.

## Goals

- Parse CDDL documents into an AST
- Verify conformance of CDDL documents against RFC 8610
- Validate JSON documents using CDDL (only interested in the JSON subset of the CBOR generic data model for now)
- Basic REPL
- Generate dummy JSON from conformant CDDL
- Close to zero-copy as possible
- Compile for use in the browser with WebAssembly
- `no_std` support (lexing and parsing only)

## Non-goals

- Validate CBOR data structures (might eventually support this given it's one of the primary goals for CDDL, but not focused on this at the moment)
- Performance (if this library gains enough traction, it may be prudent to explore using a parser-combinator framework like [nom](https://github.com/Geal/nom))
- Support CBOR diagnostic notation
- I-JSON compatibility

## Why Rust?

Rust is a systems programming language designed around safety and is ideally-suited for resource-constrained systems. CDDL and CBOR are designed around small code and message sizes and constrained nodes, scenarios that Rust has also been designed for.

## Features supported by the parser

- [x] maps
  - [x] structs
  - [x] tables
  - [x] cuts
- [x] groups
- [x] arrays
- [x] values
- [x] choices
- [x] ranges
- [x] enumeration (building a choice from a group)
- [x] root type
- [x] occurrence
- [x] predefined types
- [X] tags
- [x] unwrapping
- [x] controls
- [x] socket/plug
- [x] generics
- [ ] operator precedence
- [x] comments
- [x] numerical int/uint values
- [ ] numerical hexfloat values
- [ ] numerical values with exponents

## Validating JSON

> Incomplete

This library uses the [Serde](https://serde.rs/) framework, and more specifically, the [serde_json](https://crates.io/crates/serde_json) crate, for parsing and validating JSON. Serde was chosen due to its maturity in the ecosystem and its support for serializing and deserializing CBOR via the [serde_cbor](https://crates.io/crates/serde_cbor) crate.

As outlined in [Appendix E.](https://tools.ietf.org/html/rfc8610#appendix-E) of the standard, only the JSON data model subset of CBOR can be used for validation. The limited prelude from the spec has been included below for brevity:

```
any = #

uint = #0
nint = #1
int = uint / nint

tstr = #3
text = tstr

number = int / float

float16 = #7.25
float32 = #7.26
float64 = #7.27
float16-32 = float16 / float32
float32-64 = float32 / float64
float = float16-32 / float64

false = #7.20
true = #7.21
bool = false / true
nil = #7.22
null = nil
```

The first non-group rule defined by a CDDL data structure definition determines the root type, which is subsequently used for validating the top-level JSON data type.

### Supported JSON validation features

The following types and features of CDDL are supported by this library for validating JSON:

|CDDL|JSON|
|----|----|
|structs|objects|
|arrays|arrays|
|text / tstr|string|
|number / int / float|number*|
|bool / true / false|boolean|
|null / nil|null|
|any|any valid JSON|

Occurrence indicators can be used to validate key/value pairs in a JSON object and the number of elements in a JSON array; depending on how the indicators are defined in a CDDL data definition. CDDL groups, generics, sockets/plugs and group-to-choice enumerations are all parsed into their full representations before being evaluated for JSON validation.

All CDDL control operators can be used for validating JSON, with the exception of the `.cbor` and `.cborseq` operators.

*Note: While JSON itself does not distinguish between integers and floating-point numbers, this library does provide the ability to validate numbers against a more specific numerical CBOR type, provided that its equivalent representation is allowed by JSON.

### Comparing with JSON schema

Both CDDL and JSON schema can be used to define JSON data structures. However, the approaches taken to develop these are vastly different. One can refer to the IETF mail archive for more in-depth discussion on the differences between the two.

## `no_std` support

Only the lexer and parser can be used in a `no_std` context provided that a heap allocator is available. This can be enabled by opting out of the default features in your `Cargo.toml` file as follows:

```toml
[dependencies]
cddl = { version = "<version>", default-features = false }
```

JSON validation is dependent on the heap allocated [`Value`](https://docs.rs/serde_json/1.0.40/serde_json/value/index.html) type, but since this type isn't supported in a `no_std` context per https://japaric.github.io/serde-json-core/serde_json_core/index.html#non-features, the JSON validation module does not support `no_std`.

## Dependency graph

Below is a graph of the dependencies used by this project:

![cddl dependencies](dep-graph.png)