use std::fmt::Display;

use crate::{attrs::TypeGenerationConfig, comments::write_doc_comments};

use super::{ToStringWithIndent, TsType};

/// Built-in TypeScript types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TsKeywordTypeKind {
    /// The `number` type.
    Number,
    /// The `bigint` type.
    Bigint,
    /// The `boolean` type.
    Boolean,
    /// The `string` type.
    String,
    /// The `void` type.
    Void,
    /// The `undefined` type.
    Undefined,
    /// The `null` type.
    Null,
    /// The `never` type.
    Never,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TsTypeElementKey {
    /// Types members like `x: number` or "x_x": string`
    Lit(String),
    /// Types members like `[MyConstants.x]: number` or `"[x]": string`
    Var(String),
}

impl From<TsTypeElementKey> for TsType {
    fn from(value: TsTypeElementKey) -> Self {
        match value {
            TsTypeElementKey::Lit(s) => TsType::Lit(s),
            TsTypeElementKey::Var(v) => TsType::Computed(v),
        }
    }
}

impl From<String> for TsTypeElementKey {
    fn from(value: String) -> Self {
        TsTypeElementKey::Lit(value)
    }
}

fn is_js_ident(string: &str) -> bool {
    !string.is_empty()
        && !string.starts_with(|c: char| c.is_ascii_digit())
        && !string.contains(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '$')
}

impl Display for TsTypeElementKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TsTypeElementKey::Lit(key) => {
                if is_js_ident(key) {
                    write!(f, "{key}")
                } else {
                    write!(f, "\"{key}\"")
                }
            }
            TsTypeElementKey::Var(key) => write!(f, "[{}]", key),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TsTypeElement {
    pub key: TsTypeElementKey,
    pub type_ann: TsType,
    pub optional: bool,
    pub comments: Vec<String>,
}

impl Display for TsTypeElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = &self.key;
        let optional_ann = if self.optional { "?" } else { "" };
        let type_ann = &self.type_ann;

        write_doc_comments(f, &self.comments)?;
        write!(f, "{}{}: {}", key, optional_ann, type_ann)
    }
}

impl From<TsTypeElement> for TsTypeLit {
    fn from(m: TsTypeElement) -> Self {
        TsTypeLit { members: vec![m] }
    }
}

impl From<&[TsTypeElement]> for TsTypeLit {
    fn from(m: &[TsTypeElement]) -> Self {
        TsTypeLit {
            members: m.to_vec(),
        }
    }
}

impl From<TsTypeElement> for TsType {
    fn from(m: TsTypeElement) -> Self {
        TsType::TypeLit(m.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TsTypeLit {
    pub members: Vec<TsTypeElement>,
}

impl TsTypeLit {
    pub fn get_mut(&mut self, key: &TsTypeElementKey) -> Option<&mut TsTypeElement> {
        self.members.iter_mut().find(|member| member.key == *key)
    }

    pub fn and(self, other: Self) -> Self {
        let init = TsTypeLit { members: vec![] };

        self.members
            .into_iter()
            .chain(other.members)
            .fold(init, |mut acc, m| {
                if let Some(acc_m) = acc.get_mut(&m.key) {
                    let mut tmp = TsType::NULL;
                    std::mem::swap(&mut acc_m.type_ann, &mut tmp);
                    acc_m.type_ann = tmp.and(m.type_ann);
                } else {
                    acc.members.push(m)
                }

                acc
            })
    }
}

impl From<TsTypeLit> for TsType {
    fn from(lit: TsTypeLit) -> Self {
        TsType::TypeLit(lit)
    }
}

impl Display for TsTypeLit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.members.is_empty() {
            write!(f, "{{}}")
        } else {
            let members = self
                .members
                .iter()
                .map(|elem| format!("\n{};", elem.to_string_with_indent(4)))
                .collect::<Vec<_>>()
                .join("");

            write!(f, "{{{members}\n}}")
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TsValueEnumLit {
    None,
    StringLit(String),
    NumberLit(String),
}

impl Display for TsValueEnumLit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TsValueEnumLit::None => Ok(()),
            TsValueEnumLit::StringLit(value) => write!(f, " = \"{}\"", value),
            TsValueEnumLit::NumberLit(value) => write!(f, " = {}", value),
        }
    }
}

/// A member in a TypeScript enum, e.g. `Foo = 5,`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TsValueEnumMember {
    pub id: String,
    pub value: TsValueEnumLit,
    pub comments: Vec<String>,
}

impl Display for TsValueEnumMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_doc_comments(f, &self.comments)?;
        if is_js_ident(self.id.as_str()) {
            write!(f, "{}{}", self.id, self.value)
        } else {
            write!(f, "\"{}\"{}", self.id, self.value)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NullType {
    Null,
    Undefined,
}

impl NullType {
    pub const fn new(config: &TypeGenerationConfig) -> Self {
        if cfg!(feature = "js") && !config.missing_as_null {
            Self::Undefined
        } else {
            Self::Null
        }
    }

    pub const fn to_type(&self) -> TsType {
        match self {
            Self::Null => TsType::NULL,
            Self::Undefined => TsType::UNDEFINED,
        }
    }
}
