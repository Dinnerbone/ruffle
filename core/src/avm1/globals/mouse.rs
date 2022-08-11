use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::globals::as_broadcaster::BroadcasterFunctions;
use crate::avm1::property_decl::{define_properties_on, Declaration};
use crate::avm1::{Object, ScriptObject, Value};
use gc_arena::MutationContext;
use ruffle_types::backend::Backend;

pub fn show_mouse<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let was_visible = activation.context.ui.mouse_visible();
    activation.context.ui.set_mouse_visible(true);
    Ok(if was_visible { 0 } else { 1 }.into())
}

pub fn hide_mouse<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let was_visible = activation.context.ui.mouse_visible();
    activation.context.ui.set_mouse_visible(false);
    Ok(if was_visible { 0 } else { 1 }.into())
}

pub fn create_mouse_object<'gc, B: Backend>(
    gc_context: MutationContext<'gc, '_>,
    proto: Option<Object<'gc, B>>,
    fn_proto: Object<'gc, B>,
    broadcaster_functions: BroadcasterFunctions<'gc, B>,
    array_proto: Object<'gc, B>,
) -> Object<'gc, B> {
    let mouse = ScriptObject::object(gc_context, proto);
    broadcaster_functions.initialize(gc_context, mouse.into(), array_proto);

    let OBJECT_DECLS: &[Declaration<B>] = declare_properties! {
        "show" => method(show_mouse; DONT_DELETE | DONT_ENUM | READ_ONLY);
        "hide" => method(hide_mouse; DONT_DELETE | DONT_ENUM | READ_ONLY);
    };
    define_properties_on(OBJECT_DECLS, gc_context, mouse, fn_proto);

    mouse.into()
}
