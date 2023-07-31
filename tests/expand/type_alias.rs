#[tsify::declare]
type TypeAlias<T, U> = Foo<T, i32, U>;

#[tsify::declare(type_prefix = "Special")]
type PrefixedTypeAlias<T, U> = Foo<T, i32, U>;