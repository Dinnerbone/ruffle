//! `flash.system.Security` native methods

use crate::avm2::activation::Activation;
use crate::avm2::object::Object;
use crate::avm2::value::Value;
use crate::avm2::Error;
use ruffle_types::backend::Backend;
use ruffle_types::string::AvmString;

pub fn get_sandbox_type<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    let sandbox_type = activation.context.system.sandbox_type.to_string();
    return Ok(AvmString::new_utf8(activation.context.gc_context, sandbox_type).into());
}

pub fn allow_domain<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    log::warn!("Security.allowDomain not implemented");
    Ok(Value::Undefined)
}

pub fn allow_insecure_domain<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    log::warn!("Security.allowInsecureDomain not implemented");
    Ok(Value::Undefined)
}

pub fn load_policy_file<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    log::warn!("Security.loadPolicyFile not implemented");
    Ok(Value::Undefined)
}

pub fn show_settings<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    log::warn!("Security.showSettings not implemented");
    Ok(Value::Undefined)
}
