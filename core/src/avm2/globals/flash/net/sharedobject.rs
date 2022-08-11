//! `flash.net.SharedObject` builtin/prototype

use crate::avm2::class::{Class, ClassAttributes};
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::object::TObject;
use crate::avm2::traits::Trait;
use crate::avm2::{Activation, Error, Namespace, Object, QName, Value};
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;

fn instance_init<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mut this) = this {
        activation.super_init(this, &[])?;

        let data = activation
            .context
            .avm2
            .classes()
            .object
            .construct(activation, &[])?;
        this.set_property(
            &QName::new(Namespace::public(), "data").into(),
            data.into(),
            activation,
        )?;
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

fn get_local<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    log::warn!("SharedObject.getLocal not implemented");
    let class = activation.context.avm2.classes().sharedobject;
    let new_shared_object = class.construct(activation, &[])?;

    Ok(new_shared_object.into())
}

fn flush<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    log::warn!("SharedObject.flush not implemented");
    Ok(Value::Undefined)
}

/// Construct `SharedObject`'s class.
pub fn create_class<'gc, B: Backend>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc, B>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.net"), "SharedObject"),
        Some(QName::new(Namespace::package("flash.events"), "EventDispatcher").into()),
        Method::from_builtin(instance_init, "<SharedObject instance initializer>", mc),
        Method::from_builtin(class_init, "<SharedObject class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);
    write.set_attributes(ClassAttributes::SEALED);

    write.define_instance_trait(Trait::from_slot(
        QName::new(Namespace::public(), "data"),
        QName::new(Namespace::public(), "Object").into(),
        None,
    ));

    let public_class_methods: &[(&str, NativeMethodImpl<B>)] = &[("getLocal", get_local)];
    write.define_public_builtin_class_methods(mc, public_class_methods);

    let public_instance_methods: &[(&str, NativeMethodImpl<B>)] = &[("flush", flush)];
    write.define_public_builtin_instance_methods(mc, public_instance_methods);
    class
}
