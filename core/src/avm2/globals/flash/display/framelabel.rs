//! `flash.display.FrameLabel` impl

use crate::avm2::activation::Activation;
use crate::avm2::class::{define_indirect_properties, Class};
use crate::avm2::globals::NS_RUFFLE_INTERNAL;
use crate::avm2::method::Method;
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::{Object, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;

/// Implements `flash.display.FrameLabel`'s instance constructor.
pub fn instance_init<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    let name = args
        .get(0)
        .cloned()
        .unwrap_or(Value::Undefined)
        .coerce_to_string(activation)?;
    let frame = args
        .get(1)
        .cloned()
        .unwrap_or(Value::Undefined)
        .coerce_to_i32(activation)?;

    if let Some(mut this) = this {
        activation.super_init(this, &[])?;

        this.set_property(
            &QName::new(Namespace::Private(NS_RUFFLE_INTERNAL.into()), "name").into(),
            name.into(),
            activation,
        )?;
        this.set_property(
            &QName::new(Namespace::Private(NS_RUFFLE_INTERNAL.into()), "frame").into(),
            frame.into(),
            activation,
        )?;
    }

    Ok(Value::Undefined)
}

/// Implements `flash.display.FrameLabel`'s class constructor.
pub fn class_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}
/// Construct `FrameLabel`'s class.
pub fn create_class<'gc, B: Backend>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc, B>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.display"), "FrameLabel"),
        Some(QName::new(Namespace::package("flash.events"), "EventDispatcher").into()),
        Method::from_builtin(instance_init, "<FrameLabel instance initializer>", mc),
        Method::from_builtin(class_init, "<FrameLabel class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);

    define_indirect_properties!(write, mc, [("name", "", "String"), ("frame", "", "int")]);
    class
}
