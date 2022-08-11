//! `flash.display.Loader` builtin/prototype

use crate::avm2::activation::Activation;
use crate::avm2::class::{Class, ClassAttributes};
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::names::{Namespace, QName};
use crate::avm2::value::Value;
use crate::avm2::{Error, Object};
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;

/// Implements `flash.display.Loader`'s class constructor.
pub fn class_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

/// Implements `flash.display.Loader`'s instance constructor.
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

// TODO: this is entirely stubbed, just to get addEventListener to not crash
fn content_loader_info<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    log::warn!("Loader::contentLoaderInfo is a stub");
    Ok(this.unwrap().into())
}

pub fn create_class<'gc, B: Backend>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc, B>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.display"), "Loader"),
        Some(
            QName::new(
                Namespace::package("flash.display"),
                "DisplayObjectContainer",
            )
            .into(),
        ),
        Method::from_builtin(instance_init, "<Loader instance initializer>", mc),
        Method::from_builtin(class_init, "<Loader class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);

    write.set_attributes(ClassAttributes::SEALED);

    let public_instance_properties: &[(
        &str,
        Option<NativeMethodImpl<B>>,
        Option<NativeMethodImpl<B>>,
    )] = &[("contentLoaderInfo", Some(content_loader_info), None)];
    write.define_public_builtin_instance_properties(mc, public_instance_properties);

    class
}
