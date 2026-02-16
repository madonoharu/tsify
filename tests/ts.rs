//! Tests for Ts<T>'s implementations of the unstable wasm_bindgen::convert / describe traits.
//! Based on tests/wasm.rs

use tsify::Ts;
use tsify::Tsify;

use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsError;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn test_convert() {
    #[derive(Debug, PartialEq, Serialize, Deserialize, Tsify)]
    struct Unit;

    let ts = Unit.into_ts().unwrap();

    let js = ts.js_value();
    if cfg!(feature = "js") {
        assert!(js.is_undefined());
    } else {
        assert!(js.is_null());
    }

    assert_eq!(ts.to_rust().unwrap(), Unit);
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Tsify, Clone)]
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
        value: Ts<SimpleData>,
        validation: &dyn Fn(Ts<SimpleData>),
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = "validate")]
    pub fn validate_simple_data_ref(
        value: &Ts<SimpleData>,
        validation: &dyn Fn(&Ts<SimpleData>),
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = "validateArray")]
    pub fn validate_array(
        value: Vec<Ts<SimpleData>>,
        validation: &dyn Fn(Box<[Ts<SimpleData>]>),
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = "noop")]
    pub fn do_not_serialize(value: Ts<CantBeSerialized>) -> Result<(), JsValue>;
}

#[wasm_bindgen_test]
fn test_convert_simple_value_type() -> Result<(), JsError> {
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

    validate_simple_data(first_value.into_ts().unwrap(), &|val_after| {
        let from_ts = val_after
            .to_rust()
            .expect("validate_simple_data to deserialize");
        assert_eq!(from_ts, first_value);
    })
    .unwrap_throw();

    validate_simple_data_ref(&first_value.into_ts().unwrap(), &|val_after| {
        let from_ts = val_after
            .to_rust()
            .expect("validate_simple_data_ref to deserialize");
        assert_eq!(&from_ts, &first_value);
    })
    .unwrap_throw();

    let values = vec![
        first_value.into_ts().unwrap(),
        second_value.into_ts().unwrap(),
        third_value.into_ts().unwrap(),
    ];
    validate_array(values, &|values| {
        assert_eq!(values.len(), 3);
        assert_eq!(
            values[0].to_rust().expect("validate_array to deserialize"),
            first_value
        );
    })
    .unwrap_throw();

    Ok(())
}

// Test that the error message encountered during serialization is propagated to the caller
#[derive(Debug, PartialEq, Tsify, Clone)]
struct CantBeSerialized {
    value: i32,
}

impl<'de> Deserialize<'de> for CantBeSerialized {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Err(serde::de::Error::custom(
            "This type can't be deserialized NO_SERIALIZE",
        ))
    }
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
    let val = CantBeSerialized { value: 42 }.into_ts().unwrap();

    let _ = do_not_serialize(val).unwrap();
}

// No point testing Vec<Ts<CantBeSerialized>> here, since you call the same
// CantBeSerialized::into_ts to build such a thing.

#[wasm_bindgen_test]
fn error_includes_type_name_ser() {
    let val = CantBeSerialized { value: 42 };
    let err = val.into_ts().unwrap_err();
    let err_msg = format!("{}", err);
    wasm_bindgen_test::console_log!("Error message: {}", err_msg);
    assert!(err_msg.contains("serialize type `ts::CantBeSerialized`"));
}

#[wasm_bindgen_test]
fn error_includes_type_name_de() {
    let val: Ts<CantBeSerialized> = Ts::new_unchecked(JsValue::NULL);
    let err = val.to_rust().unwrap_err();
    let err_msg = format!("{}", err);
    wasm_bindgen_test::console_log!("Error message: {}", err_msg);
    assert!(err_msg.contains("deserialize"));
    assert!(err_msg.contains("`ts::CantBeSerialized`"));
}
