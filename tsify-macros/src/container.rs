use serde_derive_internals::{ast, ast::Container as SerdeContainer, attr};

use crate::{attrs::TsifyContainerAttrs, error_tracker::ErrorTracker};

/// Data structure storing information about a type decorated with `#[derive(Tsify)]`.
/// This structure also keeps information that was parsed via Serde's macros.
pub struct Container<'a> {
    /// Errors that occurred during processing.
    pub errors: ErrorTracker,
    /// Attributes passed to the `#[derive(Tsify)]` macro.
    pub attrs: TsifyContainerAttrs,
    /// Information about the type as parsed by Serde.
    pub serde_container: SerdeContainer<'a>,
    /// The `ident` of the type as written in the Rust code.
    pub ident_str: String,
    /// The name type that will be serialized to Typescript.
    pub name: String,
}

impl<'a> Container<'a> {
    pub fn new(serde_container: SerdeContainer<'a>) -> Self {
        let input = &serde_container.original;
        let attrs = TsifyContainerAttrs::from_derive_input(input);
        let errors = ErrorTracker::new();

        let attrs = match attrs {
            Ok(attrs) => attrs,
            Err(err) => {
                errors.syn_error(err);
                Default::default()
            }
        };

        let name = attrs
            .ty_config
            .format_name(serde_container.attrs.name().serialize_name().to_string());

        let ident_str = attrs
            .ty_config
            .format_name(serde_container.ident.to_string());

        Self {
            errors,
            attrs,
            serde_container,
            ident_str,
            name,
        }
    }

    pub fn from_derive_input(input: &'a syn::DeriveInput) -> syn::Result<Self> {
        let cx = serde_derive_internals::Ctxt::new();
        let serde_cont =
            SerdeContainer::from_ast(&cx, input, serde_derive_internals::Derive::Serialize);

        match serde_cont {
            Some(serde_container) => {
                cx.check()?;
                Ok(Self::new(serde_container))
            }
            None => Err(cx.check().expect_err("serde_cont is None")),
        }
    }

    /// The `ident` of the type as written in the Rust code.
    pub fn ident(&self) -> &syn::Ident {
        &self.serde_container.ident
    }

    /// The `ident` of the type as written in the Rust code as a string.
    pub fn ident_str(&self) -> String {
        self.ident_str.clone()
    }

    #[inline]
    pub fn serde_attrs(&self) -> &attr::Container {
        &self.serde_container.attrs
    }

    /// Whether or not Serde has marked this type as `transparent`.
    pub fn transparent(&self) -> bool {
        self.serde_attrs().transparent()
    }

    /// The name of the type that will be serialized to Typescript.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Information about the generics associated with the type as parsed by Serde.
    pub fn generics(&self) -> &syn::Generics {
        self.serde_container.generics
    }

    /// Remove the default from every type parameter because in the generated impls
    /// they look like associated types: "error: associated type bindings are not
    /// allowed here".
    pub fn generics_without_defaults(&self) -> syn::Generics {
        let generics = self.generics();
        syn::Generics {
            params: generics
                .params
                .iter()
                .map(|param| match param {
                    syn::GenericParam::Type(param) => syn::GenericParam::Type(syn::TypeParam {
                        eq_token: None,
                        default: None,
                        ..param.clone()
                    }),
                    _ => param.clone(),
                })
                .collect(),
            ..generics.clone()
        }
    }

    /// Information about the data fields of the type as parsed by Serde.
    pub fn serde_data(&self) -> &ast::Data<'_> {
        &self.serde_container.data
    }

    /// Add a new error to the list of processing errors.
    pub fn syn_error(&self, err: syn::Error) {
        self.errors.syn_error(err);
    }

    /// Return all accumulated errors.
    pub fn check(self) -> syn::Result<()> {
        self.errors.check()
    }
}
