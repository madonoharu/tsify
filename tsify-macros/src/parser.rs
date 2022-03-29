use serde_derive_internals::{
    ast::{Data, Field, Style, Variant},
    attr::TagType,
};

use crate::{
    attrs::TsifyFieldAttrs,
    container::Container,
    typescript::{TsType, TsTypeAliasDecl, TsTypeElement, TsTypeLit},
};

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

    pub fn parse(&self) -> TsTypeAliasDecl {
        let cont = &self.container;
        let type_ann = match cont.serde_data() {
            Data::Struct(style, ref fields) => self.parse_struct(*style, fields),
            Data::Enum(ref variants) => self.parse_enum(variants),
        };

        let type_ref_names = type_ann.type_ref_names();

        let relevant_type_params = cont
            .generics()
            .type_params()
            .into_iter()
            .map(|p| p.ident.to_string())
            .filter(|t| type_ref_names.contains(t))
            .collect::<Vec<_>>();

        TsTypeAliasDecl {
            id: cont.name(),
            type_params: relevant_type_params,
            type_ann,
        }
    }

    fn parse_struct(&self, style: Style, fields: &Vec<Field>) -> TsType {
        let type_ann = self.parse_fields(style, fields);
        let tag_type = self.container.serde_attrs().tag();
        let name = self.container.ident().to_string();

        match tag_type {
            TagType::Internal { .. } => type_ann.with_tag_type(name, style, tag_type),
            _ => type_ann,
        }
    }

    fn parse_fields(&self, style: Style, fields: &Vec<Field>) -> TsType {
        match style {
            Style::Struct => self.parse_struct_or_tuple(FieldsStyle::Named, fields),
            Style::Newtype => self.parse_field(&fields[0]).0,
            Style::Tuple => self.parse_struct_or_tuple(FieldsStyle::Unnamed, fields),
            Style::Unit => TsType::NULL,
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

    fn parse_struct_or_tuple(&self, style: FieldsStyle, fields: &Vec<Field>) -> TsType {
        let fields = fields
            .iter()
            .filter(|field| {
                !field.attrs.skip_serializing()
                    && !field.attrs.skip_deserializing()
                    && !is_phantom(field.ty)
            })
            .collect::<Vec<_>>();

        if fields.len() == 1 && self.container.transparent() {
            return self.parse_field(&fields[0]).0;
        }

        match style {
            FieldsStyle::Named => {
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

                let type_lit = TsType::from(TsTypeLit { members });

                if flatten_fields.is_empty() {
                    type_lit
                } else {
                    let flatten_fields = flatten_fields
                        .into_iter()
                        .map(|field| self.parse_field(field).0)
                        .collect();
                    type_lit.and(TsType::Intersection(flatten_fields))
                }
            }
            FieldsStyle::Unnamed => {
                let elems = fields
                    .into_iter()
                    .map(|field| self.parse_field(field).0)
                    .collect();

                TsType::Tuple(elems)
            }
        }
    }

    fn parse_enum(&self, variants: &Vec<Variant>) -> TsType {
        let variants = variants
            .iter()
            .filter(|v| !v.attrs.skip_serializing() && !v.attrs.skip_deserializing())
            .map(|variant| self.parse_variant(variant))
            .collect::<Vec<_>>();

        TsType::Union(variants)
    }

    fn parse_variant(&self, variant: &Variant) -> TsType {
        let tag_type = self.container.serde_attrs().tag();
        let name = variant.attrs.name().serialize_name();
        let style = variant.style;
        let type_ann = self.parse_fields(style, &variant.fields);
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
