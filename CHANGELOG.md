# tsify Changelog

## v0.5.7

- Added `Ts<T>`, a wrapper for `#[wasm_bindgen]` parameters and return types. `#[tsify(into_wasm_abi, from_wasm_abi)]` leak memory whenever (de)serialization fails: the conversion happens at the ABI boundary, where the only way to report failure is `wasm_bindgen::throw_str`, which does not run destructors. `Ts<T>` moves the conversion into the function body, where it is an ordinary `Result`. Resolves #65, #47 and #86. @cormacrelf contributed #71
- Deprecated `into_wasm_abi` and `from_wasm_abi` in favour of `Ts<T>`. The README explains the mechanism; the attributes still work, and no removal is planned
- `Ts<T>` can now be returned from `async fn`. @hgiesel contributed #84
- `#[tsify(namespace)]` enums now emit `export type E = E.A | E.B` instead of repeating each variant's shape in the union. @hgiesel contributed #78
- Fixed raw string artifacts in doc comments copied into the generated TypeScript. @samkearney contributed the fix

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
