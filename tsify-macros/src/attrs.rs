use crate::decl::TsValueEnumDecl;
use serde_derive_internals::ast::Field;
use std::any::Any;
use std::borrow::Cow;

/// Attributes that can be applied to a type decorated with `#[derive(Tsify)]`.
/// E.g., through `#[tsify(into_wasm_abi)]`.
#[derive(Debug, Default)]
pub struct TsifyContainerAttrs {
    pub type_override: Option<String>,
    pub type_params: Option<Vec<String>>,
    /// Implement `IntoWasmAbi` for the type.
    pub into_wasm_abi: bool,
    /// Implement `FromWasmAbi` for the type.
    pub from_wasm_abi: bool,
    /// How to rename the variant identifier. At this stage, just defining it means it shouldn't change.
    pub rename_variants: bool,
    /// Must be enum. Whether the variant types should be wrapped in a TypeScript namespace.
    pub namespace: bool,
    /// Must be enum. Whether enum with variant identifiers should be generated.
    pub discriminants: DiscriminantEnumGenerationConfig,
    /// Must be enum with unit variants only. Whether TypeScript should be generated.
    pub value_enum: bool,
    /// Information about how the type should be serialized.
    pub ty_config: TypeGenerationConfig,
}

/// Configuration whether type discriminant enum is generated.
#[derive(Debug, Default, PartialEq)]
pub enum DiscriminantEnumGenerationConfig {
    #[default]
    NoGeneration,
    InferName,
    WithName(String),
}

impl DiscriminantEnumGenerationConfig {
    pub fn as_name(&self, container_name: &str) -> Option<Cow<str>> {
        match self {
            DiscriminantEnumGenerationConfig::NoGeneration => None,
            DiscriminantEnumGenerationConfig::InferName => {
                Some(Cow::Owned(format!("{container_name}Type")))
            }
            DiscriminantEnumGenerationConfig::WithName(name) => Some(Cow::Borrowed(name)),
        }
    }

    pub fn to_enum_decl(&self, container_name: &str) -> Option<TsValueEnumDecl> {
        self.as_name(container_name).map(|name| TsValueEnumDecl {
            id: name.to_string(),
            constant: false,
            members: vec![],
        })
    }
}

/// Configuration affecting how TypeScript types are generated.
#[derive(Debug, Default)]
pub struct TypeGenerationConfig {
    /// Universal prefix for generated types
    pub type_prefix: Option<String>,
    /// Universal suffix for generated types
    pub type_suffix: Option<String>,
    /// Whether missing fields should be represented as null in Typescript
    pub missing_as_null: bool,
    /// Whether a hashmap should be represented as an object in Typescript
    pub hashmap_as_object: bool,
    /// Whether large number types should be represented as BigInts in Typescript
    pub large_number_types_as_bigints: bool,
}

impl TypeGenerationConfig {
    /// Format a type `name` adding a prefix and suffix if they are set.
    pub fn format_name(&self, name: String) -> String {
        let prefix = self.type_prefix.as_ref().map_or("", String::as_str);
        let suffix = self.type_suffix.as_ref().map_or("", String::as_str);
        format!("{}{}{}", prefix, name, suffix)
    }
}

impl TsifyContainerAttrs {
    pub fn from_derive_input(input: &syn::DeriveInput) -> syn::Result<Self> {
        let mut attrs = Self::default();

        for attr in &input.attrs {
            if !attr.path().is_ident("tsify") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("type") {
                    if attrs.type_override.is_some() {
                        return Err(meta.error("duplicate attribute"));
                    }
                    let lit = meta.value()?.parse::<syn::LitStr>()?;
                    attrs.type_override = Some(lit.value());
                    return Ok(());
                }

                if meta.path.is_ident("type_params") {
                    if attrs.type_params.is_some() {
                        return Err(meta.error("duplicate attribute"));
                    }
                    let lit = meta.value()?.parse::<syn::LitStr>()?;
                    attrs.type_params = Some(lit.value().split(',').map(|s| s.trim().to_string()).collect());
                    return Ok(());
                }

                if meta.path.is_ident("into_wasm_abi") {
                    if attrs.into_wasm_abi {
                        return Err(meta.error("duplicate attribute"));
                    }
                    attrs.into_wasm_abi = true;
                    return Ok(());
                }

                if meta.path.is_ident("from_wasm_abi") {
                    if attrs.from_wasm_abi {
                        return Err(meta.error("duplicate attribute"));
                    }
                    attrs.from_wasm_abi = true;
                    return Ok(());
                }

                if meta.path.is_ident("namespace") {
                    if !matches!(input.data, syn::Data::Enum(_)) {
                        return Err(meta.error("#[tsify(namespace)] can only be used on enums"));
                    }
                    if attrs.namespace {
                        return Err(meta.error("duplicate attribute"));
                    }
                    attrs.namespace = true;
                    return Ok(());
                }

                if meta.path.is_ident("discriminants") {
                    let value = meta
                        .input
                        .peek(syn::Token![=])
                        .then(|| {
                            let _: syn::Token![=] = meta.input.parse()?;
                            let lit: syn::LitStr = meta.input.parse()?;
                            Ok(lit.value())
                        })
                        .transpose()
                        .map_err(|_: syn::Error| meta.error(r#"#[tsify(discriminants = "..")] if you want to specify a discriminant type name"#))?;

                    if !matches!(input.data, syn::Data::Enum(_)) {
                        return Err(meta.error("#[tsify(discriminants)] can only be used on enums"));
                    }
                    if !matches!(attrs.discriminants, DiscriminantEnumGenerationConfig::NoGeneration) {
                        return Err(meta.error("duplicate attribute"));
                    }
                    attrs.discriminants = match value {
                        None => DiscriminantEnumGenerationConfig::InferName,
                        Some(name) => DiscriminantEnumGenerationConfig::WithName(name),
                    };
                    return Ok(());
                }

                if meta.path.is_ident("rename_variants") {
                    if !matches!(input.data, syn::Data::Enum(_)) {
                        return Err(meta.error("#[tsify(rename_variants)] can only be used on enums"));
                    }
                    if attrs.rename_variants {
                        return Err(meta.error("duplicate attribute"));
                    }
                    attrs.rename_variants = true;
                    return Ok(());
                }

                if meta.path.is_ident("value_enum") {
                    if !matches!(input.data, syn::Data::Enum(_)) {
                        return Err(meta.error("#[tsify(value_enum)] can only be used on enums"));
                    }
                    if attrs.value_enum {
                        return Err(meta.error("duplicate attribute"));
                    }
                    attrs.value_enum = true;
                    return Ok(());
                }

                if meta.path.is_ident("type_prefix") {
                    if attrs.ty_config.type_prefix.is_some() {
                        return Err(meta.error("duplicate attribute"));
                    }
                    let lit: syn::LitStr = meta.value()?.parse()?;
                    attrs.ty_config.type_prefix = Some(lit.value());
                    return Ok(());
                }

                if meta.path.is_ident("type_suffix") {
                    if attrs.ty_config.type_suffix.is_some() {
                        return Err(meta.error("duplicate attribute"));
                    }
                    let lit: syn::LitStr = meta.value()?.parse()?;
                    attrs.ty_config.type_suffix = Some(lit.value());
                    return Ok(());
                }

                if meta.path.is_ident("missing_as_null") {
                    if attrs.ty_config.missing_as_null {
                        return Err(meta.error("duplicate attribute"));
                    }
                    if cfg!(not(feature = "js")) {
                        return Err(meta.error(
                            "#[tsify(missing_as_null)] requires the `js` feature",
                        ));
                    }
                    attrs.ty_config.missing_as_null = true;
                    return Ok(());
                }

                if meta.path.is_ident("hashmap_as_object") {
                    if attrs.ty_config.hashmap_as_object {
                        return Err(meta.error("duplicate attribute"));
                    }
                    if cfg!(not(feature = "js")) {
                        return Err(meta.error(
                            "#[tsify(hashmap_as_object)] requires the `js` feature",
                        ));
                    }
                    attrs.ty_config.hashmap_as_object = true;
                    return Ok(());
                }

                if meta.path.is_ident("large_number_types_as_bigints") {
                    if attrs.ty_config.large_number_types_as_bigints {
                        return Err(meta.error("duplicate attribute"));
                    }
                    if cfg!(not(feature = "js")) {
                        return Err(meta.error(
                            "#[tsify(large_number_types_as_bigints)] requires the `js` feature",
                        ));
                    }
                    attrs.ty_config.large_number_types_as_bigints = true;
                    return Ok(());
                }

                Err(meta.error("unsupported tsify attribute, expected one of `type`, `type_params`, `into_wasm_abi`, `from_wasm_abi`, `rename_variant`, namespace`, `discriminants`, `value_enum`, `type_prefix`, `type_suffix`, `missing_as_null`, `hashmap_as_object`, `large_number_types_as_bigints`"))
            })?;
        }

        Ok(attrs)
    }
}

#[derive(Debug, Default)]
pub struct TsifyFieldAttrs {
    pub type_override: Option<String>,
    pub type_params: Option<Vec<String>>,
    pub optional: bool,
}

impl TsifyFieldAttrs {
    pub fn from_serde_field(field: &Field) -> syn::Result<Self> {
        let mut attrs = Self {
            type_override: None,
            type_params: None,
            optional: false,
        };

        for attr in &field.original.attrs {
            if !attr.path().is_ident("tsify") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("type") {
                    if attrs.type_override.is_some() {
                        return Err(meta.error("duplicate attribute"));
                    }
                    let lit = meta.value()?.parse::<syn::LitStr>()?;
                    attrs.type_override = Some(lit.value());
                    return Ok(());
                }

                if meta.path.is_ident("type_params") {
                    if attrs.type_params.is_some() {
                        return Err(meta.error("duplicate attribute"));
                    }
                    let lit = meta.value()?.parse::<syn::LitStr>()?;
                    attrs.type_params = Some(
                        lit.value()
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect(),
                    );
                    return Ok(());
                }

                if meta.path.is_ident("optional") {
                    if attrs.optional {
                        return Err(meta.error("duplicate attribute"));
                    }
                    attrs.optional = true;
                    return Ok(());
                }

                Err(meta.error("unsupported tsify attribute, expected one of `type`, `type_params` or `optional`"))
            })?;
        }

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
