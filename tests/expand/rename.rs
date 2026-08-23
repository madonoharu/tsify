use tsify::Tsify;

#[derive(Tsify)]
#[tsify(rename = "RenamedWasmType")]
pub struct RustType {
    value: String,
}
