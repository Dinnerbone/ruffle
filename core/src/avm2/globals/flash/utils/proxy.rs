use crate::avm2::activation::Activation;
use crate::avm2::class::{Class, ClassAttributes};
use crate::avm2::globals::flash::utils::NS_FLASH_PROXY;
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::{proxy_allocator, Object};
use crate::avm2::value::Value;
use crate::avm2::Error;
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;

/// Implements `flash.utils.Proxy`'s instance constructor.
pub fn instance_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

/// Implements `flash.utils.Proxy`'s class constructor.
pub fn class_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

/// Implements `Proxy.getProperty`
pub fn get_property<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("getproperty is not implemented for this Proxy".into())
}

/// Implements `Proxy.setProperty`
pub fn set_property<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("setproperty is not implemented for this Proxy".into())
}

/// Implements `Proxy.deleteProperty`
pub fn delete_property<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("deleteproperty is not implemented for this Proxy".into())
}

/// Implements `Proxy.callProperty`
pub fn call_property<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("callproperty is not implemented for this Proxy".into())
}

/// Implements `Proxy.hasProperty`
pub fn has_property<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("hasproperty is not implemented for this Proxy".into())
}

/// Implements `Proxy.isAttribute`
pub fn is_attribute<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("isattribute is not implemented for this Proxy".into())
}

/// Implements `Proxy.getDescendants`
pub fn get_descendants<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("getdescendants is not implemented for this Proxy".into())
}

/// Implements `Proxy.nextNameIndex`
pub fn next_name_index<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("hasnext/nextNameIndex is not implemented for this Proxy".into())
}

/// Implements `Proxy.nextName`
pub fn next_name<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("nextname is not implemented for this Proxy".into())
}

/// Implements `Proxy.nextValue`
pub fn next_value<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("nextvalue is not implemented for this Proxy".into())
}

pub fn create_class<'gc, B: Backend>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc, B>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.utils"), "Proxy"),
        Some(QName::new(Namespace::public(), "Object").into()),
        Method::from_builtin(instance_init, "<Proxy instance initializer>", mc),
        Method::from_builtin(class_init, "<Proxy class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);

    write.set_attributes(ClassAttributes::SEALED);
    write.set_instance_allocator(proxy_allocator);

    let flash_proxy_instance_methods: &[(&str, NativeMethodImpl<B>)] = &[
        ("getProperty", get_property),
        ("setProperty", set_property),
        ("deleteProperty", delete_property),
        ("callProperty", call_property),
        ("hasProperty", has_property),
        ("isAttribute", is_attribute),
        ("getDescendants", get_descendants),
        ("nextNameIndex", next_name_index),
        ("nextName", next_name),
        ("nextValue", next_value),
    ];

    write.define_ns_builtin_instance_methods(mc, NS_FLASH_PROXY, flash_proxy_instance_methods);

    class
}
