//! `flash.system.ApplicationDomain` class

use crate::avm2::activation::Activation;
use crate::avm2::class::Class;
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::{appdomain_allocator, DomainObject, Object, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;

/// Implements `flash.system.ApplicationDomain`'s instance constructor.
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

/// Implements `flash.system.ApplicationDomain`'s class constructor.
pub fn class_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

/// `currentDomain` static property.
pub fn current_domain<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    let appdomain = activation.caller_domain();

    Ok(DomainObject::from_domain(activation, appdomain)?.into())
}

/// `parentDomain` property
pub fn parent_domain<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(appdomain) = this.and_then(|this| this.as_application_domain()) {
        if let Some(parent_domain) = appdomain.parent_domain() {
            return Ok(DomainObject::from_domain(activation, parent_domain)?.into());
        }
    }

    Ok(Value::Null)
}

/// `getDefinition` method
pub fn get_definition<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(appdomain) = this.and_then(|this| this.as_application_domain()) {
        let local_name = args
            .get(0)
            .cloned()
            .unwrap_or_else(|| "".into())
            .coerce_to_string(activation)?;
        let qname = QName::new(Namespace::public(), local_name);

        let (qname, mut defined_script) = appdomain
            .get_defining_script(&qname.into())?
            .ok_or_else(|| format!("No definition called {} exists", local_name))?;
        let globals = defined_script.globals(&mut activation.context)?;
        let definition = globals.get_property(&qname.into(), activation)?;

        return Ok(definition);
    }

    Ok(Value::Undefined)
}

/// `hasDefinition` method
pub fn has_definition<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(appdomain) = this.and_then(|this| this.as_application_domain()) {
        let local_name = args
            .get(0)
            .cloned()
            .unwrap_or_else(|| "".into())
            .coerce_to_string(activation)?;
        let qname = QName::new(Namespace::public(), local_name);

        return Ok(appdomain.has_definition(qname).into());
    }

    Ok(Value::Undefined)
}

/// `domainMemory` property setter
pub fn set_domain_memory<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(Value::Object(arg)) = args.get(0) {
        if let Some(bytearray_obj) = arg.as_bytearray_object() {
            if let Some(appdomain) = this.and_then(|this| this.as_application_domain()) {
                appdomain.set_domain_memory(activation.context.gc_context, bytearray_obj);
            }
        }
    }

    Ok(Value::Undefined)
}

/// `domainMemory` property getter
pub fn domain_memory<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(appdomain) = this.and_then(|this| this.as_application_domain()) {
        let bytearray_object: Object<'gc, B> = appdomain.domain_memory().into();
        return Ok(bytearray_object.into());
    }

    Ok(Value::Undefined)
}

/// Construct `ApplicationDomain`'s class.
pub fn create_class<'gc, B: Backend>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc, B>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.system"), "ApplicationDomain"),
        Some(QName::new(Namespace::public(), "Object").into()),
        Method::from_builtin(
            instance_init,
            "<ApplicationDomain instance initializer>",
            mc,
        ),
        Method::from_builtin(class_init, "<ApplicationDomain class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);
    write.set_instance_allocator(appdomain_allocator);

    let public_class_methods: &[(&str, NativeMethodImpl<B>)] = &[
        ("currentDomain", current_domain),
        ("parentDomain", parent_domain),
        ("getDefinition", get_definition),
        ("hasDefinition", has_definition),
    ];
    write.define_public_builtin_class_methods(mc, public_class_methods);

    let public_instance_properties: &[(
        &str,
        Option<NativeMethodImpl<B>>,
        Option<NativeMethodImpl<B>>,
    )] = &[("domainMemory", Some(domain_memory), Some(set_domain_memory))];
    write.define_public_builtin_instance_properties(mc, public_instance_properties);

    class
}
