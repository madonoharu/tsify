// sentinel: staled on purpose to verify the nightly update path (PR #99)
type TypeAlias<T, U> = Foo<T, i32, U>;
const _: () = {
    use wasm_bindgen::prelude::*;
};
