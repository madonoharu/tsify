use std::mem::ManuallyDrop;

use crate::Tsify;
use wasm_bindgen::convert::{
    FromWasmAbi, IntoWasmAbi, LongRefFromWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi,
    RefFromWasmAbi, VectorFromWasmAbi, VectorIntoWasmAbi,
};
use wasm_bindgen::describe::WasmDescribe;
use wasm_bindgen::describe::WasmDescribeVector;
use wasm_bindgen::{JsCast, JsValue};

/// A wrapper for a type implementing [`Tsify`], to be used in `#[wasm_bindgen]` function
/// parameters and return types.
///
/// Conceptually, this is just a [`JsValue`] that has easy conversion methods. The reason it exists
/// is that the `#[tsify(into_wasm_abi, from_wasm_abi)]` attributes cause memory
/// leaks when (de)serialization fails. This approach is less automatic but doesn't leak memory.
///
/// Exposes two main methods: [`ts_value.to_rust()`][Ts::to_rust] and [`Ts::from_rust`] to convert.
/// However for the latter you can also use [Tsify::into_ts] as `t.into_ts()`.
///
/// This can (and should) be used with the [`-> Result<_, JsError>`][wasm_bindgen::JsError]
/// pattern from wasm-bindgen to automatically throw any Err value returned. The point of this type
/// is robustness to serialization errors, especially when JavaScript (being a dynamic language)
/// could call your functions with any kind of data at runtime. Panicking defeats this purpose.
///
/// ## Example
///
/// ```
/// use tsify::Tsify;
/// use tsify::Ts;
/// use wasm_bindgen::prelude::*;
/// use wasm_bindgen::JsError;
///
/// #[derive(Tsify, serde::Deserialize, serde::Serialize)]
/// pub struct Vec2 {
///    x: f64,
///    y: f64,
/// }
///
/// /// The panicking version
/// #[wasm_bindgen]
/// pub fn rotate(v: Ts<Vec2>, theta_rad: f64) -> Result<Ts<Vec2>, JsError> {
///     // Deserialize to rust type, throw deserialization error if fails
///     let Vec2 { x, y } = v.to_rust()?;
///     // Do some maths
///     let cos = theta_rad.cos();
///     let sin = theta_rad.sin();
///     let result = Vec2 {
///         x: x * cos - y * sin,
///         y: x * sin + y * cos,
///     };
///     // Serialize back to JsValue, throw serialization error if fails
///     Ok(result.into_ts()?)
/// }
/// ```
///
#[repr(transparent)]
pub struct Ts<T: Tsify>(<T as Tsify>::JsType, std::marker::PhantomData<T>);

impl<T> Ts<T>
where
    T: Tsify,
{
    // Private
    fn new(js: <T as Tsify>::JsType) -> Self {
        Self(js, std::marker::PhantomData)
    }

    /// Returns the inner JsValue representation. This is a zero cost operation.
    pub fn js_value(&self) -> JsValue
    where
        <T as Tsify>::JsType: JsCast,
    {
        self.0.unchecked_ref::<JsValue>().clone()
    }

    /// Reinterpret a [`JsValue`] as a [`Ts<T>`] without any checks. This is a zero cost operation.
    ///
    /// If you get this wrong, the worst that can happen is that it will fail to deserialize
    /// when you call [`to_rust()`][Self::to_rust] on it.
    ///
    /// ```
    /// use tsify::Tsify;
    /// use tsify::Ts;
    /// use wasm_bindgen::prelude::*;
    ///
    /// #[derive(tsify::Tsify, serde::Deserialize)]
    /// struct Point {
    ///     x: f64,
    ///     y: f64,
    /// }
    ///
    /// fn some_api(_closure: impl Fn(JsValue)) { /* ... */ }
    ///
    /// some_api(|js_value: JsValue| {
    ///     let ts_value: Ts<Point> = Ts::new_unchecked(js_value);
    ///     let point = ts_value.to_rust().unwrap();
    /// });
    /// ```
    ///
    pub fn new_unchecked(js: JsValue) -> Self {
        Self::new(js.unchecked_into())
    }
}

impl<T> Clone for Ts<T>
where
    T: Tsify,
    <T as Tsify>::JsType: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), std::marker::PhantomData)
    }
}

impl<T> WasmDescribe for Ts<T>
where
    T: Tsify,
    <T as Tsify>::JsType: WasmDescribe,
{
    fn describe() {
        <T as Tsify>::JsType::describe()
    }
}

impl<T> IntoWasmAbi for Ts<T>
where
    T: Tsify,
    <T as Tsify>::JsType: IntoWasmAbi,
{
    type Abi = <T::JsType as IntoWasmAbi>::Abi;
    fn into_abi(self) -> Self::Abi {
        self.0.into_abi()
    }
}
impl<T> IntoWasmAbi for &Ts<T>
where
    T: Tsify,
    <T as Tsify>::JsType: IntoWasmAbi + Clone,
{
    type Abi = <T::JsType as IntoWasmAbi>::Abi;
    fn into_abi(self) -> Self::Abi {
        self.0.clone().into_abi()
    }
}
impl<T> FromWasmAbi for Ts<T>
where
    T: Tsify,
    <T as Tsify>::JsType: FromWasmAbi,
{
    type Abi = <T::JsType as FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        Self(<T as Tsify>::JsType::from_abi(js), std::marker::PhantomData)
    }
}

impl<T> OptionIntoWasmAbi for Ts<T>
where
    T: Tsify,
    <T as Tsify>::JsType: OptionIntoWasmAbi,
{
    fn none() -> Self::Abi {
        <T::JsType as OptionIntoWasmAbi>::none()
    }
}

impl<T> OptionFromWasmAbi for Ts<T>
where
    T: Tsify,
    <T as Tsify>::JsType: OptionFromWasmAbi,
{
    fn is_none(abi: &Self::Abi) -> bool {
        <T as Tsify>::JsType::is_none(abi)
    }
}

pub struct TsSelfOwner<T: Tsify>(Ts<T>);

impl<T: Tsify> ::core::ops::Deref for TsSelfOwner<T> {
    type Target = Ts<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> RefFromWasmAbi for Ts<T>
where
    T: Tsify,
    <T as Tsify>::JsType: RefFromWasmAbi,
{
    // JsValue uses ManuallyDrop.
    type Anchor = ManuallyDrop<Ts<T>>;
    type Abi = <JsValue as RefFromWasmAbi>::Abi; // i.e. u32
    unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
        let jstype_anchor = JsValue::ref_from_abi(js);
        let anchor_inner = ManuallyDrop::into_inner(jstype_anchor);
        let jstype: <T as Tsify>::JsType = anchor_inner.unchecked_into();
        ManuallyDrop::new(Ts::new(jstype))
    }
}

impl<T> LongRefFromWasmAbi for Ts<T>
where
    T: Tsify,
    <T as Tsify>::JsType: LongRefFromWasmAbi,
{
    type Abi = <JsValue as LongRefFromWasmAbi>::Abi;
    type Anchor = Ts<T>;
    unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
        let jsvalue = <JsValue as LongRefFromWasmAbi>::long_ref_from_abi(js);
        let jstype: <T as Tsify>::JsType = jsvalue.unchecked_into();
        Self::new(jstype)
    }
}

impl<T> WasmDescribeVector for Ts<T>
where
    T: Tsify,
    <T as Tsify>::JsType: WasmDescribeVector,
{
    fn describe_vector() {
        <T as Tsify>::JsType::describe_vector()
    }
}

impl<T> VectorFromWasmAbi for Ts<T>
where
    T: Tsify,
    <T as Tsify>::JsType: VectorFromWasmAbi,
{
    type Abi = <<T as Tsify>::JsType as VectorFromWasmAbi>::Abi;

    unsafe fn vector_from_abi(js: Self::Abi) -> Box<[Self]> {
        let vec_js = <<T as Tsify>::JsType as VectorFromWasmAbi>::vector_from_abi(js);
        let vec = Vec::from(vec_js);
        vec.into_iter()
            .map(|js_item| Ts(js_item, std::marker::PhantomData))
            .collect::<Vec<Ts<T>>>()
            .into_boxed_slice()
    }
}

impl<T> VectorIntoWasmAbi for Ts<T>
where
    T: Tsify,
    <T as Tsify>::JsType: VectorIntoWasmAbi,
{
    type Abi = <<T as Tsify>::JsType as VectorIntoWasmAbi>::Abi;

    fn vector_into_abi(vector: Box<[Self]>) -> Self::Abi {
        let vec_js: Vec<<T as Tsify>::JsType> = vector
            .into_vec()
            .into_iter()
            .map(|ts_item| ts_item.0)
            .collect();
        <T as Tsify>::JsType::vector_into_abi(vec_js.into_boxed_slice())
    }
}

impl<T: Tsify + serde::de::DeserializeOwned> Ts<T>
where
    <T as Tsify>::JsType: Clone,
{
    /// Converts the inner JSType (e.g. JsValue) into T
    pub fn to_rust(&self) -> Result<T, crate::Error> {
        T::from_js(self.0.clone())
    }
}

impl<T: Tsify + serde::Serialize> Ts<T> {
    /// Converts a rust type T into to the inner JSType (e.g. JsValue)
    pub fn from_rust(rust: &T) -> Result<Self, crate::Error> {
        Ok(Self(T::into_js(rust)?, std::marker::PhantomData))
    }
}
