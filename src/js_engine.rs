//! Abstraction of the JS Engine.

use crate::error::Result;

/// A trait to represent a JS engine.
pub(crate) trait JsEngine: Sized {
    /// The type of a JS value.
    type JsValue<'a>: JsValue<'a>;

    /// Create a JS engine.
    fn new() -> Result<Self>;

    /// Evaluate arbitrary code in the JS engine.
    fn eval<'a>(&'a self, code: &str) -> Result<Self::JsValue<'a>>;

    /// Call a JS function in the JS engine.
    fn call_function<'a>(
        &'a self,
        func_name: &str,
        args: impl Iterator<Item = Self::JsValue<'a>>,
    ) -> Result<Self::JsValue<'a>>;

    /// Create a JS value `null`.
    fn null<'a>(&'a self) -> Self::JsValue<'a>;

    /// Create a JS value from [`bool`].
    fn from_bool<'a>(&'a self, input: bool) -> Self::JsValue<'a>;

    /// Create a JS value from [`i32`].
    fn from_int<'a>(&'a self, input: i32) -> Self::JsValue<'a>;

    /// Create a JS value from [`f64`].
    fn from_float<'a>(&'a self, input: f64) -> Self::JsValue<'a>;
    
    /// Create a JS value from [`String`].
    fn from_string<'a>(&'a self, input: String) -> Self::JsValue<'a>;

    /// Create a JS array value from an iterator for `Self`.
    fn from_array<'a>(&'a self, input: impl Iterator<Item = Self::JsValue<'a>>) -> Self::JsValue<'a>;
    
    /// Create a JS object value from an iterator for `(String, Self)`.
    fn from_object<'a>(&'a self, input: impl Iterator<Item = (String, Self::JsValue<'a>)>) -> Self::JsValue<'a>;
}

/// A trait to represent a JS value.
pub(crate) trait JsValue<'a>: Sized + Clone {
    /// Check whether the JS value is `null`.
    fn is_null(&self) -> bool;
    /// Check whether the JS value is a [`bool`].
    fn is_bool(&self) -> bool;
    /// Check whether the JS value is a [`i32`].
    fn is_int(&self) -> bool;
    /// Check whether the JS value is a [`f64`].
    fn is_float(&self) -> bool;
    /// Check whether the JS value is a [`String`].
    fn is_string(&self) -> bool;

    /// Convert the JS Value to a [`bool`].
    fn into_bool(self) -> Result<bool>;
    /// Convert the JS Value to a [`i32`].
    fn into_int(self) -> Result<i32>;
    /// Convert the JS Value to a [`f64`].
    fn into_float(self) -> Result<f64>;
    /// Convert the JS Value to a [`String`].
    fn into_string(self) -> Result<String>;
}

cfg_if::cfg_if! {
    if #[cfg(feature = "quick-js")] {
        mod quick_js;

        pub(crate) type Engine = quick_js::Engine;
    } else {
        compile_error!("Must enable one of the JS engines.");
    }
}
