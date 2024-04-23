use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
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

module.exports = { validate };
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
}

#[wasm_bindgen_test]
fn test_convert_simple_value_type() {
    let val = SimpleData {
        value: 42,
        text: "Hello".to_string(),
    };

    validate_simple_data(val.clone(), &|val_after| {
        assert_eq!(val_after, val);
    })
    .unwrap_throw();

    validate_simple_data_ref(val.clone(), &|val_after| {
        assert_eq!(val_after, &val);
    })
    .unwrap_throw();
}
