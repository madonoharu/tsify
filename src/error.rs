use std::fmt;

/// An error type wrapping serialization or deserialization errors from either `serde_json` or
/// `serde_wasm_bindgen`, depending on whether the `json` or `js` feature is enabled.
///
#[derive(Debug)]
pub struct Error {
    // All private internals
    //
    /// XXX: May want to put type-name printing behind an off-by-default feature flag,
    /// as it does add bloat that some users might not want.
    pub(crate) type_name: &'static str,
    /// true -> During deserialization; false -> during serialization
    pub(crate) de: bool,
    pub(crate) inner: SerializationError,
}

#[cfg(all(feature = "json", not(feature = "js")))]
type SerializationError = serde_json::Error;
#[cfg(feature = "js")]
type SerializationError = serde_wasm_bindgen::Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.de {
            write!(
                f,
                "Failed to deserialize JsValue into type `{}`: {}",
                self.type_name, self.inner
            )
        } else {
            write!(
                f,
                "Failed to serialize type `{}` into JsValue: {}",
                self.type_name, self.inner
            )
        }
    }
}

/// We implement [std::error::Error] so you can use `?` in `-> Result<_,
/// wasm_bindgen::JsError>` functions to automatically throw.
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}
