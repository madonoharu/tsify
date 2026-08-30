use std::ops::Deref;
use std::{fmt::Display, vec};

use crate::comments::clean_comments;
use crate::{
    comments::write_doc_comments,
    typescript::{TsType, TsTypeElement, TsTypeLit, TsTypeRef, TsTypeRefSource},
};

/// A parameter of a generated declaration: the name it is declared under, and
/// the type TypeScript falls back to where a reference leaves it out.
///
/// The two are kept apart because a parameter is written differently depending
/// on where it appears. `<T = boolean>` declares one; naming the default again
/// where the type is *used* is a syntax error, and matching a parameter by name
/// -- which is how a reference is told apart from a type it could be confused
/// with -- has to see `T` either way.
#[derive(Debug, Clone)]
pub struct TsTypeParam {
    pub name: String,
    pub default: Option<TsType>,
}

impl TsTypeParam {
    pub fn new(name: String) -> Self {
        Self {
            name,
            default: None,
        }
    }

    /// From a parameter written as TypeScript, which is how
    /// `#[tsify(type_params = "...")]` supplies them. The name is whatever
    /// precedes a default; the default is carried through as written.
    pub fn parse(source: &str) -> Self {
        match source.split_once('=') {
            Some((name, default)) => Self {
                name: name.trim().to_string(),
                default: Some(TsType::Override {
                    type_override: default.trim().to_string(),
                    type_params: Vec::new(),
                }),
            },
            None => Self::new(source.trim().to_string()),
        }
    }
}

/// Settles which defaults survive, and drops the rest.
///
/// A default is written inside the parameter list, so that is where its names
/// resolve: against the parameters first, and the declarations around them only
/// after. That gives three ways for a default to mean something other than the
/// Rust it came from, none of which can be spelled around, because a parameter
/// shadows within its own list:
///
/// * it names a parameter this declaration does not declare — one that no field
///   mentions is declared nowhere — so the name reaches nothing;
/// * it names a parameter declared after it, which TypeScript rejects outright
///   as `TS2744`;
/// * it names a type of its own whose name a parameter has taken, and the
///   parameter wins.
///
/// The default goes in each case. TypeScript then accepts defaults only on a
/// trailing run of parameters, so opening a gap takes the defaults before it as
/// well.
pub fn resolve_defaults(params: &mut [TsTypeParam]) {
    let names = param_names(params);

    for index in 0..params.len() {
        let resolves = match &params[index].default {
            Some(default) => {
                let mut type_refs = Vec::new();
                default.type_refs(&mut type_refs);

                type_refs.iter().all(|type_ref| match type_ref.source {
                    TsTypeRefSource::TypeParam => names[..index].contains(&type_ref.name),
                    TsTypeRefSource::Rust(_) | TsTypeRefSource::Synthetic => {
                        !names.contains(&type_ref.name)
                    }
                })
            }
            None => continue,
        };

        if !resolves {
            params[index].default = None;
        }
    }

    if let Some(last_without) = params.iter().rposition(|param| param.default.is_none()) {
        for param in &mut params[..last_without] {
            param.default = None;
        }
    }
}

/// The declaration form, `A, B = number`.
fn declared_params(params: &[TsTypeParam]) -> String {
    params
        .iter()
        .map(|param| match &param.default {
            Some(default) => format!("{} = {}", param.name, default),
            None => param.name.clone(),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// The names alone, in order, for a reference or for matching by name.
fn param_names(params: &[TsTypeParam]) -> Vec<String> {
    params.iter().map(|param| param.name.clone()).collect()
}

#[derive(Debug, Clone)]
pub struct TsTypeAliasDecl {
    pub id: String,
    pub export: bool,
    pub type_params: Vec<TsTypeParam>,
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
            let type_params = declared_params(&self.type_params);
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
    pub type_params: Vec<TsTypeParam>,
    pub extends: Vec<TsType>,
    pub body: Vec<TsTypeElement>,
    pub comments: Vec<String>,
}

impl Display for TsInterfaceDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_doc_comments(f, &self.comments)?;

        write!(f, "export interface {}", self.id)?;

        if !self.type_params.is_empty() {
            let type_params = declared_params(&self.type_params);
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
                .map(|elem| format!("\n{};", elem.to_string_with_indent(4)))
                .collect::<Vec<_>>()
                .join("");

            write!(f, " {{{members}\n}}")
        }
    }
}

/// A Typescript type resulting from an enum declaration.
#[derive(Debug)]
pub struct TsEnumDecl {
    pub id: String,
    pub type_params: Vec<TsTypeParam>,
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
            TsType::Ref(type_ref) => TsType::Ref(TsTypeRef {
                type_params: type_ref
                    .type_params
                    .iter()
                    .map(|_| {
                        let name = tparam(type_args.len());
                        type_args.push(name.clone());
                        TsType::Ref(TsTypeRef {
                            name,
                            source: TsTypeRefSource::TypeParam,
                            type_params: Vec::new(),
                        })
                    })
                    .collect(),
                ..type_ref
            }),
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

                    // A default is a type like any other, and is read from
                    // inside the namespace, where a sibling variant can have
                    // taken the name it meant.
                    for param in &type_alias.type_params {
                        if let Some(default) = &param.default {
                            default.type_refs(&mut type_refs);
                        }
                    }

                    type_refs
                        .iter()
                        .filter(|type_ref| {
                            !self
                                .type_params
                                .iter()
                                .any(|param| param.name == type_ref.name)
                        })
                        .map(|type_ref| {
                            let mut type_refs = Vec::new();
                            let ts_type = TsEnumDecl::replace_type_params(
                                TsType::Ref(type_ref.clone()),
                                &mut type_refs,
                            );

                            TsTypeAliasDecl {
                                id: format!("__{}{}", self.id, type_ref.name),
                                export: false,
                                type_params: type_refs.into_iter().map(TsTypeParam::new).collect(),
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
                let exceptions = param_names(&self.type_params);
                let members =
                    self.members
                        .iter()
                        .map(|elem| TsTypeAliasDecl {
                            id: elem.id.clone(),
                            export: true,
                            type_params: elem
                                .type_params
                                .iter()
                                .map(|param| TsTypeParam {
                                    name: param.name.clone(),
                                    default: param.default.clone().map(|default| {
                                        default.prefix_type_refs(&prefix, &exceptions)
                                    }),
                                })
                                .collect(),
                            type_ann: elem.type_ann.clone().prefix_type_refs(&prefix, &exceptions),
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
                        // TODO remove this once type_alias are properly formatted
                        let mut clone = member.clone();
                        clean_comments(&mut clone.type_ann);

                        if self.namespace {
                            let name = if clone.type_params.is_empty() {
                                format!("{}.{}", self.id, clone.id)
                            } else {
                                let type_params = param_names(&clone.type_params).join(", ");
                                format!("{}.{}<{}>", self.id, clone.id, type_params)
                            };

                            TsType::Ref(TsTypeRef {
                                name,
                                source: TsTypeRefSource::Synthetic,
                                type_params: vec![],
                            })
                        } else {
                            clone.type_ann.clone()
                        }
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
