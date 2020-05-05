//! Button/SimpleButton prototype

use crate::avm1::globals::display_object;
use crate::avm1::return_value::ReturnValue;
use crate::avm1::{Avm1, Error, Object, ScriptObject, UpdateContext, Value};
use gc_arena::MutationContext;
use crate::backend::Backends;

pub fn create_proto<'gc, B: Backends>(
    gc_context: MutationContext<'gc, '_>,
    proto: Object<'gc, B>,
    fn_proto: Object<'gc, B>,
) -> Object<'gc, B> {
    let object = ScriptObject::object(gc_context, Some(proto));

    display_object::define_display_object_proto(gc_context, object, fn_proto);

    object.into()
}

/// Implements `Button` constructor.
pub fn constructor<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    _action_context: &mut UpdateContext<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    Ok(Value::Undefined.into())
}
