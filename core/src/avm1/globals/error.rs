//! Error object

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::property_decl::{define_properties_on, Declaration};
use crate::avm1::{Object, ScriptObject, TObject, Value};
use gc_arena::MutationContext;
use ruffle_types::backend::Backend;

pub fn constructor<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let message: Value<'gc, B> = args.get(0).cloned().unwrap_or(Value::Undefined);

    if message != Value::Undefined {
        this.set("message", message, activation)?;
    }

    Ok(this.into())
}

pub fn create_proto<'gc, B: Backend>(
    gc_context: MutationContext<'gc, '_>,
    proto: Object<'gc, B>,
    fn_proto: Object<'gc, B>,
) -> Object<'gc, B> {
    let object = ScriptObject::object(gc_context, Some(proto));

    let PROTO_DECLS: &[Declaration<B>] = declare_properties! {
        "message" => string("Error");
        "name" => string("Error");
        "toString" => method(to_string);
    };
    define_properties_on(PROTO_DECLS, gc_context, object, fn_proto);

    object.into()
}

fn to_string<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let message = this.get("message", activation)?;
    Ok(message.coerce_to_string(activation)?.into())
}
