//! flash.external.ExternalInterface object

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::property_decl::{define_properties_on, Declaration};
use crate::avm1::{Object, ScriptObject, Value};
use crate::external::{Callback, Value as ExternalValue};
use gc_arena::MutationContext;
use ruffle_types::backend::Backend;

pub fn get_available<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    Ok(activation.context.external_interface.available().into())
}

pub fn add_callback<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if args.len() < 3 {
        return Ok(false.into());
    }

    let name = args.get(0).unwrap().coerce_to_string(activation)?;
    let this = args.get(1).unwrap().to_owned();
    let method = args.get(2).unwrap();

    if let Value::Object(method) = method {
        activation.context.external_interface.add_callback(
            name.to_string(),
            Callback::Avm1 {
                this,
                method: *method,
            },
        );
        Ok(true.into())
    } else {
        Ok(false.into())
    }
}

pub fn call<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if args.is_empty() {
        return Ok(Value::Null);
    }

    let name = args.get(0).unwrap().coerce_to_string(activation)?;
    if let Some(method) = activation
        .context
        .external_interface
        .get_method_for(&name.to_utf8_lossy())
    {
        let mut external_args = Vec::with_capacity(args.len() - 1);
        for arg in &args[1..] {
            external_args.push(ExternalValue::from_avm1(activation, arg.to_owned())?);
        }
        Ok(method
            .call(&mut activation.context, &external_args)
            .into_avm1(activation))
    } else {
        Ok(Value::Null)
    }
}

pub fn create_external_interface_object<'gc, B: Backend>(
    gc_context: MutationContext<'gc, '_>,
    proto: Object<'gc, B>,
    fn_proto: Object<'gc, B>,
) -> Object<'gc, B> {
    let object = ScriptObject::object(gc_context, Some(proto));

    let OBJECT_DECLS: &[Declaration<B>] = declare_properties! {
        "available" => property(get_available; DONT_ENUM | DONT_DELETE | READ_ONLY);
        "addCallback" => method(add_callback; DONT_ENUM | DONT_DELETE | READ_ONLY);
        "call" => method(call; DONT_ENUM | DONT_DELETE | READ_ONLY);
    };
    define_properties_on(OBJECT_DECLS, gc_context, object, fn_proto);

    object.into()
}

pub fn create_proto<'gc, B: Backend>(
    gc_context: MutationContext<'gc, '_>,
    proto: Object<'gc, B>,
) -> Object<'gc, B> {
    // It's a custom prototype but it's empty.
    ScriptObject::object(gc_context, Some(proto)).into()
}
