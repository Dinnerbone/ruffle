//! Stage object
//!
//! TODO: This is a very rough stub with not much implementation.
use crate::avm1::function::Executable;
use crate::avm1::property::Attribute;
use crate::avm1::return_value::ReturnValue;
use crate::avm1::{Avm1, Error, Object, ScriptObject, TObject, UpdateContext, Value};

use gc_arena::MutationContext;
use crate::backend::Backends;

pub fn create_stage_object<'gc, B: Backends>(
    gc_context: MutationContext<'gc, '_>,
    proto: Option<Object<'gc, B>>,
    _array_proto: Option<Object<'gc, B>>,
    fn_proto: Option<Object<'gc, B>>,
) -> Object<'gc, B> {
    let mut stage = ScriptObject::object(gc_context, proto);

    stage.force_set_function(
        "addListener",
        add_listener,
        gc_context,
        Attribute::DontEnum | Attribute::DontDelete | Attribute::ReadOnly,
        fn_proto,
    );

    stage.add_property(
        gc_context,
        "align",
        Executable::Native(align),
        Some(Executable::Native(set_align)),
        Attribute::DontEnum | Attribute::DontDelete,
    );

    stage.add_property(
        gc_context,
        "height",
        Executable::Native(height),
        None,
        Attribute::DontEnum | Attribute::DontDelete | Attribute::ReadOnly,
    );

    stage.force_set_function(
        "removeListener",
        remove_listener,
        gc_context,
        Attribute::DontEnum | Attribute::DontDelete | Attribute::ReadOnly,
        fn_proto,
    );

    stage.add_property(
        gc_context,
        "scaleMode",
        Executable::Native(scale_mode),
        Some(Executable::Native(set_scale_mode)),
        Attribute::DontEnum | Attribute::DontDelete,
    );

    stage.add_property(
        gc_context,
        "showMenu",
        Executable::Native(show_menu),
        Some(Executable::Native(set_show_menu)),
        Attribute::DontEnum | Attribute::DontDelete,
    );

    stage.add_property(
        gc_context,
        "width",
        Executable::Native(width),
        None,
        Attribute::DontEnum | Attribute::DontDelete | Attribute::ReadOnly,
    );

    stage.into()
}

fn add_listener<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    _context: &mut UpdateContext<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    log::warn!("Stage.addListener: unimplemented");
    Ok(Value::Undefined.into())
}

fn align<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    _context: &mut UpdateContext<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    log::warn!("Stage.align: unimplemented");
    Ok("".into())
}

fn set_align<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    _context: &mut UpdateContext<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    log::warn!("Stage.align: unimplemented");
    Ok(Value::Undefined.into())
}

fn height<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    context: &mut UpdateContext<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    Ok(context.stage_size.1.to_pixels().into())
}

fn remove_listener<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    _context: &mut UpdateContext<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    log::warn!("Stage.removeListener: unimplemented");
    Ok("".into())
}

fn scale_mode<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    _context: &mut UpdateContext<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    log::warn!("Stage.scaleMode: unimplemented");
    Ok("noScale".into())
}

fn set_scale_mode<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    _context: &mut UpdateContext<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    log::warn!("Stage.scaleMode: unimplemented");
    Ok(Value::Undefined.into())
}

fn show_menu<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    _context: &mut UpdateContext<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    log::warn!("Stage.showMenu: unimplemented");
    Ok(true.into())
}

fn set_show_menu<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    _context: &mut UpdateContext<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    log::warn!("Stage.showMenu: unimplemented");
    Ok(Value::Undefined.into())
}

fn width<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    context: &mut UpdateContext<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    Ok(context.stage_size.0.to_pixels().into())
}
