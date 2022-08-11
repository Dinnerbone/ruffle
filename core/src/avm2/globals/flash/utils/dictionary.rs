//! `flash.utils.Dictionary` builtin/prototype

use crate::avm2::activation::Activation;
use crate::avm2::class::Class;
use crate::avm2::method::Method;
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::{dictionary_allocator, Object};
use crate::avm2::value::Value;
use crate::avm2::Error;
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;

/// Implements `flash.utils.Dictionary`'s instance constructor.
pub fn instance_init<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        activation.super_init(this, &[])?;
    }

    Ok(Value::Undefined)
}

/// Implements `flash.utils.Dictionary`'s class constructor.
pub fn class_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

/// Construct `Dictionary`'s class.
pub fn create_class<'gc, B: Backend>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc, B>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.utils"), "Dictionary"),
        Some(QName::new(Namespace::package(""), "Object").into()),
        Method::from_builtin(instance_init, "<Dictionary instance initializer>", mc),
        Method::from_builtin(class_init, "<Dictionary class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);

    write.set_instance_allocator(dictionary_allocator);

    class
}
