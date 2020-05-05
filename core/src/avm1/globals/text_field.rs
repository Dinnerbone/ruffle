use crate::avm1::function::Executable;
use crate::avm1::globals::display_object;
use crate::avm1::property::Attribute::*;
use crate::avm1::return_value::ReturnValue;
use crate::avm1::{Avm1, Error, Object, ScriptObject, TObject, UpdateContext, Value};
use crate::display_object::{EditText, TDisplayObject};
use crate::font::TextFormat;
use gc_arena::MutationContext;
use crate::backend::Backends;

/// Implements `TextField`
pub fn constructor<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    _context: &mut UpdateContext<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    Ok(Value::Undefined.into())
}

pub fn get_text<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    _context: &mut UpdateContext<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    if let Some(display_object) = this.as_display_object() {
        if let Some(text_field) = display_object.as_edit_text() {
            return Ok(text_field.text().into());
        }
    }
    Ok(Value::Undefined.into())
}

pub fn set_text<'gc, B: Backends>(
    avm: &mut Avm1<'gc, B>,
    context: &mut UpdateContext<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    if let Some(display_object) = this.as_display_object() {
        if let Some(text_field) = display_object.as_edit_text() {
            if let Some(value) = args.get(0) {
                text_field.set_text(
                    value
                        .to_owned()
                        .coerce_to_string(avm, context)
                        .unwrap_or_else(|_| "undefined".to_string()),
                    context.gc_context,
                )
            }
        }
    }
    Ok(Value::Undefined.into())
}

macro_rules! with_text_field {
    ( $gc_context: ident, $object:ident, $fn_proto: expr, $($name:expr => $fn:expr),* ) => {{
        $(
            $object.force_set_function(
                $name,
                |avm, context: &mut UpdateContext<'_, 'gc, '_, B>, this, args| -> Result<ReturnValue<'gc, B>, Error> {
                    if let Some(display_object) = this.as_display_object() {
                        if let Some(text_field) = display_object.as_edit_text() {
                            return $fn(text_field, avm, context, args);
                        }
                    }
                    Ok(Value::Undefined.into())
                } as crate::avm1::function::NativeFunction<'gc, B>,
                $gc_context,
                DontDelete | ReadOnly | DontEnum,
                $fn_proto
            );
        )*
    }};
}

pub fn text_width<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    context: &mut UpdateContext<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    if let Some(etext) = this
        .as_display_object()
        .and_then(|dobj| dobj.as_edit_text())
    {
        let metrics = etext.measure_text(context);

        return Ok(metrics.0.to_pixels().into());
    }

    Ok(Value::Undefined.into())
}

pub fn text_height<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    context: &mut UpdateContext<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    if let Some(etext) = this
        .as_display_object()
        .and_then(|dobj| dobj.as_edit_text())
    {
        let metrics = etext.measure_text(context);

        return Ok(metrics.1.to_pixels().into());
    }

    Ok(Value::Undefined.into())
}

pub fn multiline<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    _context: &mut UpdateContext<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    if let Some(etext) = this
        .as_display_object()
        .and_then(|dobj| dobj.as_edit_text())
    {
        return Ok(etext.is_multiline().into());
    }

    Ok(Value::Undefined.into())
}

pub fn set_multiline<'gc, B: Backends>(
    avm: &mut Avm1<'gc, B>,
    context: &mut UpdateContext<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    let is_multiline = args
        .get(0)
        .cloned()
        .unwrap_or(Value::Undefined)
        .as_bool(avm.current_swf_version());

    if let Some(etext) = this
        .as_display_object()
        .and_then(|dobj| dobj.as_edit_text())
    {
        etext.set_multiline(is_multiline, context.gc_context);
    }

    Ok(Value::Undefined.into())
}

pub fn word_wrap<'gc, B: Backends>(
    _avm: &mut Avm1<'gc, B>,
    _context: &mut UpdateContext<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    if let Some(etext) = this
        .as_display_object()
        .and_then(|dobj| dobj.as_edit_text())
    {
        return Ok(etext.is_word_wrap().into());
    }

    Ok(Value::Undefined.into())
}

pub fn set_word_wrap<'gc, B: Backends>(
    avm: &mut Avm1<'gc, B>,
    context: &mut UpdateContext<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<ReturnValue<'gc, B>, Error> {
    let is_word_wrap = args
        .get(0)
        .cloned()
        .unwrap_or(Value::Undefined)
        .as_bool(avm.current_swf_version());

    if let Some(etext) = this
        .as_display_object()
        .and_then(|dobj| dobj.as_edit_text())
    {
        etext.set_word_wrap(is_word_wrap, context.gc_context);
    }

    Ok(Value::Undefined.into())
}

pub fn create_proto<'gc, B: Backends>(
    gc_context: MutationContext<'gc, '_>,
    proto: Object<'gc, B>,
    fn_proto: Object<'gc, B>,
) -> Object<'gc, B> {
    let mut object = ScriptObject::object(gc_context, Some(proto));

    display_object::define_display_object_proto(gc_context, object, fn_proto);

    with_text_field!(
        gc_context,
        object,
        Some(fn_proto),
        "getNewTextFormat" => |text_field: EditText<'gc, B>, avm: &mut Avm1<'gc, B>, context: &mut UpdateContext<'_, 'gc, '_, B>, _args| {
            let tf = text_field.new_text_format();

            Ok(tf.as_avm1_object(avm, context)?.into())
        },
        "setNewTextFormat" => |text_field: EditText<'gc, B>, avm: &mut Avm1<'gc, B>, context: &mut UpdateContext<'_, 'gc, '_, B>, args: &[Value<'gc, B>]| {
            let tf = args.get(0).cloned().unwrap_or(Value::Undefined);

            if let Value::Object(tf) = tf {
                let tf_parsed = TextFormat::from_avm1_object(tf, avm, context)?;
                text_field.set_new_text_format(tf_parsed, context.gc_context);
            }

            Ok(Value::Undefined.into())
        }
    );

    object.into()
}

pub fn attach_virtual_properties<'gc, B: Backends>(gc_context: MutationContext<'gc, '_>, object: Object<'gc, B>) {
    object.add_property(
        gc_context,
        "text",
        Executable::Native(get_text),
        Some(Executable::Native(set_text)),
        DontDelete | ReadOnly | DontEnum,
    );
    object.add_property(
        gc_context,
        "textWidth",
        Executable::Native(text_width),
        None,
        ReadOnly.into(),
    );
    object.add_property(
        gc_context,
        "textHeight",
        Executable::Native(text_height),
        None,
        ReadOnly.into(),
    );
    object.add_property(
        gc_context,
        "multiline",
        Executable::Native(multiline),
        Some(Executable::Native(set_multiline)),
        ReadOnly.into(),
    );
    object.add_property(
        gc_context,
        "wordWrap",
        Executable::Native(word_wrap),
        Some(Executable::Native(set_word_wrap)),
        ReadOnly.into(),
    );
}
