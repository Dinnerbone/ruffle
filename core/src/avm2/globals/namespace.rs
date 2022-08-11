//! `Namespace` impl

use crate::avm2::activation::Activation;
use crate::avm2::class::Class;
use crate::avm2::method::Method;
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::{namespace_allocator, Object};
use crate::avm2::value::Value;
use crate::avm2::Error;
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;

/// Implements `Namespace`'s instance initializer.
pub fn instance_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Namespace constructor is a stub.".into())
}

fn class_call<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Namespace constructor is a stub.".into())
}

/// Implements `Namespace`'s native instance initializer.
pub fn native_instance_init<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        activation.super_init(this, args)?;
    }

    Ok(Value::Undefined)
}

/// Implements `Namespace`'s class initializer.
pub fn class_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

/// Construct `Namespace`'s class.
pub fn create_class<'gc, B: Backend>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc, B>> {
    let class = Class::new(
        QName::new(Namespace::public(), "Namespace"),
        Some(QName::new(Namespace::public(), "Object").into()),
        Method::from_builtin(instance_init, "<Namespace instance initializer>", mc),
        Method::from_builtin(class_init, "<Namespace class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);
    write.set_instance_allocator(namespace_allocator);
    write.set_native_instance_init(Method::from_builtin(
        native_instance_init,
        "<Namespace native instance initializer>",
        mc,
    ));
    write.set_call_handler(Method::from_builtin(
        class_call,
        "<Namespace call handler>",
        mc,
    ));

    class
}
