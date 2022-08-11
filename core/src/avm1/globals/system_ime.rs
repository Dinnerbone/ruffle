use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::globals::as_broadcaster::BroadcasterFunctions;
use crate::avm1::object::Object;
use crate::avm1::property_decl::{define_properties_on, Declaration};
use crate::avm1::{ScriptObject, Value};
use gc_arena::MutationContext;
use ruffle_types::backend::Backend;

fn on_ime_composition<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    Ok(false.into())
}

fn do_conversion<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    Ok(true.into())
}

fn get_conversion_mode<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    Ok("KOREAN".into())
}

fn get_enabled<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    Ok(false.into())
}

fn set_composition_string<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    Ok(false.into())
}

fn set_conversion_mode<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    Ok(false.into())
}

fn set_enabled<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    Ok(false.into())
}

pub fn create<'gc, B: Backend>(
    gc_context: MutationContext<'gc, '_>,
    proto: Option<Object<'gc, B>>,
    fn_proto: Object<'gc, B>,
    broadcaster_functions: BroadcasterFunctions<'gc, B>,
    array_proto: Object<'gc, B>,
) -> Object<'gc, B> {
    let ime = ScriptObject::object(gc_context, proto);
    broadcaster_functions.initialize(gc_context, ime.into(), array_proto);

    let OBJECT_DECLS: &[Declaration<B>] = declare_properties! {
        "ALPHANUMERIC_FULL" => string("ALPHANUMERIC_FULL"; DONT_ENUM | DONT_DELETE | READ_ONLY);
        "ALPHANUMERIC_HALF" => string("ALPHANUMERIC_HALF"; DONT_ENUM | DONT_DELETE | READ_ONLY);
        "CHINESE" => string("CHINESE"; DONT_ENUM | DONT_DELETE | READ_ONLY);
        "JAPANESE_HIRAGANA" => string("JAPANESE_HIRAGANA"; DONT_ENUM | DONT_DELETE | READ_ONLY);
        "JAPENESE_KATAKANA_FULL" => string("JAPENESE_KATAKANA_FULL"; DONT_ENUM | DONT_DELETE | READ_ONLY);
        "KOREAN" => string("KOREAN"; DONT_ENUM | DONT_DELETE | READ_ONLY);
        "UNKNOWN" => string("UNKNOWN"; DONT_ENUM | DONT_DELETE | READ_ONLY);
        "onIMEComposition" => method(on_ime_composition; DONT_ENUM | DONT_DELETE | READ_ONLY);
        "doConversion" => method(do_conversion; DONT_ENUM | DONT_DELETE | READ_ONLY);
        "getConversionMode" => method(get_conversion_mode; DONT_ENUM | DONT_DELETE | READ_ONLY);
        "getEnabled" => method(get_enabled; DONT_ENUM | DONT_DELETE | READ_ONLY);
        "setCompositionString" => method(set_composition_string; DONT_ENUM | DONT_DELETE | READ_ONLY);
        "setConversionMode" => method(set_conversion_mode; DONT_ENUM | DONT_DELETE | READ_ONLY);
        "setEnabled" => method(set_enabled; DONT_ENUM | DONT_DELETE | READ_ONLY);
    };
    define_properties_on(OBJECT_DECLS, gc_context, ime, fn_proto);

    ime.into()
}
