use serde_derive_internals::ast::Field;

#[derive(Copy, Clone, Debug)]
pub enum ContainerType {
    Enum,
    Struct,
    Alias,
}

#[derive(Debug, Default)]
pub struct TsifyContainerAttrs {
    pub into_wasm_abi: bool,
    pub from_wasm_abi: bool,
    pub namespace: bool,
    pub ty_config: TypeGenerationConfig,
}

#[derive(Debug, Default)]
pub struct TypeGenerationConfig {
    pub type_prefix: Option<String>,
    pub type_suffix: Option<String>,
}
impl TypeGenerationConfig {
    pub fn format_name(&self, mut name: String) -> String {
        if let Some(ref prefix) = self.type_prefix {
            name.insert_str(0, prefix);
        }
        if let Some(ref suffix) = self.type_suffix {
            name.push_str(suffix);
        }
        name
    }
}

impl TsifyContainerAttrs {
    pub fn from_derive_input(input: &syn::DeriveInput) -> syn::Result<Self> {
        let mut attrs = Self::default();

        let container_type = match input.data {
            syn::Data::Enum(_) => ContainerType::Enum,
            syn::Data::Struct(_) => ContainerType::Struct,
            syn::Data::Union(_) => unreachable!(),
        };

        for attr in &input.attrs {
            if !attr.path().is_ident("tsify") {
                continue;
            }

            attr.parse_nested_meta(|meta| attrs.from_nested_meta(meta, container_type))?;
        }

        Ok(attrs)
    }

    pub fn from_nested_meta(
        &mut self,
        meta: syn::meta::ParseNestedMeta<'_>,
        container_type: ContainerType,
    ) -> Result<(), syn::Error> {
        if meta.path.is_ident("into_wasm_abi") {
            if matches!(container_type, ContainerType::Alias) {
                return Err(
                    meta.error("#[tsify(into_wasm_abi)] can only be used on structs and enums")
                );
            }
            if self.into_wasm_abi {
                return Err(meta.error("duplicate attribute"));
            }
            self.into_wasm_abi = true;
            return Ok(());
        }
        if meta.path.is_ident("from_wasm_abi") {
            if matches!(container_type, ContainerType::Alias) {
                return Err(
                    meta.error("#[tsify(from_wasm_abi)] can only be used on structs and enums")
                );
            }
            if self.from_wasm_abi {
                return Err(meta.error("duplicate attribute"));
            }
            self.from_wasm_abi = true;
            return Ok(());
        }
        if meta.path.is_ident("namespace") {
            if !matches!(container_type, ContainerType::Enum) {
                return Err(meta.error("#[tsify(namespace)] can only be used on enums"));
            }
            if self.namespace {
                return Err(meta.error("duplicate attribute"));
            }
            self.namespace = true;
            return Ok(());
        }
        if meta.path.is_ident("type_prefix") {
            if self.ty_config.type_prefix.is_some() {
                return Err(meta.error("duplicate attribute"));
            }
            let lit: syn::LitStr = meta.value()?.parse()?;
            self.ty_config.type_prefix = Some(lit.value());
            return Ok(());
        }
        if meta.path.is_ident("type_suffix") {
            if self.ty_config.type_suffix.is_some() {
                return Err(meta.error("duplicate attribute"));
            }
            let lit: syn::LitStr = meta.value()?.parse()?;
            self.ty_config.type_suffix = Some(lit.value());
            return Ok(());
        }
        Err(meta.error("unsupported tsify attribute, expected one of `into_wasm_abi`, `from_wasm_abi`, `namespace`, 'type_prefix', 'type_suffix'"))
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
