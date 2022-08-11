use crate::avm1::activation::{
    Activation as Avm1Activation, ActivationIdentifier as Avm1ActivationIdentifier,
};
use crate::avm1::object::TObject as _;
use crate::avm1::Value as Avm1Value;
use crate::avm1::{
    ArrayObject as Avm1ArrayObject, Error as Avm1Error, Object as Avm1Object,
    ScriptObject as Avm1ScriptObject,
};
use crate::avm2::activation::Activation as Avm2Activation;
use crate::avm2::object::TObject as _;
use crate::avm2::Value as Avm2Value;
use crate::avm2::{ArrayObject as Avm2ArrayObject, Error as Avm2Error, Object as Avm2Object};

use crate::context::UpdateContext;
use gc_arena::Collect;
use ruffle_types::backend::Backend;
use ruffle_types::string::AvmString;
use std::collections::BTreeMap;

/// An intermediate format of representing shared data between ActionScript and elsewhere.
/// Regardless of the capabilities of both sides, all data will be translated to this potentially
/// lossy format. Any recursion or additional metadata in ActionScript will not be translated.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Object(BTreeMap<String, Value>),
    List(Vec<Value>),
}

impl From<AvmString<'_>> for Value {
    fn from(string: AvmString<'_>) -> Self {
        Value::String(string.to_string())
    }
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Value::String(string)
    }
}

impl From<&'static str> for Value {
    fn from(string: &'static str) -> Self {
        Value::String(string.into())
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Number(f64::from(value))
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::Number(f64::from(value))
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::Number(f64::from(value))
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::Number(f64::from(value))
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Number(f64::from(value))
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Number(value as f64)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::Number(f64::from(value))
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Value::Number(value as f64)
    }
}

impl From<BTreeMap<String, Value>> for Value {
    fn from(value: BTreeMap<String, Value>) -> Self {
        Value::Object(value)
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::List(value)
    }
}

impl Value {
    pub fn from_avm1<'gc, B: Backend>(
        activation: &mut Avm1Activation<'_, 'gc, '_, B>,
        value: Avm1Value<'gc, B>,
    ) -> Result<Value, Avm1Error<'gc, B>> {
        Ok(match value {
            Avm1Value::Undefined | Avm1Value::Null => Value::Null,
            Avm1Value::Bool(value) => value.into(),
            Avm1Value::Number(value) => value.into(),
            Avm1Value::String(value) => Value::String(value.to_string()),
            Avm1Value::Object(object) => {
                if object.as_array_object().is_some() {
                    let length = object.length(activation)?;
                    let values: Result<Vec<_>, Avm1Error<'gc, B>> = (0..length)
                        .map(|i| {
                            let element = object.get_element(activation, i);
                            Value::from_avm1(activation, element)
                        })
                        .collect();
                    Value::List(values?)
                } else {
                    let keys = object.get_keys(activation);
                    let mut values = BTreeMap::new();
                    for key in keys {
                        let value = object.get(key, activation)?;
                        values.insert(key.to_string(), Value::from_avm1(activation, value)?);
                    }
                    Value::Object(values)
                }
            }
        })
    }

    pub fn into_avm1<'gc, B: Backend>(
        self,
        activation: &mut Avm1Activation<'_, 'gc, '_, B>,
    ) -> Avm1Value<'gc, B> {
        match self {
            Value::Null => Avm1Value::Null,
            Value::Bool(value) => Avm1Value::Bool(value),
            Value::Number(value) => Avm1Value::Number(value),
            Value::String(value) => {
                Avm1Value::String(AvmString::new_utf8(activation.context.gc_context, value))
            }
            Value::Object(values) => {
                let object = Avm1ScriptObject::object(
                    activation.context.gc_context,
                    Some(activation.context.avm1.prototypes().object),
                );
                for (key, value) in values {
                    let key = AvmString::new_utf8(activation.context.gc_context, key);
                    let _ = object.set(key, value.into_avm1(activation), activation);
                }
                object.into()
            }
            Value::List(values) => Avm1ArrayObject::new(
                activation.context.gc_context,
                activation.context.avm1.prototypes().array,
                values
                    .iter()
                    .map(|value| value.to_owned().into_avm1(activation)),
            )
            .into(),
        }
    }

    pub fn from_avm2<'gc, B: Backend>(
        activation: &mut Avm2Activation<'_, 'gc, '_, B>,
        value: Avm2Value<'gc, B>,
    ) -> Result<Value, Avm2Error> {
        Ok(match value {
            Avm2Value::Undefined | Avm2Value::Null => Value::Null,
            Avm2Value::Bool(value) => value.into(),
            Avm2Value::Number(value) => value.into(),
            Avm2Value::Unsigned(value) => value.into(),
            Avm2Value::Integer(value) => value.into(),
            Avm2Value::String(value) => Value::String(value.to_string()),
            Avm2Value::Object(object) => {
                if let Some(array) = object.as_array_storage() {
                    let length = array.length();
                    let values: Result<Vec<_>, Avm2Error> = (0..length)
                        .map(|i| {
                            // FIXME - is this right?
                            let element = array.get(i).unwrap_or(Avm2Value::Null);
                            Value::from_avm2(activation, element)
                        })
                        .collect();
                    Value::List(values?)
                } else {
                    log::warn!("from_avm2 needs to be implemented for Avm2Value::Object");
                    Value::Null
                }
            }
        })
    }

    pub fn into_avm2<'gc, B: Backend>(
        self,
        activation: &mut Avm2Activation<'_, 'gc, '_, B>,
    ) -> Avm2Value<'gc, B> {
        match self {
            Value::Null => Avm2Value::Null,
            Value::Bool(value) => Avm2Value::Bool(value),
            Value::Number(value) => Avm2Value::Number(value),
            Value::String(value) => {
                Avm2Value::String(AvmString::new_utf8(activation.context.gc_context, value))
            }
            Value::Object(_values) => {
                log::warn!("into_avm2 needs to be implemented for Value::Object");
                Avm2Value::Undefined
            }
            Value::List(values) => {
                let storage = values
                    .iter()
                    .map(|value| value.to_owned().into_avm2(activation))
                    .collect();

                Avm2Value::Object(Avm2ArrayObject::from_storage(activation, storage).unwrap())
            }
        }
    }
}

#[derive(Collect, Clone)]
#[collect(no_drop)]
pub enum Callback<'gc, B: Backend> {
    Avm1 {
        this: Avm1Value<'gc, B>,
        method: Avm1Object<'gc, B>,
    },
    Avm2 {
        method: Avm2Object<'gc, B>,
    },
}

impl<'gc, B: Backend> Callback<'gc, B> {
    pub fn call(
        &self,
        context: &mut UpdateContext<'_, 'gc, '_, B>,
        name: &str,
        args: impl IntoIterator<Item = Value>,
    ) -> Value {
        match self {
            Callback::Avm1 { this, method } => {
                let base_clip = context.stage.root_clip();
                let globals = context.avm1.global_object_cell();
                let mut activation = Avm1Activation::from_nothing(
                    context.reborrow(),
                    Avm1ActivationIdentifier::root("[ExternalInterface]"),
                    globals,
                    base_clip,
                );
                let this = this.coerce_to_object(&mut activation);
                let args: Vec<Avm1Value<B>> = args
                    .into_iter()
                    .map(|v| v.into_avm1(&mut activation))
                    .collect();
                let name = AvmString::new_utf8(activation.context.gc_context, name);
                if let Ok(result) = method
                    .call(name, &mut activation, this.into(), &args)
                    .and_then(|value| Value::from_avm1(&mut activation, value))
                {
                    result
                } else {
                    Value::Null
                }
            }
            Callback::Avm2 { method } => {
                let mut activation = Avm2Activation::from_nothing(context.reborrow());
                let args: Vec<Avm2Value<B>> = args
                    .into_iter()
                    .map(|v| v.into_avm2(&mut activation))
                    .collect();
                if let Ok(result) = method
                    .call(None, &args, &mut activation)
                    .and_then(|value| Value::from_avm2(&mut activation, value))
                {
                    result
                } else {
                    Value::Null
                }
            }
        }
    }
}

pub trait ExternalInterfaceProvider<B: Backend> {
    fn get_method(&self, name: &str) -> Option<Box<dyn ExternalInterfaceMethod<B>>>;

    fn on_callback_available(&self, name: &str);

    fn on_fs_command(&self, command: &str, args: &str) -> bool;
}

pub trait ExternalInterfaceMethod<B: Backend> {
    fn call(&self, context: &mut UpdateContext<'_, '_, '_, B>, args: &[Value]) -> Value;
}

impl<F, B: Backend> ExternalInterfaceMethod<B> for F
where
    F: Fn(&mut UpdateContext<'_, '_, '_, B>, &[Value]) -> Value,
{
    fn call(&self, context: &mut UpdateContext<'_, '_, '_, B>, args: &[Value]) -> Value {
        self(context, args)
    }
}

#[derive(Default, Collect)]
#[collect(no_drop)]
pub struct ExternalInterface<'gc, B: Backend> {
    #[collect(require_static)]
    providers: Vec<Box<dyn ExternalInterfaceProvider<B>>>,
    callbacks: BTreeMap<String, Callback<'gc, B>>,
}

impl<'gc, B: Backend> ExternalInterface<'gc, B> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_provider(&mut self, provider: Box<dyn ExternalInterfaceProvider<B>>) {
        self.providers.push(provider);
    }

    pub fn add_callback(&mut self, name: String, callback: Callback<'gc, B>) {
        self.callbacks.insert(name.clone(), callback);
        for provider in &self.providers {
            provider.on_callback_available(&name);
        }
    }

    pub fn get_callback(&self, name: &str) -> Option<Callback<'gc, B>> {
        self.callbacks.get(name).cloned()
    }

    pub fn get_method_for(&self, name: &str) -> Option<Box<dyn ExternalInterfaceMethod<B>>> {
        for provider in &self.providers {
            if let Some(method) = provider.get_method(name) {
                return Some(method);
            }
        }
        None
    }

    pub fn available(&self) -> bool {
        !self.providers.is_empty()
    }

    pub fn invoke_fs_command(&self, command: &str, args: &str) -> bool {
        for provider in &self.providers {
            if provider.on_fs_command(command, args) {
                return true;
            }
        }
        false
    }
}
