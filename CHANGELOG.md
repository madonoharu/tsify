# tsify Changelog

## Unreleased

- **A default type parameter is now declared with its default.** `struct Foo<T = bool>(T)` declared `export type Foo<T> = T;` and now declares `export type Foo<T = boolean> = T;`, the same for interfaces, enums and `#[declare]` aliases. **Your `.d.ts` changes if you use one.** A default that cannot be honoured is dropped: one naming a parameter no field mentions, and then every default before it, since TypeScript only allows them on a trailing run
- While [#76](https://github.com/madonoharu/tsify/issues/76) is open this makes it quieter — `fn bar(foo: Ts<Foo<i64>>)` still writes `bar(foo: Foo)`, which the default now resolves to `Foo<boolean>` rather than raising `TS2314`

## v0.5.8

- Added `#[tsify(rename = "...")]`, which renames the generated TypeScript declaration and nothing else — references from other types still emit the Rust ident, so point them at the new name with `#[tsify(type = "...")]` at each reference site. Cannot be combined with `type_prefix` or `type_suffix`. Resolves #70, which had been blocking the 0.4 → 0.5 upgrade for anyone with two same-named types in different modules
- **`type_prefix` and `type_suffix` no longer rename type parameters.** A type parameter has no declaration for the affix to rename, so `#[tsify(type_prefix = "Ts")] struct Wrapper<T> { x: T }` declared `export interface TsWrapper { x: TsT; }` — the parameter dropped, and `TsT` never declared anywhere. It now declares `export interface TsWrapper<T> { x: T; }`. **If you use either attribute on a generic type, its `.d.ts` changes.** There was no correct way to use the affix on a generic type before this, so nothing that type-checked without `skipLibCheck` is affected
- **`type_prefix` and `type_suffix` no longer reach the tag literal of an internally-tagged type.** That value comes from serde, which was never told about the affix, so `#[serde(tag = "kind")] #[tsify(type_prefix = "A")] struct Config` declared `kind: "AConfig"` where serde has always serialized `"Config"`. It now declares `kind: "Config"`. **If you narrowed on the old literal, that code compiled and never matched at runtime; it is now a compile error.** Applying either attribute to only some of your types still emits references to names that were never declared — that half of #94 is still open

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
