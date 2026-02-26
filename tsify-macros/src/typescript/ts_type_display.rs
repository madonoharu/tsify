use std::fmt::Display;

use super::TsType;

impl Display for TsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TsType::Keyword(kind) => {
                let ty = format!("{:?}", kind).to_lowercase();
                write!(f, "{ty}")
            }

            TsType::Lit(lit) => {
                write!(f, "\"{lit}\"")
            }

            TsType::Computed(comp) => {
                write!(f, "{comp}")
            }

            TsType::Array(elem) => match elem.as_ref() {
                TsType::Union(_) | TsType::Intersection(_) | &TsType::Option(_, _) => {
                    write!(f, "({elem})[]")
                }
                _ => write!(f, "{elem}[]"),
            },

            TsType::Tuple(elems) => {
                let elems = elems
                    .iter()
                    .map(|elem| elem.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "[{elems}]")
            }

            TsType::Ref { name, type_params } => {
                let params = type_params
                    .iter()
                    .map(|param| param.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                if params.is_empty() {
                    write!(f, "{name}")
                } else {
                    write!(f, "{name}<{params}>")
                }
            }

            TsType::Fn { params, type_ann } => {
                let params = params
                    .iter()
                    .enumerate()
                    .map(|(i, param)| format!("arg{i}: {param}"))
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "({params}) => {type_ann}")
            }

            TsType::Option(elem, null) => {
                write!(f, "{elem} | {}", null.to_type())
            }

            TsType::TypeLit(type_lit) => {
                write!(f, "{type_lit}")
            }

            TsType::Intersection(types) => {
                if types.len() == 1 {
                    let ty = &types[0];
                    return write!(f, "{ty}");
                }

                let types = types
                    .iter()
                    .map(|ty| match ty {
                        TsType::Union(_) => format!("({ty})"),
                        TsType::TypeLit(tl) => {
                            // Intersections are formatted as single lines, so we need to remove
                            // any comments as they are multi-line and will break the formatting.
                            let mut copy = tl.clone();
                            copy.members.iter_mut().for_each(|elem| {
                                elem.comments = vec![];
                            });
                            copy.to_string()
                        }
                        _ => ty.to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(" & ");

                write!(f, "{types}")
            }

            TsType::Union(types) => match types.len() {
                0 => {
                    write!(f, "void")
                }
                1 => {
                    let ty = &types[0];
                    write!(f, "{ty}")
                }
                _ => {
                    let types = types
                        .iter()
                        .map(|ty| match ty {
                            TsType::Intersection(_) => format!("({ty})"),
                            _ => ty.to_string(),
                        })
                        .collect::<Vec<_>>()
                        .join(" | ");

                    write!(f, "{types}")
                }
            },

            TsType::Override { type_override, .. } => f.write_str(type_override),
        }
    }
}
