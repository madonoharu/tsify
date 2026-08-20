# tsify Changelog

## v0.5.7

- Added `Ts<T>`, a wrapper for `#[wasm_bindgen]` parameters and return types. `#[tsify(from_wasm_abi)]` deserializes at the ABI boundary, which cannot report failure, so bad input from JavaScript ends in `wasm_bindgen::throw_str` — a catchable JS exception that skips destructors, leaking a little on every failure until the instance dies. `Ts<T>` keeps the boundary infallible and moves the conversion into the function body, where it is an ordinary `Result`. Addresses #65, #47 and #86. @cormacrelf contributed #71
- Deprecated `into_wasm_abi` and `from_wasm_abi` in favour of `Ts<T>`. `into_wasm_abi` panics rather than leaks on failure, but it has the same root cause and the same fix. The attributes still work, and no removal is planned; see the README for details
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
