//! `flash.media.SoundMixer` builtin/prototype

use crate::avm2::activation::Activation;
use crate::avm2::class::{Class, ClassAttributes};
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::Object;
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::display_object::SoundTransform;
use crate::display_object::SoundTransformExt;
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;

/// Implements `flash.media.SoundMixer`'s instance constructor.
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

/// Implements `flash.media.SoundMixer`'s class constructor.
pub fn class_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

/// Implements `soundTransform`'s getter
///
/// This also implements `SimpleButton`'s `soundTransform` property, as per
/// Flash Player behavior.
pub fn sound_transform<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    let dobj_st = activation.context.global_sound_transform().clone();

    Ok(dobj_st.into_avm2_object(activation)?.into())
}

/// Implements `soundTransform`'s setter
///
/// This also implements `SimpleButton`'s `soundTransform` property, as per
/// Flash Player behavior.
pub fn set_sound_transform<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    let as3_st = args
        .get(0)
        .cloned()
        .unwrap_or(Value::Undefined)
        .coerce_to_object(activation)?;
    let dobj_st = SoundTransform::from_avm2_object(activation, as3_st)?;

    activation.context.set_global_sound_transform(dobj_st);

    Ok(Value::Undefined)
}

/// Implements `SoundMixer.stopAll`
pub fn stop_all<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    activation.context.stop_all_sounds();

    Ok(Value::Undefined)
}

/// Implements `bufferTime`'s getter
pub fn buffer_time<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(activation.context.audio_manager.stream_buffer_time().into())
}

/// Implements `bufferTime`'s setter
pub fn set_buffer_time<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    let buffer_time = args
        .get(0)
        .cloned()
        .unwrap_or(Value::Undefined)
        .coerce_to_i32(activation)?;

    activation
        .context
        .audio_manager
        .set_stream_buffer_time(buffer_time);

    Ok(Value::Undefined)
}

/// Stub `SoundMixer.areSoundsInaccessible`
pub fn are_sounds_inaccessible<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("SoundMixer.areSoundsInaccessible is a stub".into())
}

/// Stub `SoundMixer.computeSpectrum`
pub fn compute_spectrum<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("SoundMixer.computeSpectrum is a stub".into())
}

/// Construct `SoundMixer`'s class.
pub fn create_class<'gc, B: Backend>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc, B>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.media"), "SoundMixer"),
        Some(QName::new(Namespace::public(), "Object").into()),
        Method::from_builtin(instance_init, "<SoundMixer instance initializer>", mc),
        Method::from_builtin(class_init, "<SoundMixer class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);

    write.set_attributes(ClassAttributes::SEALED | ClassAttributes::FINAL);

    let public_class_properties: &[(
        &str,
        Option<NativeMethodImpl<B>>,
        Option<NativeMethodImpl<B>>,
    )] = &[
        (
            "soundTransform",
            Some(sound_transform),
            Some(set_sound_transform),
        ),
        ("bufferTime", Some(buffer_time), Some(set_buffer_time)),
    ];
    write.define_public_builtin_class_properties(mc, public_class_properties);

    let public_class_methods: &[(&str, NativeMethodImpl<B>)] = &[
        ("stopAll", stop_all),
        ("areSoundsInaccessible", are_sounds_inaccessible),
        ("computeSpectrum", compute_spectrum),
    ];
    write.define_public_builtin_class_methods(mc, public_class_methods);

    class
}
