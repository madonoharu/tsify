use std::collections::HashSet;

use serde_derive_internals::{
    ast::{Data, Field, Style, Variant},
    attr::TagType,
};

use crate::{
    attrs::TsifyFieldAttrs,
    container::Container,
    decl::{Decl, TsInterfaceDecl, TsTypeAliasDecl},
    typescript::{TsType, TsTypeElement, TsTypeLit},
};
use crate::decl::TsEnumDecl;

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
        match self.container.serde_data() {
            Data::Struct(style, ref fields) => self.parse_struct(*style, fields),
            Data::Enum(ref variants) => self.parse_enum(variants),
        }
    }

    fn create_enum_decl(&self, body: Vec<TsTypeAliasDecl>) -> Decl {
        let type_ref_names = body.iter()
            .flat_map(|type_alias| type_alias.type_ann.type_ref_names())
            .collect::<HashSet<_>>();

        let relevant_type_params = self
            .container
            .generics()
            .type_params()
            .into_iter()
            .map(|p| p.ident.to_string())
            .filter(|t| type_ref_names.contains(t))
            .collect::<Vec<_>>();

        Decl::TsEnum(TsEnumDecl {
            id: self.container.name(),
            type_params: relevant_type_params,
            body,
        })
    }

    fn create_type_alias_decl(&self, type_ann: TsType) -> Decl {
        let type_ref_names = type_ann.type_ref_names();

        let relevant_type_params = self
            .container
            .generics()
            .type_params()
            .into_iter()
            .map(|p| p.ident.to_string())
            .filter(|t| type_ref_names.contains(t))
            .collect::<Vec<_>>();

        Decl::TsTypeAlias(TsTypeAliasDecl {
            id: self.container.name(),
            type_params: relevant_type_params,
            type_ann,
        })
    }

    fn create_interface_decl(&self, members: Vec<TsTypeElement>, extends: Vec<TsType>) -> Decl {
        let mut type_ref_names: HashSet<&String> = HashSet::new();
        members.iter().for_each(|member| {
            type_ref_names.extend(member.type_ann.type_ref_names());
        });
        extends.iter().for_each(|ty| {
            type_ref_names.extend(ty.type_ref_names());
        });

        let relevant_type_params = self
            .container
            .generics()
            .type_params()
            .into_iter()
            .map(|p| p.ident.to_string())
            .filter(|t| type_ref_names.contains(t))
            .collect::<Vec<_>>();

        Decl::TsInterface(TsInterfaceDecl {
            id: self.container.name(),
            type_params: relevant_type_params,
            extends,
            body: members,
        })
    }

    fn parse_struct(&self, style: Style, fields: &Vec<Field>) -> Decl {
        let parsed_fields = self.parse_fields(style, fields);
        let tag_type = self.container.serde_attrs().tag();

        match (tag_type, parsed_fields) {
            (TagType::Internal { tag }, ParsedFields::Named(members, extends)) => {
                let name = self.container.name();

                let tag_field = TsTypeElement {
                    key: tag.clone(),
                    type_ann: TsType::Lit(name),
                    optional: false,
                };

                let mut vec = Vec::with_capacity(members.len() + 1);
                vec.push(tag_field);
                vec.extend(members);

                self.create_interface_decl(vec, extends)
            }
            (_, ParsedFields::Named(members, extends)) => {
                self.create_interface_decl(members, extends)
            }
            (_, parsed_fields) => self.create_type_alias_decl(parsed_fields.into()),
        }
    }

    fn parse_fields(&self, style: Style, fields: &Vec<Field>) -> ParsedFields {
        let style = match style {
            Style::Struct => FieldsStyle::Named,
            Style::Newtype => return ParsedFields::Transparent(self.parse_field(&fields[0]).0),
            Style::Tuple => FieldsStyle::Unnamed,
            Style::Unit => return ParsedFields::Transparent(TsType::NULL),
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
            return ParsedFields::Transparent(self.parse_field(&fields[0]).0);
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
                self.container.darling_error(err);
                return (TsType::NEVER, None);
            }
        };

        if let Some(t) = &ts_attrs.type_override {
            return (TsType::Override(t.clone()), Some(ts_attrs));
        }

        let type_ann = TsType::from(field.ty);

        let type_ann = if ts_attrs.optional {
            match type_ann {
                TsType::Option(t) => *t,
                _ => type_ann,
            }
        } else {
            type_ann
        };

        (type_ann, Some(ts_attrs))
    }

    fn parse_named_fields(&self, fields: Vec<&Field>) -> (Vec<TsTypeElement>, Vec<TsType>) {
        let (flatten_fields, members): (Vec<_>, Vec<_>) =
            fields.into_iter().partition(|field| field.attrs.flatten());

        let members = members
            .into_iter()
            .map(|field| {
                let key = field.attrs.name().serialize_name();
                let (type_ann, field_attrs) = self.parse_field(field);

                let optional = !self.container.serde_attrs().default().is_none()
                    || field_attrs.map_or(false, |attrs| attrs.optional);

                TsTypeElement {
                    key,
                    type_ann,
                    optional,
                }
            })
            .collect();

        let flatten_fields = flatten_fields
            .into_iter()
            .map(|field| self.parse_field(field).0)
            .collect();

        (members, flatten_fields)
    }

    fn parse_enum(&self, variants: &Vec<Variant>) -> Decl {
        let type_alias = variants
            .iter()
            .filter(|v| !v.attrs.skip_serializing() && !v.attrs.skip_deserializing())
            .map(|variant| {
                let decl = self.create_type_alias_decl(self.parse_variant(variant));
                if let Decl::TsTypeAlias(mut type_alias) = decl {
                    type_alias.id = variant.attrs.name().serialize_name();

                    type_alias
                } else {
                    panic!();
                }
            })
            .collect::<Vec<_>>();

        self.create_enum_decl(type_alias)
    }

    fn parse_variant(&self, variant: &Variant) -> TsType {
        let tag_type = self.container.serde_attrs().tag();
        let name = variant.attrs.name().serialize_name();
        let style = variant.style;
        let type_ann: TsType = self.parse_fields(style, &variant.fields).into();
        type_ann.with_tag_type(name, style, tag_type)
    }
}

fn is_phantom(ty: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath { path, .. }) = ty {
        path.segments
            .last()
            .map_or(false, |path| path.ident == "PhantomData")
    } else {
        false
    }
}
