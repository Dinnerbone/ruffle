//! Video class

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::object::Object;
use crate::avm1::value::Value;
use crate::avm1::ScriptObject;
use gc_arena::MutationContext;
use ruffle_types::backend::Backend;

/// Implements `Video`
pub fn constructor<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    Ok(Value::Undefined)
}

pub fn create_proto<'gc, B: Backend>(
    gc_context: MutationContext<'gc, '_>,
    proto: Object<'gc, B>,
    _fn_proto: Object<'gc, B>,
) -> Object<'gc, B> {
    let object = ScriptObject::object(gc_context, Some(proto));
    object.into()
}
