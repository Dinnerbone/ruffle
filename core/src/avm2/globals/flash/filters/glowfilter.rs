//! `flash.filters.GlowFilter` builtin/prototype

use crate::avm2::activation::Activation;
use crate::avm2::class::{Class, ClassAttributes};
use crate::avm2::method::Method;
use crate::avm2::names::{Namespace, QName};
use crate::avm2::value::Value;
use crate::avm2::{Error, Object};
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;

/// Implements `flash.filters.GlowFilter`'s class constructor.
pub fn class_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

/// Implements `flash.filters.GlowFilter`'s instance constructor.
pub fn instance_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

pub fn create_class<'gc, B: Backend>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc, B>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.filters"), "GlowFilter"),
        Some(QName::new(Namespace::package("flash.filters"), "BitmapFilter").into()),
        Method::from_builtin(instance_init, "<GlowFilter instance initializer>", mc),
        Method::from_builtin(class_init, "<GlowFilter class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);
    write.set_attributes(ClassAttributes::FINAL | ClassAttributes::SEALED);

    const PUBLIC_INSTANCE_SLOTS: &[(&str, &str, &str)] = &[
        ("alpha", "", "Number"),
        ("blurX", "", "Number"),
        ("blurY", "", "Number"),
        ("color", "", "uint"),
        ("inner", "", "Boolean"),
        ("knockout", "", "Boolean"),
        ("quality", "", "int"),
        ("strength", "", "Number"),
    ];
    write.define_public_slot_instance_traits(PUBLIC_INSTANCE_SLOTS);

    class
}
