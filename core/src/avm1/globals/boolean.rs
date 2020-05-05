//! `Boolean` class impl

use crate::avm1::function::{Executable, FunctionObject};
use crate::avm1::return_value::ReturnValue;
use crate::avm1::value_object::ValueObject;
use crate::avm1::{Avm1, Error, Object, TObject, Value};
use crate::context::UpdateContext;
use enumset::EnumSet;
use gc_arena::MutationContext;
use crate::backend::Backends;

/// `Boolean` constructor/function
pub fn boolean<'gc, B: Backends>(
    avm: &mut Avm1<'gc, B>,
    context: &mut UpdateContext<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    let (ret_value, cons_value) = if let Some(val) = args.get(0) {
        let b = Value::Bool(val.as_bool(avm.current_swf_version()));
        (b.clone(), b)
    } else {
        (Value::Undefined, Value::Bool(false))
    };

    // If called from a constructor, populate `this`.
    if let Some(mut vbox) = this.as_value_object() {
        vbox.replace_value(context.gc_context, cons_value);
    }

    // If called as a function, return the value.
    // Boolean() with no argument returns undefined.
    Ok(ret_value.into())
}

pub fn create_boolean_object<'gc, B: Backends>(
    gc_context: MutationContext<'gc, '_>,
    boolean_proto: Option<Object<'gc, B>>,
    fn_proto: Option<Object<'gc, B>>,
) -> Object<'gc, B> {
    FunctionObject::function(
        gc_context,
        Executable::Native(boolean),
        fn_proto,
        boolean_proto,
    )
}

/// Creates `Boolean.prototype`.
pub fn create_proto<'gc, B: Backends>(
    gc_context: MutationContext<'gc, '_>,
    proto: Object<'gc, B>,
    fn_proto: Object<'gc, B>,
) -> Object<'gc, B> {
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

pub fn to_string<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    _context: &mut UpdateContext<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    if let Some(vbox) = this.as_value_object() {
        // Must be a bool.
        // Boolean.prototype.toString.call(x) returns undefined for non-bools.
        if let Value::Bool(b) = vbox.unbox() {
            return Ok(b.to_string().into());
        }
    }

    Ok(Value::Undefined.into())
}

pub fn value_of<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    _context: &mut UpdateContext<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    if let Some(vbox) = this.as_value_object() {
        // Must be a bool.
        // Boolean.prototype.valueOf.call(x) returns undefined for non-bools.
        if let Value::Bool(b) = vbox.unbox() {
            return Ok(b.into());
        }
    }

    Ok(Value::Undefined.into())
}
