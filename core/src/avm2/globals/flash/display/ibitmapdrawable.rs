//! `flash.display.IBitmapDrawable` builtin

use crate::avm2::activation::Activation;
use crate::avm2::class::{Class, ClassAttributes};
use crate::avm2::method::Method;
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::Object;
use crate::avm2::value::Value;
use crate::avm2::Error;
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;

/// Emulates attempts to execute bodiless methods.
pub fn bodiless_method<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Cannot execute non-native method without body".into())
}

/// Implements `flash.display.IBitmapDrawable`'s class constructor.
pub fn class_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

/// Construct `IBitmapDrawable`'s class.
pub fn create_interface<'gc, B: Backend>(
    mc: MutationContext<'gc, '_>,
) -> GcCell<'gc, Class<'gc, B>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.display"), "IBitmapDrawable"),
        None,
        Method::from_builtin(
            bodiless_method,
            "<IBitmapDrawable instance initializer>",
            mc,
        ),
        Method::from_builtin(class_init, "<IBitmapDrawable interface initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);

    write.set_attributes(ClassAttributes::INTERFACE);

    class
}
