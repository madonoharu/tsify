# tsify Changelog

## v0.5.6

- Resolve the issue with default parameters in generics
- @maartendeprez contributed #33, implements type overrides at the container level

## v0.5.5

- Don't assume a struct named `Range` is automatically a `Range` type
- Put `#[automatically_derived]` on `impl` blocks
- Better handling of `#[serde(skip)]`
- Bump the `wasm_bindgen` dep version

## v0.5.4

- Allow serializing of `Vec<Struct>` provided that `Struct` is serializable.

## v0.5.3

- Propagate errors encountered during serialization.
- More fixes for missing `From` trait implementations.

## v0.5.2

- Fix missing trait bounds for implemented `From` traits.

## v0.5.1

- @Pantamis contributed #22, implementing more `From` traits for more ergonomic use of Futures.
- Fix: empty enums now produce a valid type of `void` rather than producing invalid Typescript.

## v0.5.0

- Forked from `tsify` merging most PRs that were queued on Github
