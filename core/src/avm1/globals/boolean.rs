//! `Boolean` class impl

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::function::{Executable, FunctionObject};
use crate::avm1::object::value_object::ValueObject;
use crate::avm1::{AvmString, Object, TObject, Value};
use enumset::EnumSet;
use gc_arena::MutationContext;

/// `Boolean` constructor/function
pub fn boolean<'gc>(
    activation: &mut Activation<'_, '_, 'gc, '_>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let (ret_value, cons_value) = if let Some(val) = args.get(0) {
        let b = Value::Bool(val.as_bool(activation.current_swf_version()));
        (b.clone(), b)
    } else {
        (Value::Undefined, Value::Bool(false))
    };

    // If called from a constructor, populate `this`.
    if let Some(mut vbox) = this.as_value_object() {
        vbox.replace_value(activation.context.gc_context, cons_value);
    }

    // If called as a function, return the value.
    // Boolean() with no argument returns undefined.
    Ok(ret_value)
}

pub fn create_boolean_object<'gc>(
    gc_context: MutationContext<'gc, '_>,
    boolean_proto: Option<Object<'gc>>,
    fn_proto: Option<Object<'gc>>,
) -> Object<'gc> {
    FunctionObject::function(
        gc_context,
        Executable::Native(boolean),
        fn_proto,
        boolean_proto,
    )
}

/// Creates `Boolean.prototype`.
pub fn create_proto<'gc>(
    gc_context: MutationContext<'gc, '_>,
    proto: Object<'gc>,
    fn_proto: Object<'gc>,
) -> Object<'gc> {
    let boolean_proto = ValueObject::empty_box(gc_context, Some(proto));
    let mut object = boolean_proto.as_script_object().unwrap();

    object.force_set_function(
        "toString",
        to_string,
        gc_context,
        EnumSet::empty(),
        Some(fn_proto),
    );
    object.force_set_function(
        "valueOf",
        value_of,
        gc_context,
        EnumSet::empty(),
        Some(fn_proto),
    );

    boolean_proto
}

pub fn to_string<'gc>(
    activation: &mut Activation<'_, '_, 'gc, '_>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(vbox) = this.as_value_object() {
        // Must be a bool.
        // Boolean.prototype.toString.call(x) returns undefined for non-bools.
        if let Value::Bool(b) = vbox.unbox() {
            return Ok(AvmString::new(activation.context.gc_context, b.to_string()).into());
        }
    }

    Ok(Value::Undefined)
}

pub fn value_of<'gc>(
    _activation: &mut Activation<'_, '_, 'gc, '_>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(vbox) = this.as_value_object() {
        // Must be a bool.
        // Boolean.prototype.valueOf.call(x) returns undefined for non-bools.
        if let Value::Bool(b) = vbox.unbox() {
            return Ok(b.into());
        }
    }

    Ok(Value::Undefined)
}
