# Tsify

Tsify is a library for generating TypeScript definitions from Rust code.

Using this with [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) will automatically output the types to `.d.ts`.

Inspired by [`typescript-definitions`](https://github.com/arabidopsis/typescript-definitions) and [`ts-rs`](https://github.com/Aleph-Alpha/ts-rs).

## Example

<details>
<summary>
Click to show Cargo.toml.
</summary>

```toml
[dependencies]
tsify = "0.5.7"
serde = { version = "1.0", features = ["derive"] }
wasm-bindgen = { version = "0.2" }
```

</details>

```rust
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use tsify::Ts;
use tsify::declare;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsError;

#[declare]
pub type Coordinate = i32;

#[derive(Tsify, Serialize, Deserialize)]
pub struct Point {
    x: Coordinate,
    y: Coordinate,
}

#[wasm_bindgen]
pub fn into_js() -> Result<Ts<Point>, JsError> {
    let point = Point { x: 0, y: 0 };
    Ok(point.into_ts()?)
}

#[wasm_bindgen]
pub fn from_js(point: Ts<Point>) -> Result<(), JsError> {
    let point: Point = point.to_rust()?;
    Ok(())
}
```

Will generate the following `.d.ts` file:

```ts
/* tslint:disable */
/* eslint-disable */
export interface Point {
    x: Coordinate;
    y: Coordinate;
}

export type Coordinate = number;

export function from_js(point: Point): void;

export function into_js(): Point;
```

This is the behavior due to [`typescript_custom_section`](https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/typescript_custom_section.html) and [`Rust Type conversions`](https://rustwasm.github.io/docs/wasm-bindgen/contributing/design/rust-type-conversions.html).

## Crate Features

-   `json` (default) enables serialization through [`serde_json`](https://github.com/serde-rs/json).
-   `js` enables serialization through [`serde-wasm-bindgen`](https://github.com/cloudflare/serde-wasm-bindgen) and generates the appropriate types for it. This will be the default in future versions.

## Attributes

These are the options to modify a `#[derive(Tsify)]` 

Tsify container attributes

-   `namespace` generates a namespace for the enum variants.
-   `type` overrides at the container level.
-   `type_params` overrides params at the container level.

[Serializer configuration options](https://github.com/RReverser/serde-wasm-bindgen?tab=readme-ov-file#serializer-configuration-options)
-   `missing_as_null` 
-   `hashmap_as_object`
-   `large_number_types_as_bigints`

Tsify field attributes

-   `type`
-   `type_params`
-   `optional`

Serde attributes

-   `rename`
-   `rename-all`
-   `tag`
-   `content`
-   `untagged`
-   `skip`
-   `skip_serializing`
-   `skip_deserializing`
-   `skip_serializing_if = "Option::is_none"`
-   `flatten`
-   `default`
-   `transparent`

Deprecated attributes

-   `into_wasm_abi` (deprecated) implements `IntoWasmAbi` and `OptionIntoWasmAbi`. This can be converted directly from Rust to JS via `serde_json` or `serde-wasm-bindgen`. Deprecated in favour of using `Ts<T>` as on function parameters and return type ([why](#why-are-the-wasm_abi-attributes-deprecated)).
-   `from_wasm_abi` (deprecated) implements `FromWasmAbi` and `OptionFromWasmAbi`. This is the opposite operation of the above. Deprecated in favour of using `Ts<T>` as on function parameters and return type ([why](#why-are-the-wasm_abi-attributes-deprecated)).

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
export interface Foo {
    x: 0 | 1 | 2;
}
```

## Optional Properties

```rust
use tsify::Tsify;

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
export interface Optional {
    a?: number;
    b?: string;
    c?: number;
}
```

## Enum

```rust
use tsify::Tsify;

#[derive(Tsify)]
enum Color {
    Red,
    Blue,
    Green,
    Rgb(u8, u8, u8),
    Hsv {
        hue: f64,
        saturation: f64,
        value: f64,
    },
}
```

Generated type:

```ts
export type Color =
    | "Red"
    | "Blue"
    | "Green"
    | { Rgb: [number, number, number] }
    | { Hsv: { hue: number; saturation: number; value: number } };
```

## Enum with namespace

```rust
use tsify::Tsify;

#[derive(Tsify)]
#[tsify(namespace)]
enum Color {
    Red,
    Blue,
    Green,
    Rgb(u8, u8, u8),
    Hsv {
        hue: f64,
        saturation: f64,
        value: f64,
    },
}
```

Generated type:

```ts
declare namespace Color {
    export type Red = "Red";
    export type Blue = "Blue";
    export type Green = "Green";
    export type Rgb = { Rgb: [number, number, number] };
    export type Hsv = {
        Hsv: { hue: number; saturation: number; value: number };
    };
}

export type Color = Color.Red | Color.Blue | Color.Green | Color.Rgb | Color.Hsv;
```

## Type Aliases

```rust
use tsify::{declare, Tsify};

#[derive(Tsify)]
struct Foo<T>(T);

#[declare]
type Bar = Foo<i32>;
```

Generated type:

```ts
export type Foo<T> = T;
export type Bar = Foo<number>;
```

### Why are the `wasm_abi` attributes deprecated?

`#[tsify(into_wasm_abi, from_wasm_abi)]` moves (de)serialization *into the wasm-bindgen ABI boundary*, and that boundary cannot report failure.

`wasm_bindgen::convert::FromWasmAbi::from_abi` returns `Self`, not `Result<Self, _>`, and there is no fallible variant of it or of `RefFromWasmAbi` / `LongRefFromWasmAbi` / `VectorFromWasmAbi`. So when serde fails to deserialize what JavaScript passed in, the generated impl has only one way out: `wasm_bindgen::throw_str`, which raises a JavaScript exception that unwinds straight past the wasm frames. As wasm-bindgen's own documentation warns:

> Note that it is very easy to leak memory with this function because this function, unlike `panic!` on other platforms, **will not run destructors**.

Everything alive at that moment leaks: the serde error, the partially deserialized value, and — because arguments are converted one after another — every argument already converted before the failing one. From JavaScript this looks like an ordinary, catchable exception, so an application can appear to handle bad input correctly while its wasm heap grows on every failure, until the instance dies with `RuntimeError: memory access out of bounds` (see [#65](https://github.com/madonoharu/tsify/issues/65) and [#86](https://github.com/madonoharu/tsify/issues/86)).

`Ts<T>` keeps the boundary infallible: it is a `#[repr(transparent)]` wrapper whose `FromWasmAbi` impl only forwards the underlying `JsValue`. Deserialization then happens inside your function, where it is an ordinary `Result` — the `from_js` example at the top of this page shows the shape. Because the function returns normally, destructors run and nothing leaks. The generated TypeScript is unchanged, so `.d.ts` consumers are unaffected.

`Ts<T>` needs only `#[derive(Tsify)]` — do not add `#[tsify(from_wasm_abi)]` alongside it. Note also that `Ts<Vec<T>>` is not supported, only `Vec<Ts<T>>`; to convert a whole vector, use `items.into_iter().map(|x| x.to_rust()).collect::<Result<Vec<_>, _>>()?`.
