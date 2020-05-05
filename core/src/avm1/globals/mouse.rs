use crate::avm1::listeners::Listeners;
use crate::avm1::property::Attribute;
use crate::avm1::return_value::ReturnValue;
use crate::avm1::{Avm1, Error, Object, ScriptObject, TObject, UpdateContext, Value};

use gc_arena::MutationContext;
use crate::backend::Backends;

pub fn show_mouse<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    context: &mut UpdateContext<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    let was_visible = context.input.mouse_visible();
    context.input.show_mouse();
    if was_visible {
        Ok(0.into())
    } else {
        Ok(1.into())
    }
}

pub fn hide_mouse<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    context: &mut UpdateContext<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    let was_visible = context.input.mouse_visible();
    context.input.hide_mouse();
    if was_visible {
        Ok(0.into())
    } else {
        Ok(1.into())
    }
}

pub fn create_mouse_object<'gc, B: Backends>(
    gc_context: MutationContext<'gc, '_>,
    proto: Option<Object<'gc, B>>,
    fn_proto: Option<Object<'gc, B>>,
    listener: &Listeners<'gc, B>,
) -> Object<'gc, B> {
    let mut mouse = ScriptObject::object(gc_context, proto);

    register_listener!(gc_context, mouse, listener, fn_proto, mouse);

    mouse.force_set_function(
        "show",
        show_mouse,
        gc_context,
        Attribute::DontEnum | Attribute::DontDelete | Attribute::ReadOnly,
        fn_proto,
    );

    mouse.force_set_function(
        "hide",
        hide_mouse,
        gc_context,
        Attribute::DontEnum | Attribute::DontDelete | Attribute::ReadOnly,
        fn_proto,
    );

    mouse.into()
}
