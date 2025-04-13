use core::panic;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn test_convert() {
    #[derive(Debug, PartialEq, Serialize, Deserialize, Tsify)]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    struct Unit;

    let js = Unit.into_js().unwrap();

    if cfg!(feature = "js") {
        assert!(js.is_undefined());
    } else {
        assert!(js.is_null());
    }

    assert_eq!(Unit::from_js(js).unwrap(), Unit);
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
struct SimpleData {
    value: i32,
    text: String,
}

#[wasm_bindgen(inline_js = r#"
function validate(value, validation) {
    validation(value); 

    // Validate twice to make sure the value is not moved in any way to rust
    validation(value); 
}

function validateArray(value, validation) {
    validation(value);

    // Validate twice to make sure the value is not moved in any way to rust
    validation(value); 
}

function noop(value) {}

function noopArray(value) {}

module.exports = { validate, validateArray, noop };
"#)]
extern "C" {
    #[wasm_bindgen(catch, js_name = "validate")]
    pub fn validate_simple_data(
        value: SimpleData,
        validation: &dyn Fn(SimpleData),
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = "validate")]
    pub fn validate_simple_data_ref(
        value: SimpleData,
        validation: &dyn Fn(&SimpleData),
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = "validateArray")]
    pub fn validate_array(
        value: Vec<SimpleData>,
        validation: &dyn Fn(Box<[SimpleData]>),
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = "noop")]
    pub fn do_not_serialize(value: CantBeSerialized) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = "noopArray")]
    pub fn do_not_serialize_vector(value: Vec<CantBeSerialized>) -> Result<(), JsValue>;
}

#[wasm_bindgen_test]
fn test_convert_simple_value_type() {
    let first_value = SimpleData {
        value: 42,
        text: "Hello".to_string(),
    };

    let second_value = SimpleData {
        value: 25,
        text: "World".to_string(),
    };

    let third_value = SimpleData {
        value: 271828,
        text: "Haskell".to_string(),
    };

    validate_simple_data(first_value.clone(), &|val_after| {
        assert_eq!(val_after, first_value);
    })
    .unwrap_throw();

    validate_simple_data_ref(first_value.clone(), &|val_after| {
        assert_eq!(val_after, &first_value);
    })
    .unwrap_throw();

    let values = vec![
        first_value.clone(),
        second_value.clone(),
        third_value.clone(),
    ];
    validate_array(values, &|values| {
        assert_eq!(values.len(), 3);
        assert_eq!(values[0], first_value);
    })
    .unwrap_throw();
}

// Test that the error message encountered during serialization is propagated to the caller
#[derive(Debug, PartialEq, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi)]
struct CantBeSerialized {
    value: i32,
}

impl Serialize for CantBeSerialized {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Err(serde::ser::Error::custom(
            "This type can't be serialized NO_SERIALIZE",
        ))
    }
}

#[wasm_bindgen_test]
#[should_panic(expected = "NO_SERIALIZE")]
fn test_data_that_cant_be_serialized_throws_an_appropriate_error() {
    let val = CantBeSerialized { value: 42 };

    let _ = do_not_serialize(val);
}

#[wasm_bindgen_test]
#[should_panic(expected = "NO_SERIALIZE")]
fn test_vector_of_data_that_cant_be_serialized_throws_an_appropriate_error() {
    let first_value = CantBeSerialized { value: 42 };
    let second_value = CantBeSerialized { value: 43 };
    let third_value = CantBeSerialized { value: 44 };

    let vector = vec![first_value, second_value, third_value];

    let _ = do_not_serialize_vector(vector);
}
