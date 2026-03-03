use std::collections::HashSet;

use serde_derive_internals::{ast::Style, attr::TagType};

use crate::attrs::TypeGenerationConfig;

use super::{NullType, TsKeywordTypeKind, TsTypeElement, TsTypeElementKey, TsTypeLit};

/// A TypeScript type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TsType {
    /// A keyword type like `number`, `string`, etc.
    Keyword(TsKeywordTypeKind),
    /// A literal type like `"foo"`, `42`, etc.
    Lit(String),
    /// A computed type like `foo_bar`, etc.
    Computed(String),
    /// An array type like `number[]`, `(number | string)[]`, etc.
    Array(Box<Self>),
    /// A tuple type like `[number, string]`, `[number, string, boolean]`, etc.
    Tuple(Vec<Self>),
    /// An optional type along with how a missing value is represented (i.e., as `undefined` or `null`).
    Option(Box<Self>, NullType),
    /// A reference to a type like `Foo`, `Bar<T>`, etc.
    Ref {
        name: String,
        type_params: Vec<Self>,
    },
    /// A function type like `(arg0: string, arg1: number) => void`
    Fn {
        params: Vec<Self>,
        type_ann: Box<Self>,
    },
    /// A type literal like `{ foo: number; bar: string; }`
    TypeLit(TsTypeLit),
    /// An intersection type like `number & string`, `(number & string) & boolean`, etc.
    Intersection(Vec<Self>),
    /// A union type like `number | string`, `(number | string) | boolean`, etc.
    Union(Vec<Self>),
    /// Explicitly specified type
    Override {
        type_override: String,
        type_params: Vec<String>,
    },
}

impl From<TsKeywordTypeKind> for TsType {
    fn from(kind: TsKeywordTypeKind) -> Self {
        Self::Keyword(kind)
    }
}

impl TsType {
    pub const NUMBER: TsType = TsType::Keyword(TsKeywordTypeKind::Number);
    pub const BIGINT: TsType = TsType::Keyword(TsKeywordTypeKind::Bigint);
    pub const BOOLEAN: TsType = TsType::Keyword(TsKeywordTypeKind::Boolean);
    pub const STRING: TsType = TsType::Keyword(TsKeywordTypeKind::String);
    pub const VOID: TsType = TsType::Keyword(TsKeywordTypeKind::Void);
    pub const UNDEFINED: TsType = TsType::Keyword(TsKeywordTypeKind::Undefined);
    pub const NULL: TsType = TsType::Keyword(TsKeywordTypeKind::Null);
    pub const NEVER: TsType = TsType::Keyword(TsKeywordTypeKind::Never);

    pub const fn nullish(config: &TypeGenerationConfig) -> Self {
        NullType::new(config).to_type()
    }

    pub const fn empty_type_lit() -> Self {
        Self::TypeLit(TsTypeLit { members: vec![] })
    }

    pub fn is_ref(&self) -> bool {
        matches!(self, Self::Ref { .. })
    }

    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (TsType::TypeLit(x), TsType::TypeLit(y)) => x.and(y).into(),
            (TsType::Intersection(x), TsType::Intersection(y)) => {
                let mut vec = Vec::with_capacity(x.len() + y.len());
                vec.extend(x);
                vec.extend(y);
                TsType::Intersection(vec)
            }
            (TsType::Intersection(x), y) => {
                let mut vec = Vec::with_capacity(x.len() + 1);
                vec.extend(x);
                vec.push(y);
                TsType::Intersection(vec)
            }
            (x, TsType::Intersection(y)) => {
                let mut vec = Vec::with_capacity(y.len() + 1);
                vec.push(x);
                vec.extend(y);
                TsType::Intersection(vec)
            }
            (x, y) => TsType::Intersection(vec![x, y]),
        }
    }

    /// Convert a `syn::Type` to a `TsType`
    pub fn from_syn_type(config: &TypeGenerationConfig, ty: &syn::Type) -> Self {
        use syn::Type::*;
        use syn::{
            TypeArray, TypeBareFn, TypeGroup, TypeImplTrait, TypeParamBound, TypeParen, TypePath,
            TypeReference, TypeSlice, TypeTraitObject, TypeTuple,
        };

        match ty {
            Array(TypeArray { elem, len, .. }) => {
                let elem = Self::from_syn_type(config, elem);
                let len = parse_len(len);

                match len {
                    Some(len) if len <= 16 => Self::Tuple(vec![elem; len]),
                    _ => Self::Array(Box::new(elem)),
                }
            }

            Slice(TypeSlice { elem, .. }) => {
                Self::Array(Box::new(Self::from_syn_type(config, elem)))
            }

            Reference(TypeReference { elem, .. })
            | Paren(TypeParen { elem, .. })
            | Group(TypeGroup { elem, .. }) => Self::from_syn_type(config, elem),

            BareFn(TypeBareFn { inputs, output, .. }) => {
                let params = inputs
                    .iter()
                    .map(|arg| Self::from_syn_type(config, &arg.ty))
                    .collect();

                let type_ann = if let syn::ReturnType::Type(_, ty) = output {
                    Self::from_syn_type(config, ty)
                } else {
                    TsType::VOID
                };

                Self::Fn {
                    params,
                    type_ann: Box::new(type_ann),
                }
            }

            Tuple(TypeTuple { elems, .. }) => {
                if elems.is_empty() {
                    TsType::nullish(config)
                } else {
                    let elems = elems
                        .iter()
                        .map(|ty| Self::from_syn_type(config, ty))
                        .collect();
                    Self::Tuple(elems)
                }
            }

            Path(TypePath { path, .. }) => Self::from_path(config, path).unwrap_or(TsType::NEVER),

            TraitObject(TypeTraitObject { bounds, .. })
            | ImplTrait(TypeImplTrait { bounds, .. }) => {
                let elems = bounds
                    .iter()
                    .filter_map(|t| match t {
                        TypeParamBound::Trait(t) => Self::from_path(config, &t.path),
                        _ => None, // skip lifetime etc.
                    })
                    .collect();

                Self::Intersection(elems)
            }

            Ptr(_) | Infer(_) | Macro(_) | Never(_) | Verbatim(_) => TsType::NEVER,

            _ => TsType::NEVER,
        }
    }

    /// Convert a `syn::Path` to a `TsType`. For example `core::option::Option<i32>` would be
    /// converted to `Self::Option(number)`.
    fn from_path(config: &TypeGenerationConfig, path: &syn::Path) -> Option<Self> {
        path.segments
            .last()
            .map(|segment| Self::from_terminal_path_segment(config, segment))
    }

    /// Convert a `syn::PathSegment` to a `TsType`. For example `Option<i32>` would be converted to
    /// `Self::Option(number)`.
    fn from_terminal_path_segment(
        config: &TypeGenerationConfig,
        segment: &syn::PathSegment,
    ) -> Self {
        let name = segment.ident.to_string();

        let (args, output) = match &segment.arguments {
            syn::PathArguments::AngleBracketed(path) => {
                let args = path
                    .args
                    .iter()
                    .filter_map(|p| match p {
                        syn::GenericArgument::Type(t) => Some(t),
                        syn::GenericArgument::AssocType(t) => Some(&t.ty),
                        _ => None,
                    })
                    .collect();

                (args, None)
            }

            syn::PathArguments::Parenthesized(path) => {
                let args = path.inputs.iter().collect();

                let output = match &path.output {
                    syn::ReturnType::Default => None,
                    syn::ReturnType::Type(_, tp) => Some(tp.as_ref()),
                };

                (args, output)
            }

            syn::PathArguments::None => (vec![], None),
        };

        Self::from_name(config, &name, args, output)
    }

    pub fn with_tag_type(
        self,
        config: &TypeGenerationConfig,
        key: TsTypeElementKey,
        style: Style,
        tag_type: &TagType,
    ) -> Self {
        let type_ann = self;

        match tag_type {
            TagType::External => {
                if matches!(style, Style::Unit) {
                    match key {
                        TsTypeElementKey::Lit(s) => TsType::Lit(s),
                        TsTypeElementKey::Var(v) => TsType::Computed(v),
                    }
                } else {
                    TsTypeElement {
                        key,
                        type_ann,
                        optional: false,
                        comments: vec![],
                    }
                    .into()
                }
            }
            TagType::Internal { tag } => {
                if type_ann == TsType::nullish(config) {
                    let tag_field: TsType = TsTypeElement {
                        key: tag.clone().into(),
                        type_ann: key.into(),
                        optional: false,
                        comments: vec![],
                    }
                    .into();

                    tag_field
                } else {
                    let tag_field: TsType = TsTypeElement {
                        key: tag.clone().into(),
                        type_ann: key.into(),
                        optional: false,
                        comments: vec![],
                    }
                    .into();

                    tag_field.and(type_ann)
                }
            }
            TagType::Adjacent { tag, content } => {
                let tag_field = TsTypeElement {
                    key: tag.clone().into(),
                    type_ann: key.into(),
                    optional: false,
                    comments: vec![],
                };

                if matches!(style, Style::Unit) {
                    tag_field.into()
                } else {
                    let content_field = TsTypeElement {
                        key: content.clone().into(),
                        type_ann,
                        optional: false,
                        comments: vec![],
                    };

                    TsTypeLit {
                        members: vec![tag_field, content_field],
                    }
                    .into()
                }
            }
            TagType::None => type_ann,
        }
    }

    pub fn visit<'a, F: FnMut(&'a TsType)>(&'a self, f: &mut F) {
        f(self);

        match self {
            TsType::Ref { type_params, .. } => {
                type_params.iter().for_each(|t| t.visit(f));
            }
            TsType::Array(elem) => elem.visit(f),
            TsType::Tuple(elems) => {
                elems.iter().for_each(|t| t.visit(f));
            }
            TsType::Option(t, _) => t.visit(f),
            TsType::Fn { params, type_ann } => {
                params
                    .iter()
                    .chain(Some(type_ann.as_ref()))
                    .for_each(|t| t.visit(f));
            }
            TsType::TypeLit(TsTypeLit { members }) => {
                members.iter().for_each(|m| m.type_ann.visit(f));
            }
            TsType::Intersection(tys) | TsType::Union(tys) => {
                tys.iter().for_each(|t| t.visit(f));
            }
            TsType::Keyword(_) | TsType::Lit(_) | TsType::Computed(_) | TsType::Override { .. } => {
            }
        }
    }

    pub fn type_ref_names(&self) -> HashSet<&String> {
        let mut set: HashSet<&String> = HashSet::new();

        self.visit(&mut |ty: &TsType| match ty {
            TsType::Ref { name, .. } => {
                set.insert(name);
            }
            TsType::Override { type_params, .. } => set.extend(type_params),
            _ => (),
        });

        set
    }

    pub fn prefix_type_refs(self, prefix: &String, exceptions: &Vec<String>) -> Self {
        match self {
            TsType::Array(t) => TsType::Array(Box::new(t.prefix_type_refs(prefix, exceptions))),
            TsType::Tuple(tv) => TsType::Tuple(
                tv.iter()
                    .map(|t| t.clone().prefix_type_refs(prefix, exceptions))
                    .collect(),
            ),
            TsType::Option(t, null) => {
                TsType::Option(Box::new(t.prefix_type_refs(prefix, exceptions)), null)
            }
            TsType::Ref { name, type_params } => {
                if exceptions.contains(&name) {
                    TsType::Ref {
                        name,
                        type_params: type_params
                            .iter()
                            .map(|t| t.clone().prefix_type_refs(prefix, exceptions))
                            .collect(),
                    }
                } else {
                    TsType::Ref {
                        name: format!("{}{}", prefix, name),
                        type_params: type_params
                            .iter()
                            .map(|t| t.clone().prefix_type_refs(prefix, exceptions))
                            .collect(),
                    }
                }
            }
            TsType::Fn { params, type_ann } => TsType::Fn {
                params: params
                    .iter()
                    .map(|t| t.clone().prefix_type_refs(prefix, exceptions))
                    .collect(),
                type_ann: Box::new(type_ann.prefix_type_refs(prefix, exceptions)),
            },
            TsType::TypeLit(lit) => TsType::TypeLit(TsTypeLit {
                members: lit
                    .members
                    .iter()
                    .map(|t| TsTypeElement {
                        key: t.key.clone(),
                        optional: t.optional,
                        type_ann: t.type_ann.clone().prefix_type_refs(prefix, exceptions),
                        comments: t.comments.clone(),
                    })
                    .collect(),
            }),
            TsType::Intersection(tv) => TsType::Intersection(
                tv.iter()
                    .map(|t| t.clone().prefix_type_refs(prefix, exceptions))
                    .collect(),
            ),
            TsType::Union(tv) => TsType::Union(
                tv.iter()
                    .map(|t| t.clone().prefix_type_refs(prefix, exceptions))
                    .collect(),
            ),
            _ => self,
        }
    }

    pub fn type_refs(&self, type_refs: &mut Vec<(String, Vec<TsType>)>) {
        match self {
            TsType::Array(t) | TsType::Option(t, _) => t.type_refs(type_refs),
            TsType::Tuple(tv) | TsType::Union(tv) | TsType::Intersection(tv) => {
                tv.iter().for_each(|t| t.type_refs(type_refs))
            }
            TsType::Ref { name, type_params } => {
                type_refs.push((name.clone(), type_params.clone()));
                type_params
                    .iter()
                    .for_each(|t| t.clone().type_refs(type_refs));
            }
            TsType::Fn { params, type_ann } => {
                params.iter().for_each(|t| t.clone().type_refs(type_refs));
                type_ann.type_refs(type_refs);
            }
            TsType::TypeLit(lit) => {
                lit.members.iter().for_each(|t| {
                    t.type_ann.type_refs(type_refs);
                });
            }
            _ => {}
        }
    }
}

fn parse_len(expr: &syn::Expr) -> Option<usize> {
    if let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Int(lit_int),
        ..
    }) = expr
    {
        lit_int.base10_parse::<usize>().ok()
    } else {
        None
    }
}

#[cfg(test)]
#[path = "ts_type.test.rs"]
mod test;
