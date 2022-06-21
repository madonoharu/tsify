use std::fmt::Display;

use crate::typescript::{TsType, TsTypeElement};

#[derive(Clone)]
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

pub struct TsEnumDecl {
    pub id: String,
    pub type_params: Vec<String>,
    pub body: Vec<TsTypeAliasDecl>,
}

impl Display for TsEnumDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_refs = self
            .body
            .iter()
            .flat_map(|type_alias| {
                let mut type_refs = Vec::new();
                type_alias.type_ann.type_refs(&mut type_refs);

                type_refs.iter().filter(|(name, _)| {
                    !self.type_params.contains(name)
                }).map(|(name, type_args)| {
                    let mut alias_type_params = Vec::new();
                    type_args.iter().for_each(|t| t.type_refs(&mut alias_type_params));
                    TsTypeAliasDecl {
                        id: format!("__{}{}", self.id, name),
                        type_params: self.type_params.iter().filter(|tp| alias_type_params
                            .iter()
                            .any(|atp| &atp.0 == *tp))
                            .map(|tp| tp.clone())
                            .collect(),
                        type_ann: TsType::Ref {
                            name: name.clone(),
                            type_params: type_args.clone()
                        }
                    }
                }).collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        for type_ref in type_refs {
            write!(f, "{}\n", type_ref)?;
        }
        write!(f, "declare namespace {}", self.id)?;

        if self.body.is_empty() {
            write!(f, " {{}}")?;
        } else {
            let prefix = format!("__{}", self.id);
            let members = self
                .body
                .iter()
                .map(|elem| {
                    TsTypeAliasDecl {
                        id: elem.id.clone(),
                        type_params: elem.type_params.clone(),
                        type_ann: elem.type_ann.clone().prefix_type_refs(
                            &prefix,
                            &self.type_params,
                        ),
                    }
                })
                .map(|elem| format!("\n    {elem}"))
                .collect::<Vec<_>>()
                .join("");

            write!(f, " {{{members}\n}}")?;
        }

        write!(f, "\n\n")?;

        TsTypeAliasDecl {
            id: self.id.clone(),
            type_params: self.type_params.clone(),
            type_ann: TsType::Union(
                self.body
                    .iter()
                    .map(|elem| TsType::Ref {
                        name: format!("{}.{}", self.id, elem.id),
                        type_params: elem
                            .type_params
                            .iter()
                            .map(|param| TsType::Ref { name: param.clone(), type_params: Vec::new() })
                            .collect()
                    })
                    .collect()
            ),
        }.fmt(f)
    }
}

pub enum Decl {
    TsTypeAlias(TsTypeAliasDecl),
    TsInterface(TsInterfaceDecl),
    TsEnum(TsEnumDecl),
}

impl Decl {
    pub fn id(&self) -> &String {
        match self {
            Decl::TsTypeAlias(decl) => &decl.id,
            Decl::TsInterface(decl) => &decl.id,
            Decl::TsEnum(decl) => &decl.id,
        }
    }
}

impl Display for Decl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Decl::TsTypeAlias(decl) => decl.fmt(f),
            Decl::TsInterface(decl) => decl.fmt(f),
            Decl::TsEnum(decl) => decl.fmt(f),
        }
    }
}
