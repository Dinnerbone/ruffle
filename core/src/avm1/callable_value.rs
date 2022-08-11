use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::{AvmString, Object, TObject, Value};
use gc_arena::Collect;
use ruffle_types::backend::Backend;

#[derive(Clone, Collect, Debug)]
#[collect(no_drop)]
pub enum CallableValue<'gc, B: Backend> {
    UnCallable(Value<'gc, B>),
    Callable(Object<'gc, B>, Value<'gc, B>),
}

impl<'gc, B: Backend> From<CallableValue<'gc, B>> for Value<'gc, B> {
    fn from(c: CallableValue<'gc, B>) -> Self {
        match c {
            CallableValue::UnCallable(v) => v,
            CallableValue::Callable(_, v) => v,
        }
    }
}

impl<'gc, B: Backend> CallableValue<'gc, B> {
    pub fn call_with_default_this(
        self,
        default_this: Object<'gc, B>,
        name: AvmString<'gc>,
        activation: &mut Activation<'_, 'gc, '_, B>,
        args: &[Value<'gc, B>],
    ) -> Result<Value<'gc, B>, Error<'gc, B>> {
        match self {
            CallableValue::Callable(this, Value::Object(val)) => {
                val.call(name, activation, this.into(), args)
            }
            CallableValue::UnCallable(Value::Object(val)) => {
                val.call(name, activation, default_this.into(), args)
            }
            _ => Ok(Value::Undefined),
        }
    }
}
