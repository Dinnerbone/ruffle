//! XML builtin and prototype

use crate::avm2::activation::Activation;
use crate::avm2::class::Class;
use crate::avm2::method::{Method, ParamConfig};
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::{xml_allocator, Object};
use crate::avm2::value::Value;
use crate::avm2::Error;
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;

/// Implements `XML`'s instance initializer.
pub fn instance_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

/// Implements `XML`'s class initializer
pub fn class_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

pub fn create_class<'gc, B: Backend>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc, B>> {
    let class = Class::new(
        QName::new(Namespace::public(), "XML"),
        Some(QName::new(Namespace::public(), "Object").into()),
        Method::from_builtin_and_params(
            instance_init,
            "<XML instance initializer>",
            vec![ParamConfig::optional(
                "value",
                QName::new(Namespace::public(), "Object").into(),
                Value::Undefined,
            )],
            false,
            mc,
        ),
        Method::from_builtin(class_init, "<XML class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);
    write.set_instance_allocator(xml_allocator);

    class
}
