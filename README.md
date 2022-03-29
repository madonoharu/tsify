# Tsify

Tsify is a library for generating TypeScript definitions from rust code.

Using this with [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) will automatically output the types to `.d.ts`.

Inspired by [`typescript-definitions`](https://github.com/arabidopsis/typescript-definitions) and [`ts-rs`](https://github.com/Aleph-Alpha/ts-rs).

## Example

<details>
<summary>
Click to show Cargo.toml.
</summary>

```toml
[dependencies]
tsify = "0.1"
serde = { version = "1.0", features = ["derive"] }
wasm-bindgen = { version = "0.2", features = ["serde-serialize"] }
```

</details>

```rust
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Point {
    x: i32,
    y: i32,
}

#[wasm_bindgen]
pub fn into_js() -> Point {
    Point { x: 0, y: 0 }
}

#[wasm_bindgen]
pub fn from_js(point: Point) {}
```

Will generate the following `.d.ts` file:

```ts
/* tslint:disable */
/* eslint-disable */
/**
 * @returns {Point}
 */
export function into_js(): Point;
/**
 * @param {Point} point
 */
export function from_js(point: Point): void;
export type Point = { x: number; y: number };
```

## Attributes

Tsify container attributes

- `into_wasm_abi` implements `IntoWasmAbi` and `OptionIntoWasmAbi`. This can be converted directly from Rust to JS via JSON.
- `from_wasm_abi` implements `FromWasmAbi` and `OptionFromWasmAbi`. This is the opposite operation of the above.

Tsify field attributes

- `type`
- `optional`

Serde attributes

- `rename`
- `rename-all`
- `tag`
- `content`
- `untagged`
- `skip`
- `skip_serializing`
- `skip_deserializing`
- `skip_serializing_if = "Option::is_none"`
- `flatten`
- `default`
- `transparent`

## Type Override

```rust
use tsify::Tsify;

#[derive(Tsify)]
pub struct Foo {
    #[tsify(type = "0 | 1 | 2")]
    x: i32,
}
```

Generated type:

```ts
export type Foo = { x: 0 | 1 | 2 };
```

## Optional Properties

```rust
#[derive(Tsify)]
struct Optional {
    #[tsify(optional)]
    a: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    b: Option<String>,
    #[serde(default)]
    c: i32,
}
```

Generated type:

```ts
export type Optional = { a?: number; b?: string; c?: number };
```

## Crate Features

- `wasm-bindgen-impl` (default) Generate
  [`typescript_custom_section`](https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/typescript_custom_section.html)
  and
  [`Rust Type conversions`](https://rustwasm.github.io/docs/wasm-bindgen/contributing/design/rust-type-conversions.html)
