use serde_derive_internals::{ast, ast::Container as SerdeContainer, attr};

use crate::{attrs::TsifyContainerAttrs, ctxt::Ctxt};

pub struct Container<'a> {
    pub ctxt: Ctxt,
    pub attrs: TsifyContainerAttrs,
    pub serde_container: SerdeContainer<'a>,
    pub ident_str: String,
    pub name: String,
}

impl<'a> Container<'a> {
    pub fn new(serde_container: SerdeContainer<'a>) -> Self {
        let input = &serde_container.original;
        let attrs = TsifyContainerAttrs::from_derive_input(input);
        let ctxt = Ctxt::new();

        let attrs = match attrs {
            Ok(attrs) => attrs,
            Err(err) => {
                ctxt.syn_error(err);
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
            ctxt,
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

    pub fn ident(&self) -> &syn::Ident {
        &self.serde_container.ident
    }

    pub fn ident_str(&self) -> String {
        self.ident_str.clone()
    }

    #[inline]
    pub fn serde_attrs(&self) -> &attr::Container {
        &self.serde_container.attrs
    }

    pub fn transparent(&self) -> bool {
        self.serde_attrs().transparent()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn generics(&self) -> &syn::Generics {
        self.serde_container.generics
    }

    pub fn serde_data(&self) -> &ast::Data {
        &self.serde_container.data
    }

    pub fn syn_error(&self, err: syn::Error) {
        self.ctxt.syn_error(err);
    }

    pub fn check(self) -> syn::Result<()> {
        self.ctxt.check()
    }
}
