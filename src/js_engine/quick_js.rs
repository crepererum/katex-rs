//! JS Engine implemented by [QuickJs](https://crates.io/crates/quick-js).

use core::convert::TryInto;

use crate::{
    error::{Error, Result},
    js_engine::{JsEngine, JsValue},
};

/// QuickJS Engine.
pub struct Engine(quick_js::Context);

impl JsEngine for Engine {
    type JsValue<'a> = Value;

    fn new() -> Result<Self> {
        Ok(Self(quick_js::Context::new()?))
    }

    fn eval<'a>(&'a self, code: &str) -> Result<Self::JsValue<'a>> {
        Ok(Value(self.0.eval(code)?))
    }

    fn call_function<'a>(
        &'a self,
        func_name: &str,
        args: impl Iterator<Item = Self::JsValue<'a>>,
    ) -> Result<Self::JsValue<'a>> {
        Ok(Value(self.0.call_function(func_name, args.map(|v| v.0))?))
    }

    fn null<'a>(&'a self) -> Self::JsValue<'a> {
        Value(quick_js::JsValue::Null)
    }

    fn from_bool<'a>(&'a self, input: bool) -> Self::JsValue<'a> {
        Value(quick_js::JsValue::Bool(input))
    }

    fn from_int<'a>(&'a self, input: i32) -> Self::JsValue<'a> {
        Value(quick_js::JsValue::Int(input))
    }

    fn from_float<'a>(&'a self, input: f64) -> Self::JsValue<'a> {
        Value(quick_js::JsValue::Float(input))
    }
    
    fn from_string<'a>(&'a self, input: String) -> Self::JsValue<'a> {
        Value(quick_js::JsValue::String(input))
    }

    fn from_array<'a>(&'a self, input: impl Iterator<Item = Self::JsValue<'a>>) -> Self::JsValue<'a> {
        let array = input.into_iter().map(|v| v.0).collect();
        Value(quick_js::JsValue::Array(array))
    }
    
    fn from_object<'a>(&'a self, input: impl Iterator<Item = (String, Self::JsValue<'a>)>) -> Self::JsValue<'a> {
        let obj = input.into_iter().map(|(k, v)| (k, v.0)).collect();
        Value(quick_js::JsValue::Object(obj))
    }
}

/// QuickJS Value.
#[derive(Debug, Clone)]
pub struct Value(quick_js::JsValue);

impl<'a> JsValue<'a> for Value {
    fn is_null(&self) -> bool {
        matches!(self.0, quick_js::JsValue::Null)
    }

    fn is_bool(&self) -> bool {
        matches!(self.0, quick_js::JsValue::Bool(_))
    }

    fn is_int(&self) -> bool {
        matches!(self.0, quick_js::JsValue::Int(_))
    }

    fn is_float(&self) -> bool {
        matches!(self.0, quick_js::JsValue::Float(_))
    }

    fn is_string(&self) -> bool {
        matches!(self.0, quick_js::JsValue::String(_))
    }

    fn into_bool(self) -> Result<bool> {
        Ok(self.0.try_into()?)
    }

    fn into_int(self) -> Result<i32> {
        Ok(self.0.try_into()?)
    }

    fn into_float(self) -> Result<f64> {
        Ok(self.0.try_into()?)
    }

    fn into_string(self) -> Result<String> {
        Ok(self.0.try_into()?)
    }
}

impl From<quick_js::ContextError> for Error {
    fn from(e: quick_js::ContextError) -> Self {
        Self::JsInitError(format!("{}", e))
    }
}

impl From<quick_js::ExecutionError> for Error {
    fn from(e: quick_js::ExecutionError) -> Self {
        Self::JsExecError(format!("{}", e))
    }
}

impl From<quick_js::ValueError> for Error {
    fn from(e: quick_js::ValueError) -> Self {
        Self::JsValueError(format!("{}", e))
    }
}
