use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Point {
    x: i32,
    y: i32,
}

#[allow(unused_variables)]
#[wasm_bindgen]
pub fn consume(point: Point) {}

#[wasm_bindgen]
pub fn into_js() -> Point {
    Point { x: 0, y: 0 }
}

#[allow(unused_variables)]
#[wasm_bindgen]
pub fn consume_vector(points: Vec<Point>) {}

#[wasm_bindgen]
pub fn vector_into_js() -> Vec<Point> {
    vec![
        Point { x: 1, y: 6 },
        Point { x: 2, y: 5 },
        Point { x: 3, y: 4 },
    ]
}
