use std::cell::RefCell;

use darling::FromDeriveInput;
use serde_derive_internals::{ast, ast::Container as SerdeContainer, attr};

use crate::attrs::TsifyContainerAttars;

fn syn_errors_into_darling_error(errors: Vec<syn::Error>) -> darling::Error {
    let errors = errors.into_iter().map(darling::Error::from).collect();
    darling::Error::multiple(errors)
}
pub struct Container<'a> {
    pub attrs: TsifyContainerAttars,
    pub serde_container: SerdeContainer<'a>,
    errors: RefCell<Option<Vec<darling::Error>>>,
}

impl<'a> Container<'a> {
    pub fn new(serde_container: SerdeContainer<'a>) -> Self {
        let input = &serde_container.original;
        let attrs = TsifyContainerAttars::from_derive_input(input);
        let mut errors = Vec::new();

        let attrs = match attrs {
            Ok(attrs) => attrs,
            Err(err) => {
                errors.push(err);
                Default::default()
            }
        };

        Self {
            errors: RefCell::new(Some(errors)),
            attrs,
            serde_container,
        }
    }

    pub fn from_derive_input(input: &'a syn::DeriveInput) -> Result<Self, darling::Error> {
        use serde_derive_internals::{Ctxt, Derive};

        let cx = Ctxt::new();
        let serde_cont = SerdeContainer::from_ast(&cx, input, Derive::Serialize);

        match serde_cont {
            Some(serde_container) => {
                cx.check().map_err(syn_errors_into_darling_error)?;
                Ok(Self::new(serde_container))
            }
            None => {
                let errors = cx.check().expect_err("serde_cont is None");
                Err(syn_errors_into_darling_error(errors))
            }
        }
    }

    pub fn ident(&self) -> &syn::Ident {
        &self.serde_container.ident
    }

    #[inline]
    pub fn serde_attrs(&self) -> &attr::Container {
        &self.serde_container.attrs
    }

    pub fn transparent(&self) -> bool {
        self.serde_attrs().transparent()
    }

    pub fn name(&self) -> String {
        self.serde_attrs().name().serialize_name()
    }

    pub fn generics(&self) -> &syn::Generics {
        &self.serde_container.generics
    }

    pub fn serde_data(&self) -> &ast::Data {
        &self.serde_container.data
    }

    pub fn darling_error(&self, err: darling::Error) {
        self.errors.borrow_mut().as_mut().unwrap().push(err)
    }

    pub fn check(self) -> Result<(), darling::Error> {
        let errors = self.errors.take().unwrap();

        match errors.len() {
            0 => Ok(()),
            _ => Err(darling::Error::multiple(errors)),
        }
    }
}

impl Drop for Container<'_> {
    fn drop(&mut self) {
        if !std::thread::panicking() && self.errors.borrow().is_some() {
            panic!("forgot to check for errors");
        }
    }
}
