use std::ops::Deref;
use std::{fmt::Display, vec};

use crate::comments::clean_comments;
use crate::{
    comments::write_doc_comments,
    typescript::{TsType, TsTypeElement, TsTypeLit},
};

#[derive(Debug, Clone)]
pub struct TsTypeAliasDecl {
    pub id: String,
    pub export: bool,
    pub type_params: Vec<String>,
    pub type_ann: TsType,
    pub comments: Vec<String>,
}

impl TsTypeAliasDecl {
    pub fn to_string_with_indent(&self, indent: usize) -> String {
        let out = self.to_string();
        let indent_str = " ".repeat(indent);
        out.split('\n')
            .map(|line| format!("{}{}", indent_str, line))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Display for TsTypeAliasDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let right = if self.type_params.is_empty() {
            self.id.clone()
        } else {
            let type_params = self.type_params.join(", ");
            format!("{}<{}>", self.id, type_params)
        };

        write_doc_comments(f, &self.comments)?;

        if self.export {
            write!(f, "export ")?;
        }
        write!(f, "type {} = {};", right, self.type_ann)
    }
}

#[derive(Debug)]
pub struct TsInterfaceDecl {
    pub id: String,
    pub type_params: Vec<String>,
    pub extends: Vec<TsType>,
    pub body: Vec<TsTypeElement>,
    pub comments: Vec<String>,
}

impl Display for TsInterfaceDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_doc_comments(f, &self.comments)?;

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

        write!(f, " ")?;
        TsTypeLit::from(self.body.as_slice()).fmt(f)
    }
}

/// A Typescript type resulting from an enum declaration.
#[derive(Debug)]
pub struct TsEnumDecl {
    pub id: String,
    pub type_params: Vec<String>,
    pub members: Vec<TsTypeAliasDecl>,
    pub namespace: bool,
    pub comments: Vec<String>,
}

const ALPHABET_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn tparam(i: usize) -> String {
    let mut s = String::new();
    let mut i = i;
    loop {
        s.push(ALPHABET_UPPER[i % ALPHABET_UPPER.len()]);
        if i < ALPHABET_UPPER.len() {
            return s;
        }
        i /= ALPHABET_UPPER.len();
    }
}

impl TsEnumDecl {
    fn replace_type_params(ts_type: TsType, type_args: &mut Vec<String>) -> TsType {
        match ts_type {
            TsType::Ref { name, type_params } => TsType::Ref {
                name,
                type_params: type_params
                    .iter()
                    .map(|_| {
                        let name = tparam(type_args.len());
                        type_args.push(name.clone());
                        TsType::Ref {
                            name,
                            type_params: Vec::new(),
                        }
                    })
                    .collect(),
            },
            TsType::Array(t) => TsType::Array(Box::new(TsEnumDecl::replace_type_params(
                t.deref().clone(),
                type_args,
            ))),
            TsType::Tuple(tv) => TsType::Tuple(
                tv.iter()
                    .map(|t| TsEnumDecl::replace_type_params(t.clone(), type_args))
                    .collect(),
            ),
            TsType::Option(t, null) => TsType::Option(
                Box::new(TsEnumDecl::replace_type_params(
                    t.deref().clone(),
                    type_args,
                )),
                null,
            ),
            TsType::Fn { params, type_ann } => TsType::Fn {
                params: params
                    .iter()
                    .map(|t| TsEnumDecl::replace_type_params(t.clone(), type_args))
                    .collect(),
                type_ann: Box::new(TsEnumDecl::replace_type_params(
                    type_ann.deref().clone(),
                    type_args,
                )),
            },
            TsType::TypeLit(lit) => TsType::TypeLit(TsTypeLit {
                members: lit
                    .members
                    .iter()
                    .map(|t| TsTypeElement {
                        key: t.key.clone(),
                        optional: t.optional,
                        type_ann: TsEnumDecl::replace_type_params(t.type_ann.clone(), type_args),
                        comments: vec![],
                    })
                    .collect(),
            }),
            TsType::Intersection(tv) => TsType::Intersection(
                tv.iter()
                    .map(|t| TsEnumDecl::replace_type_params(t.clone(), type_args))
                    .collect(),
            ),
            TsType::Union(tv) => TsType::Union(
                tv.iter()
                    .map(|t| TsEnumDecl::replace_type_params(t.clone(), type_args))
                    .collect(),
            ),
            _ => ts_type,
        }
    }
}

impl Display for TsEnumDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.namespace {
            let mut type_refs = self
                .members
                .iter()
                .flat_map(|type_alias| {
                    let mut type_refs = Vec::new();
                    type_alias.type_ann.type_refs(&mut type_refs);

                    type_refs
                        .iter()
                        .filter(|(name, _)| !self.type_params.contains(name))
                        .map(|(name, type_args)| {
                            let mut type_refs = Vec::new();
                            let ts_type = TsEnumDecl::replace_type_params(
                                TsType::Ref {
                                    name: name.clone(),
                                    type_params: type_args.clone(),
                                },
                                &mut type_refs,
                            );

                            TsTypeAliasDecl {
                                id: format!("__{}{}", self.id, name),
                                export: false,
                                type_params: type_refs,
                                type_ann: ts_type,
                                comments: vec![],
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            type_refs.sort_by_key(|type_ref| type_ref.id.clone());
            type_refs.dedup_by_key(|type_ref| type_ref.id.clone());
            for type_ref in type_refs {
                writeln!(f, "{}", type_ref)?;
            }

            write_doc_comments(f, &self.comments)?;

            write!(f, "declare namespace {}", self.id)?;

            if self.members.is_empty() {
                write!(f, " {{}}")?;
            } else {
                let prefix = format!("__{}", self.id);
                let members = self
                    .members
                    .iter()
                    .map(|elem| TsTypeAliasDecl {
                        id: elem.id.clone(),
                        export: true,
                        type_params: elem.type_params.clone(),
                        type_ann: elem
                            .type_ann
                            .clone()
                            .prefix_type_refs(&prefix, &self.type_params),
                        comments: elem.comments.clone(),
                    })
                    .map(|elem| format!("\n{}", elem.to_string_with_indent(4)))
                    .collect::<Vec<_>>()
                    .join("");

                write!(f, " {{{members}\n}}")?;
            }

            write!(f, "\n\n")?;
        }

        TsTypeAliasDecl {
            id: self.id.clone(),
            export: true,
            type_params: self.type_params.clone(),
            type_ann: TsType::Union(
                self.members
                    .iter()
                    .map(|member| {
                        let mut clone = member.type_ann.clone();
                        clean_comments(&mut clone);
                        clone
                    })
                    .collect(),
            ),
            comments: self.comments.clone(),
        }
        .fmt(f)
    }
}

/// A typescript type declaration. For example `type Foo = string;`
/// or `interface Bar { baz: number; }`
#[allow(clippy::enum_variant_names)]
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
