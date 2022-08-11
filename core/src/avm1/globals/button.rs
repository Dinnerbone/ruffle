//! Button prototype

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::property_decl::{define_properties_on, Declaration};
use crate::avm1::{globals, Object, ScriptObject, TObject, Value};
use crate::display_object::{Avm1Button, TDisplayObject};
use gc_arena::MutationContext;
use ruffle_types::backend::Backend;

macro_rules! button_getter {
    ($name:ident) => {
        |activation, this, _args| {
            if let Some(display_object) = this.as_display_object() {
                if let Some(button) = display_object.as_avm1_button() {
                    return $name(button, activation);
                }
            }
            Ok(Value::Undefined)
        }
    };
}

macro_rules! button_setter {
    ($name:ident) => {
        |activation, this, args| {
            if let Some(display_object) = this.as_display_object() {
                if let Some(button) = display_object.as_avm1_button() {
                    let value = args.get(0).unwrap_or(&Value::Undefined).clone();
                    $name(button, activation, value)?;
                }
            }
            Ok(Value::Undefined)
        }
    };
}

pub fn create_proto<'gc, B: Backend>(
    gc_context: MutationContext<'gc, '_>,
    proto: Object<'gc, B>,
    fn_proto: Object<'gc, B>,
) -> Object<'gc, B> {
    let object = ScriptObject::object(gc_context, Some(proto));

    let PROTO_DECLS: &[Declaration<B>] = declare_properties! {
        "enabled" => property(button_getter!(enabled), button_setter!(set_enabled));
        "getDepth" => method(globals::get_depth; DONT_ENUM | DONT_DELETE | READ_ONLY; version(6));
        "useHandCursor" => property(button_getter!(use_hand_cursor), button_setter!(set_use_hand_cursor));
    };
    define_properties_on(PROTO_DECLS, gc_context, object, fn_proto);

    object.into()
}

/// Implements `Button` constructor.
pub fn constructor<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    Ok(this.into())
}

fn enabled<'gc, B: Backend>(
    this: Avm1Button<'gc, B>,
    _activation: &mut Activation<'_, 'gc, '_, B>,
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    Ok(this.enabled().into())
}

fn set_enabled<'gc, B: Backend>(
    this: Avm1Button<'gc, B>,
    activation: &mut Activation<'_, 'gc, '_, B>,
    value: Value<'gc, B>,
) -> Result<(), Error<'gc, B>> {
    let enabled = value.as_bool(activation.swf_version());
    this.set_enabled(&mut activation.context, enabled);
    Ok(())
}

fn use_hand_cursor<'gc, B: Backend>(
    this: Avm1Button<'gc, B>,
    _activation: &mut Activation<'_, 'gc, '_, B>,
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    Ok(this.use_hand_cursor().into())
}

fn set_use_hand_cursor<'gc, B: Backend>(
    this: Avm1Button<'gc, B>,
    activation: &mut Activation<'_, 'gc, '_, B>,
    value: Value<'gc, B>,
) -> Result<(), Error<'gc, B>> {
    let use_hand_cursor = value.as_bool(activation.swf_version());
    this.set_use_hand_cursor(&mut activation.context, use_hand_cursor);
    Ok(())
}
