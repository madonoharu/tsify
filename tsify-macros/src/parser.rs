use std::collections::HashSet;

use crate::{
    attrs::TsifyFieldAttrs,
    comments::extract_doc_comments,
    container::Container,
    decl::{Decl, TsEnumDecl, TsInterfaceDecl, TsTypeAliasDecl, TsValueEnumDecl},
    typescript::{
        TsType, TsTypeElement, TsTypeElementKey, TsTypeLit, TsValueEnumLit, TsValueEnumMember,
    },
};
use serde_derive_internals::{
    ast::{Data, Field, Style, Variant},
    attr::TagType,
};

enum ParsedFields {
    Named(Vec<TsTypeElement>, Vec<TsType>),
    Unnamed(Vec<TsType>),
    Transparent(TsType),
}

impl From<ParsedFields> for TsType {
    fn from(fields: ParsedFields) -> Self {
        match fields {
            ParsedFields::Named(members, extends) => {
                let type_lit = TsType::from(TsTypeLit { members });

                if extends.is_empty() {
                    type_lit
                } else {
                    type_lit.and(TsType::Intersection(extends))
                }
            }
            ParsedFields::Unnamed(elems) => TsType::Tuple(elems),
            ParsedFields::Transparent(ty) => ty,
        }
    }
}

enum FieldsStyle {
    Named,
    Unnamed,
}

#[derive(Clone)]
pub struct Parser<'a> {
    pub container: &'a Container<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(container: &'a Container<'a>) -> Self {
        Self { container }
    }

    pub fn parse(&self) -> Decl {
        if let Some(decl) = &self.container.attrs.type_override {
            self.create_type_alias_decl(TsType::Override {
                type_override: decl.to_string(),
                type_params: self
                    .container
                    .generics()
                    .type_params()
                    .map(|p| p.ident.to_string())
                    .collect(),
            })
        } else {
            match self.container.serde_data() {
                Data::Struct(style, fields) => self.parse_struct(*style, fields),
                Data::Enum(variants) => {
                    if self.container.attrs.value_enum {
                        self.parse_value_enum(variants)
                    } else {
                        self.parse_enum(variants)
                    }
                }
            }
        }
    }

    fn create_relevant_type_params(&self, type_ref_names: HashSet<&String>) -> Vec<String> {
        self.container
            .generics()
            .type_params()
            .map(|p| p.ident.to_string())
            .filter(|t| type_ref_names.contains(t))
            .collect()
    }

    fn create_type_alias_decl(&self, type_ann: TsType) -> Decl {
        Decl::TsTypeAlias(TsTypeAliasDecl {
            id: self.container.ident_str(),
            export: true,
            type_params: self
                .container
                .attrs
                .type_params
                .as_ref()
                .cloned()
                .unwrap_or_else(|| self.create_relevant_type_params(type_ann.type_ref_names())),
            type_ann,
            comments: extract_doc_comments(&self.container.serde_container.original.attrs),
        })
    }

    fn create_decl(&self, members: Vec<TsTypeElement>, extends: Vec<TsType>) -> Decl {
        // An interface can only extend an identifier/qualified-name with optional type arguments.
        if extends.iter().all(|ty| ty.is_ref()) {
            let mut type_ref_names: HashSet<&String> = HashSet::new();
            members.iter().for_each(|member| {
                type_ref_names.extend(member.type_ann.type_ref_names());
            });
            extends.iter().for_each(|ty| {
                type_ref_names.extend(ty.type_ref_names());
            });

            let type_params = self
                .container
                .attrs
                .type_params
                .as_ref()
                .cloned()
                .unwrap_or_else(|| self.create_relevant_type_params(type_ref_names));

            Decl::TsInterface(TsInterfaceDecl {
                id: self.container.ident_str(),
                type_params,
                extends,
                body: members,
                comments: extract_doc_comments(&self.container.serde_container.original.attrs),
            })
        } else {
            let extra = TsType::Intersection(
                extends
                    .into_iter()
                    .map(|ty| match ty {
                        TsType::Option(ty, _) => TsType::Union(vec![*ty, TsType::empty_type_lit()]),
                        _ => ty,
                    })
                    .collect(),
            );
            let type_ann = TsType::TypeLit(TsTypeLit { members }).and(extra);

            self.create_type_alias_decl(type_ann)
        }
    }

    fn parse_struct(&self, style: Style, fields: &[Field]) -> Decl {
        let parsed_fields = self.parse_fields(style, fields);
        let tag_type = self.container.serde_attrs().tag();

        match (tag_type, parsed_fields) {
            (TagType::Internal { tag }, ParsedFields::Named(members, extends)) => {
                let name = self.container.name();

                let tag_field = TsTypeElement {
                    key: tag.clone().into(),
                    type_ann: TsType::Lit(name),
                    optional: false,
                    comments: vec![],
                };

                let mut vec = Vec::with_capacity(members.len() + 1);
                vec.push(tag_field);
                vec.extend(members);

                self.create_decl(vec, extends)
            }
            (_, ParsedFields::Named(members, extends)) => self.create_decl(members, extends),
            (_, parsed_fields) => self.create_type_alias_decl(parsed_fields.into()),
        }
    }

    fn parse_fields(&self, style: Style, fields: &[Field]) -> ParsedFields {
        let style = match style {
            Style::Struct => FieldsStyle::Named,
            Style::Newtype => return ParsedFields::Transparent(self.parse_field(&fields[0]).0),
            Style::Tuple => FieldsStyle::Unnamed,
            Style::Unit => {
                return ParsedFields::Transparent(TsType::nullish(&self.container.attrs.ty_config))
            }
        };

        let fields = fields
            .iter()
            .filter(|field| {
                !field.attrs.skip_serializing()
                    && !field.attrs.skip_deserializing()
                    && !is_phantom(field.ty)
            })
            .collect::<Vec<_>>();

        if fields.len() == 1 && self.container.transparent() {
            return ParsedFields::Transparent(self.parse_field(fields[0]).0);
        }

        match style {
            FieldsStyle::Named => {
                let (members, flatten_fields) = self.parse_named_fields(fields);

                ParsedFields::Named(members, flatten_fields)
            }
            FieldsStyle::Unnamed => {
                let elems = fields
                    .into_iter()
                    .map(|field| self.parse_field(field).0)
                    .collect();

                ParsedFields::Unnamed(elems)
            }
        }
    }

    fn parse_field(&self, field: &Field) -> (TsType, Option<TsifyFieldAttrs>) {
        let ts_attrs = match TsifyFieldAttrs::from_serde_field(field) {
            Ok(attrs) => attrs,
            Err(err) => {
                self.container.syn_error(err);
                return (TsType::NEVER, None);
            }
        };

        let type_ann = TsType::from_syn_type(&self.container.attrs.ty_config, field.ty);

        if let Some(t) = &ts_attrs.type_override {
            let type_params = if let Some(params) = &ts_attrs.type_params {
                params.clone()
            } else {
                let type_ref_names = type_ann.type_ref_names();
                self.create_relevant_type_params(type_ref_names)
            };
            (
                TsType::Override {
                    type_override: t.clone(),
                    type_params,
                },
                Some(ts_attrs),
            )
        } else {
            (type_ann, Some(ts_attrs))
        }
    }

    fn parse_named_fields(&self, fields: Vec<&Field>) -> (Vec<TsTypeElement>, Vec<TsType>) {
        let (flatten_fields, members): (Vec<_>, Vec<_>) =
            fields.into_iter().partition(|field| field.attrs.flatten());

        let members = members
            .into_iter()
            .map(|field| {
                let key = field.attrs.name().serialize_name().to_owned();
                let (type_ann, field_attrs) = self.parse_field(field);

                let optional = field_attrs.is_some_and(|attrs| attrs.optional);
                let default_is_none = self.container.serde_attrs().default().is_none()
                    && field.attrs.default().is_none();

                let type_ann = if optional {
                    match type_ann {
                        TsType::Option(t, _) => *t,
                        _ => type_ann,
                    }
                } else {
                    type_ann
                };

                let comments = extract_doc_comments(&field.original.attrs);

                TsTypeElement {
                    key: key.into(),
                    type_ann,
                    optional: optional || !default_is_none,
                    comments,
                }
            })
            .collect();

        let flatten_fields = flatten_fields
            .into_iter()
            .map(|field| self.parse_field(field).0)
            .collect();

        (members, flatten_fields)
    }

    fn parse_value_enum(&self, variants: &[Variant]) -> Decl {
        let members = variants
            .into_iter()
            .filter(|v| !v.attrs.skip_serializing() && !v.attrs.skip_deserializing())
            .map(|variant| {
                let variant_serialized = variant.attrs.name().serialize_name();
                let member_value = if self.container.attrs.rename_variants {
                    variant.ident.to_string()
                } else {
                    variant_serialized.to_owned()
                };

                TsValueEnumMember {
                    id: member_value,
                    value: TsValueEnumLit::StringLit(variant_serialized.to_owned()),
                    comments: extract_doc_comments(&variant.original.attrs),
                }
            })
            .collect::<Vec<_>>();

        Decl::TsValueEnum(TsValueEnumDecl {
            id: self.container.ident_str(),
            constant: false,
            members,
        })
    }

    fn parse_enum(&self, variants: &[Variant]) -> Decl {
        let mut discriminants = self.container.attrs.discriminants.to_enum_decl();

        let members = variants
            .into_iter()
            .filter(|v| !v.attrs.skip_serializing() && !v.attrs.skip_deserializing())
            .map(|variant| {
                let variant_serialized = variant.attrs.name().serialize_name();
                let variant_name = if self.container.attrs.rename_variants {
                    variant.ident.to_string()
                } else {
                    variant_serialized.to_owned()
                };

                let discriminant = if let Some(discriminants) = &discriminants {
                    TsTypeElementKey::Var(format!("{}.{}", discriminants.id, variant_name))
                } else {
                    TsTypeElementKey::Lit(variant_serialized.to_owned())
                };

                let decl = self.create_type_alias_decl(self.parse_variant(variant, discriminant));
                if let Decl::TsTypeAlias(mut type_alias) = decl {
                    type_alias.id = variant_name;
                    type_alias.comments = extract_doc_comments(&variant.original.attrs);

                    if let Some(discriminants) = &mut discriminants {
                        discriminants.members.push(TsValueEnumMember {
                            id: type_alias.id.clone(),
                            value: TsValueEnumLit::StringLit(variant_serialized.to_owned()),
                            comments: type_alias.comments.clone(),
                        })
                    }

                    type_alias
                } else {
                    panic!();
                }
            })
            .collect::<Vec<_>>();

        let type_ref_names = members
            .iter()
            .flat_map(|type_alias| type_alias.type_ann.type_ref_names())
            .collect::<HashSet<_>>();

        let relevant_type_params = self.create_relevant_type_params(type_ref_names);

        Decl::TsEnum(TsEnumDecl {
            id: self.container.ident_str(),
            type_params: relevant_type_params,
            members,
            namespace: self.container.attrs.namespace,
            discriminants,
            comments: extract_doc_comments(&self.container.serde_container.original.attrs),
        })
    }

    fn parse_variant(&self, variant: &Variant, key: TsTypeElementKey) -> TsType {
        let tag_type = self.container.serde_attrs().tag();
        // Checks for Newtype with a skip attribute and treats it as a Unit
        let style = if matches!(variant.style, Style::Newtype)
            && (variant.fields[0].attrs.skip_serializing()
                || variant.fields[0].attrs.skip_deserializing()
                || is_phantom(variant.fields[0].ty))
        {
            Style::Unit
        } else {
            variant.style
        };
        let type_ann: TsType = self.parse_fields(style, &variant.fields).into();
        type_ann.with_tag_type(&self.container.attrs.ty_config, key, style, tag_type)
    }
}

fn is_phantom(ty: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath { path, .. }) = ty {
        path.segments
            .last()
            .is_some_and(|path| path.ident == "PhantomData")
    } else {
        false
    }
}
