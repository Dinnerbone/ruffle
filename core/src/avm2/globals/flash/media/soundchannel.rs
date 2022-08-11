//! `flash.media.SoundChannel` builtin/prototype

use crate::avm2::activation::Activation;
use crate::avm2::class::{Class, ClassAttributes};
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::{soundchannel_allocator, Object, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::display_object::SoundTransform;
use crate::display_object::SoundTransformExt;
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;

/// Implements `flash.media.SoundChannel`'s instance constructor.
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

/// Implements `flash.media.SoundChannel`'s class constructor.
pub fn class_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

/// Stub `SoundChannel.leftPeak`
pub fn left_peak<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Sound.leftPeak is a stub.".into())
}

/// Stub `SoundChannel.rightPeak`
pub fn right_peak<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Sound.rightPeak is a stub.".into())
}

/// Impl `SoundChannel.position`
pub fn position<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(instance) = this.and_then(|this| this.as_sound_channel()) {
        return Ok(instance.position().into());
    }
    Ok(Value::Undefined)
}

/// Implements `soundTransform`'s getter
pub fn sound_transform<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(instance) = this
        .and_then(|this| this.as_sound_channel())
        .and_then(|channel| channel.instance())
    {
        let dobj_st = activation.context.local_sound_transform(instance).cloned();

        if let Some(dobj_st) = dobj_st {
            return Ok(dobj_st.into_avm2_object(activation)?.into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `soundTransform`'s setter
pub fn set_sound_transform<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(instance) = this
        .and_then(|this| this.as_sound_channel())
        .and_then(|channel| channel.instance())
    {
        let as3_st = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_object(activation)?;
        let dobj_st = SoundTransform::from_avm2_object(activation, as3_st)?;

        activation
            .context
            .set_local_sound_transform(instance, dobj_st);
    }

    Ok(Value::Undefined)
}

/// Impl `SoundChannel.stop`
pub fn stop<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(instance) = this
        .and_then(|this| this.as_sound_channel())
        .and_then(|channel| channel.instance())
    {
        activation.context.stop_sound(instance);
    }

    Ok(Value::Undefined)
}

/// Construct `SoundChannel`'s class.
pub fn create_class<'gc, B: Backend>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc, B>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.media"), "SoundChannel"),
        Some(QName::new(Namespace::package("flash.events"), "EventDispatcher").into()),
        Method::from_builtin(instance_init, "<SoundChannel instance initializer>", mc),
        Method::from_builtin(class_init, "<SoundChannel class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);

    write.set_attributes(ClassAttributes::SEALED | ClassAttributes::FINAL);
    write.set_instance_allocator(soundchannel_allocator);

    let public_instance_properties: &[(
        &str,
        Option<NativeMethodImpl<B>>,
        Option<NativeMethodImpl<B>>,
    )] = &[
        ("leftPeak", Some(left_peak), None),
        ("rightPeak", Some(right_peak), None),
        ("position", Some(position), None),
        (
            "soundTransform",
            Some(sound_transform),
            Some(set_sound_transform),
        ),
    ];
    write.define_public_builtin_instance_properties(mc, public_instance_properties);

    let public_instance_methods: &[(&str, NativeMethodImpl<B>)] = &[("stop", stop)];
    write.define_public_builtin_instance_methods(mc, public_instance_methods);

    class
}
