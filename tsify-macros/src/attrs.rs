use darling::{FromDeriveInput, FromField};
use serde_derive_internals::ast::Field;

#[derive(Debug, Default, FromDeriveInput)]
#[darling(attributes(tsify), default)]
pub struct TsifyContainerAttars {
    pub into_wasm_abi: bool,
    pub from_wasm_abi: bool,
}

#[derive(Debug, Default, FromField)]
#[darling(attributes(tsify), default)]
pub struct TsifyFieldAttrs {
    #[darling(rename = "type")]
    pub type_override: Option<String>,
    pub optional: bool,
}

impl TsifyFieldAttrs {
    pub fn from_serde_field(field: &Field) -> darling::Result<Self> {
        let mut attrs = Self::from_field(&field.original)?;

        if let Some(expr) = field.attrs.skip_serializing_if() {
            let path = expr
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");

            attrs.optional |= &path == "Option::is_none";
        }

        Ok(attrs)
    }
}
