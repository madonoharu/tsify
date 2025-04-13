use serde_derive_internals::ast::Field;

/// Attributes that can be applied to a type decorated with `#[derive(Tsify)]`.
/// E.g., through `#[tsify(into_wasm_abi)]`.
#[derive(Debug, Default)]
pub struct TsifyContainerAttrs {
    /// Implement `IntoWasmAbi` for the type.
    pub into_wasm_abi: bool,
    /// Implement `FromWasmAbi` for the type.
    pub from_wasm_abi: bool,
    /// Whether the type should be wrapped in a Typescript namespace.
    pub namespace: bool,
    /// Information about how the type should be serialized.
    pub ty_config: TypeGenerationConfig,
}

/// Configuration affecting how Typescript types are generated.
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
        let mut attrs = Self {
            into_wasm_abi: false,
            from_wasm_abi: false,
            namespace: false,
            ty_config: TypeGenerationConfig::default(),
        };

        for attr in &input.attrs {
            if !attr.path().is_ident("tsify") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
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

                Err(meta.error("unsupported tsify attribute, expected one of `into_wasm_abi`, `from_wasm_abi`, `namespace`, `type_prefix`, `type_suffix`, `missing_as_null`, `hashmap_as_object`, `large_number_types_as_bigints`"))
            })?;
        }

        Ok(attrs)
    }
}

#[derive(Debug, Default)]
pub struct TsifyFieldAttrs {
    pub type_override: Option<String>,
    pub optional: bool,
}

impl TsifyFieldAttrs {
    pub fn from_serde_field(field: &Field) -> syn::Result<Self> {
        let mut attrs = Self {
            type_override: None,
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

                if meta.path.is_ident("optional") {
                    if attrs.optional {
                        return Err(meta.error("duplicate attribute"));
                    }
                    attrs.optional = true;
                    return Ok(());
                }

                Err(meta.error("unsupported tsify attribute, expected one of `type` or `optional`"))
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
