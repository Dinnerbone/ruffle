//! `Boolean` class impl

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::function::{Executable, FunctionObject};
use crate::avm1::object::value_object::ValueObject;
use crate::avm1::property_decl::{define_properties_on, Declaration};
use crate::avm1::{Object, TObject, Value};
use gc_arena::MutationContext;
use ruffle_types::backend::Backend;
use ruffle_types::string::AvmString;

/// `Boolean` constructor
pub fn constructor<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    // Called from a constructor, populate `this`.
    if let Some(mut vbox) = this.as_value_object() {
        let cons_value = args
            .get(0)
            .map_or(false, |value| value.as_bool(activation.swf_version()));
        vbox.replace_value(activation.context.gc_context, cons_value.into());
    }

    Ok(this.into())
}

/// `Boolean` function
pub fn boolean_function<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    // If called as a function, return the value.
    // Boolean() with no argument returns undefined.
    Ok(args
        .get(0)
        .map(|value| value.as_bool(activation.swf_version()))
        .map_or(Value::Undefined, Value::Bool))
}

pub fn create_boolean_object<'gc, B: Backend>(
    gc_context: MutationContext<'gc, '_>,
    boolean_proto: Object<'gc, B>,
    fn_proto: Option<Object<'gc, B>>,
) -> Object<'gc, B> {
    FunctionObject::constructor(
        gc_context,
        Executable::Native(constructor),
        Executable::Native(boolean_function),
        fn_proto,
        boolean_proto,
    )
}

/// Creates `Boolean.prototype`.
pub fn create_proto<'gc, B: Backend>(
    gc_context: MutationContext<'gc, '_>,
    proto: Object<'gc, B>,
    fn_proto: Object<'gc, B>,
) -> Object<'gc, B> {
    let boolean_proto = ValueObject::empty_box(gc_context, Some(proto));
    let object = boolean_proto.as_script_object().unwrap();

    let PROTO_DECLS: &[Declaration<B>] = declare_properties! {
        "toString" => method(to_string; DONT_ENUM | DONT_DELETE);
        "valueOf" => method(value_of; DONT_ENUM | DONT_DELETE);
    };
    define_properties_on(PROTO_DECLS, gc_context, object, fn_proto);

    boolean_proto
}

pub fn to_string<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(vbox) = this.as_value_object() {
        // Must be a bool.
        // Boolean.prototype.toString.call(x) returns undefined for non-bools.
        if let Value::Bool(b) = vbox.unbox() {
            return Ok(AvmString::new_utf8(activation.context.gc_context, b.to_string()).into());
        }
    }

    Ok(Value::Undefined)
}

pub fn value_of<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(vbox) = this.as_value_object() {
        // Must be a bool.
        // Boolean.prototype.valueOf.call(x) returns undefined for non-bools.
        if let Value::Bool(b) = vbox.unbox() {
            return Ok(b.into());
        }
    }

    Ok(Value::Undefined)
}
