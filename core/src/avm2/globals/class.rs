//! `Class` builtin/prototype

use crate::avm2::activation::Activation;
use crate::avm2::class::Class;
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::{Object, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;

/// Implements `Class`'s instance initializer.
///
/// Notably, you cannot construct new classes this way, so this returns an
/// error.
pub fn instance_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Classes cannot be constructed.".into())
}

/// Implement's `Class`'s class initializer.
pub fn class_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

fn prototype<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        if let Some(class) = this.as_class_object() {
            return Ok(class.prototype().into());
        }
    }
    Ok(Value::Undefined)
}

/// Construct `Class`'s class.
pub fn create_class<'gc, B: Backend>(
    gc_context: MutationContext<'gc, '_>,
) -> GcCell<'gc, Class<'gc, B>> {
    let class_class = Class::new(
        QName::new(Namespace::public(), "Class"),
        Some(QName::new(Namespace::public(), "Object").into()),
        Method::from_builtin(instance_init, "<Class instance initializer>", gc_context),
        Method::from_builtin(class_init, "<Class class initializer>", gc_context),
        gc_context,
    );

    let mut write = class_class.write(gc_context);

    let public_instance_properties: &[(
        &str,
        Option<NativeMethodImpl<B>>,
        Option<NativeMethodImpl<B>>,
    )] = &[("prototype", Some(prototype), None)];
    write.define_public_builtin_instance_properties(gc_context, public_instance_properties);

    class_class
}
