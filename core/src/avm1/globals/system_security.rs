use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::object::Object;
use crate::avm1::property_decl::{define_properties_on, Declaration};
use crate::avm1::{ScriptObject, Value};
use crate::avm_warn;
use gc_arena::MutationContext;
use ruffle_types::backend::Backend;
use ruffle_types::string::AvmString;

fn allow_domain<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    avm_warn!(activation, "System.security.allowDomain() not implemented");
    Ok(Value::Undefined)
}

fn allow_insecure_domain<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    avm_warn!(
        activation,
        "System.security.allowInsecureDomain() not implemented"
    );
    Ok(Value::Undefined)
}

fn load_policy_file<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    avm_warn!(
        activation,
        "System.security.loadPolicyFile() not implemented"
    );
    Ok(Value::Undefined)
}

fn escape_domain<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    avm_warn!(activation, "System.security.escapeDomain() not implemented");
    Ok(Value::Undefined)
}

fn get_sandbox_type<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    Ok(AvmString::new_utf8(
        activation.context.gc_context,
        activation.context.system.sandbox_type.to_string(),
    )
    .into())
}

fn get_choose_local_swf_path<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    avm_warn!(
        activation,
        "System.security.chooseLocalSwfPath() not implemented"
    );
    Ok(Value::Undefined)
}

fn policy_file_resolver<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    avm_warn!(
        activation,
        "System.security.chooseLocalSwfPath() not implemented"
    );
    Ok(Value::Undefined)
}

pub fn create<'gc, B: Backend>(
    gc_context: MutationContext<'gc, '_>,
    proto: Option<Object<'gc, B>>,
    fn_proto: Object<'gc, B>,
) -> Object<'gc, B> {
    let security = ScriptObject::object(gc_context, proto);

    let OBJECT_DECLS: &[Declaration<B>] = declare_properties! {
        "PolicyFileResolver" => method(policy_file_resolver);
        "allowDomain" => method(allow_domain);
        "allowInsecureDomain" => method(allow_insecure_domain);
        "loadPolicyFile" => method(load_policy_file);
        "escapeDomain" => method(escape_domain);
        "sandboxType" => property(get_sandbox_type);
        "chooseLocalSwfPath" => property(get_choose_local_swf_path);
    };
    define_properties_on(OBJECT_DECLS, gc_context, security, fn_proto);

    security.into()
}
