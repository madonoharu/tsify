pub use tsify_macros::Tsify;

pub trait Tsify {
    const DECL: &'static str;
}
