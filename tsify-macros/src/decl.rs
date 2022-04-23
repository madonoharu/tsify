use std::fmt::Display;

use crate::typescript::{TsType, TsTypeElement};

pub struct TsTypeAliasDecl {
    pub id: String,
    pub type_params: Vec<String>,
    pub type_ann: TsType,
}

impl Display for TsTypeAliasDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let right = if self.type_params.is_empty() {
            self.id.clone()
        } else {
            let type_params = self.type_params.join(", ");
            format!("{}<{}>", self.id, type_params)
        };

        write!(f, "export type {} = {};", right, self.type_ann)
    }
}

pub struct TsInterfaceDecl {
    pub id: String,
    pub type_params: Vec<String>,
    pub extends: Vec<TsType>,
    pub body: Vec<TsTypeElement>,
}

impl Display for TsInterfaceDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "export interface {}", self.id)?;

        if !self.type_params.is_empty() {
            let type_params = self.type_params.join(", ");
            write!(f, "<{type_params}>")?;
        }

        if !self.extends.is_empty() {
            let extends = self
                .extends
                .iter()
                .map(|ty| ty.to_string())
                .collect::<Vec<_>>()
                .join(", ");

            write!(f, " extends {extends}")?;
        }

        if self.body.is_empty() {
            write!(f, " {{}}")
        } else {
            let members = self
                .body
                .iter()
                .map(|elem| format!("\n    {elem};"))
                .collect::<Vec<_>>()
                .join("");

            write!(f, " {{{members}\n}}")
        }
    }
}

pub enum Decl {
    TsTypeAlias(TsTypeAliasDecl),
    TsInterface(TsInterfaceDecl),
}

impl Decl {
    pub fn id(&self) -> &String {
        match self {
            Decl::TsTypeAlias(decl) => &decl.id,
            Decl::TsInterface(decl) => &decl.id,
        }
    }
}

impl Display for Decl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Decl::TsTypeAlias(decl) => decl.fmt(f),
            Decl::TsInterface(decl) => decl.fmt(f),
        }
    }
}
