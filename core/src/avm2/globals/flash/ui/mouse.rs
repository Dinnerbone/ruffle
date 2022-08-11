//! `flash.ui.Mouse` builtin

use crate::avm2::activation::Activation;
use crate::avm2::class::{Class, ClassAttributes};
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::Object;
use crate::avm2::value::Value;
use crate::avm2::Error;
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;

fn instance_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("The Mouse class cannot be constructed.".into())
}

fn class_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

fn hide<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    activation.context.ui.set_mouse_visible(false);
    Ok(Value::Undefined)
}

fn show<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    activation.context.ui.set_mouse_visible(true);
    Ok(Value::Undefined)
}

pub fn create_class<'gc, B: Backend>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc, B>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.ui"), "Mouse"),
        Some(QName::new(Namespace::package(""), "Object").into()),
        Method::from_builtin(instance_init, "<Mouse instance initializer>", mc),
        Method::from_builtin(class_init, "<Mouse class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);

    write.set_attributes(ClassAttributes::SEALED | ClassAttributes::FINAL);

    let public_class_methods: &[(&str, NativeMethodImpl<B>)] = &[("show", show), ("hide", hide)];
    write.define_public_builtin_class_methods(mc, public_class_methods);

    class
}
