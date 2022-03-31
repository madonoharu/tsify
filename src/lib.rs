pub use tsify_macros::{tsify, Tsify};

pub trait Tsify {
    const DECL: &'static str;
}
