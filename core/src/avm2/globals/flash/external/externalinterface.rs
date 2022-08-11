//! `flash.external.ExternalInterface` builtin/prototype

use crate::avm2::class::{Class, ClassAttributes};
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::{Activation, Error, Namespace, Object, QName, Value};
use crate::external::{Callback, Value as ExternalValue};
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;

fn instance_init<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        activation.super_init(this, &[])?;
    }

    Ok(Value::Undefined)
}

fn class_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

pub fn call<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
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
            external_args.push(ExternalValue::from_avm2(activation, arg.to_owned())?);
        }
        Ok(method
            .call(&mut activation.context, &external_args)
            .into_avm2(activation))
    } else {
        Ok(Value::Null)
    }
}

pub fn available<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(activation.context.external_interface.available().into())
}

pub fn add_callback<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if args.len() < 2 {
        return Ok(Value::Undefined);
    }

    let name = args.get(0).unwrap().coerce_to_string(activation)?;
    let method = args.get(1).unwrap();

    if let Value::Object(method) = method {
        activation
            .context
            .external_interface
            .add_callback(name.to_string(), Callback::Avm2 { method: *method });
    }
    Ok(Value::Undefined)
}

/// Construct `ExternalInterface`'s class.
pub fn create_class<'gc, B: Backend>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc, B>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.external"), "ExternalInterface"),
        Some(QName::new(Namespace::public(), "Object").into()),
        Method::from_builtin(
            instance_init,
            "<ExternalInterface instance initializer>",
            mc,
        ),
        Method::from_builtin(class_init, "<ExternalInterface class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);
    write.set_attributes(ClassAttributes::FINAL | ClassAttributes::SEALED);

    let public_class_methods: &[(&str, NativeMethodImpl<B>)] =
        &[("call", call), ("addCallback", add_callback)];

    write.define_public_builtin_class_methods(mc, public_class_methods);

    let public_instance_properties: &[(
        &str,
        Option<NativeMethodImpl<B>>,
        Option<NativeMethodImpl<B>>,
    )] = &[("available", Some(available), None)];

    write.define_public_builtin_class_properties(mc, public_instance_properties);

    class
}
