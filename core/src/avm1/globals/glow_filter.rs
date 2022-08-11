//! flash.filters.GlowFilter object

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::object::glow_filter::GlowFilterObject;
use crate::avm1::property_decl::{define_properties_on, Declaration};
use crate::avm1::{Object, TObject, Value};
use gc_arena::MutationContext;
use ruffle_types::backend::Backend;

pub fn constructor<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    set_color(activation, this, args.get(0..1).unwrap_or_default())?;
    set_alpha(activation, this, args.get(1..2).unwrap_or_default())?;
    set_blur_x(activation, this, args.get(2..3).unwrap_or_default())?;
    set_blur_y(activation, this, args.get(3..4).unwrap_or_default())?;
    set_strength(activation, this, args.get(4..5).unwrap_or_default())?;
    set_quality(activation, this, args.get(5..6).unwrap_or_default())?;

    Ok(this.into())
}

pub fn alpha<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_glow_filter_object() {
        return Ok(filter.alpha().into());
    }

    Ok(Value::Undefined)
}

pub fn set_alpha<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let alpha = args
        .get(0)
        .unwrap_or(&1.into())
        .coerce_to_f64(activation)
        .map(|x| x.clamp(0.0, 1.0))?;

    if let Some(filter) = this.as_glow_filter_object() {
        filter.set_alpha(activation.context.gc_context, alpha);
    }

    Ok(Value::Undefined)
}

pub fn blur_x<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_glow_filter_object() {
        return Ok(filter.blur_x().into());
    }

    Ok(Value::Undefined)
}

pub fn set_blur_x<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let blur_x = args
        .get(0)
        .unwrap_or(&6.into())
        .coerce_to_f64(activation)
        .map(|x| x.clamp(0.0, 255.0))?;

    if let Some(filter) = this.as_glow_filter_object() {
        filter.set_blur_x(activation.context.gc_context, blur_x);
    }

    Ok(Value::Undefined)
}

pub fn blur_y<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_glow_filter_object() {
        return Ok(filter.blur_y().into());
    }

    Ok(Value::Undefined)
}

pub fn set_blur_y<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let blur_y = args
        .get(0)
        .unwrap_or(&6.into())
        .coerce_to_f64(activation)
        .map(|x| x.clamp(0.0, 255.0))?;

    if let Some(filter) = this.as_glow_filter_object() {
        filter.set_blur_y(activation.context.gc_context, blur_y);
    }

    Ok(Value::Undefined)
}

pub fn color<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_glow_filter_object() {
        return Ok(filter.color().into());
    }

    Ok(Value::Undefined)
}

pub fn set_color<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let color = args
        .get(0)
        .unwrap_or(&0xFF0000.into())
        .coerce_to_i32(activation)
        .map(|x| x.clamp(1, 0xFFFFFF))?;

    if let Some(filter) = this.as_glow_filter_object() {
        filter.set_color(activation.context.gc_context, color);
    }

    Ok(Value::Undefined)
}

pub fn inner<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_glow_filter_object() {
        return Ok(filter.inner().into());
    }

    Ok(Value::Undefined)
}

pub fn set_inner<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let inner = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .as_bool(activation.swf_version());

    if let Some(filter) = this.as_glow_filter_object() {
        filter.set_inner(activation.context.gc_context, inner);
    }

    Ok(Value::Undefined)
}

pub fn knockout<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_glow_filter_object() {
        return Ok(filter.knockout().into());
    }

    Ok(Value::Undefined)
}

pub fn set_knockout<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let knockout = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .as_bool(activation.swf_version());

    if let Some(filter) = this.as_glow_filter_object() {
        filter.set_knockout(activation.context.gc_context, knockout);
    }

    Ok(Value::Undefined)
}

pub fn quality<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_glow_filter_object() {
        return Ok(filter.quality().into());
    }

    Ok(Value::Undefined)
}

pub fn set_quality<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let quality = args
        .get(0)
        .unwrap_or(&1.into())
        .coerce_to_i32(activation)
        .map(|x| x.clamp(0, 15))?;

    if let Some(filter) = this.as_glow_filter_object() {
        filter.set_quality(activation.context.gc_context, quality);
    }

    Ok(Value::Undefined)
}

pub fn strength<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_glow_filter_object() {
        return Ok(filter.strength().into());
    }

    Ok(Value::Undefined)
}

pub fn set_strength<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let strength = args
        .get(0)
        .unwrap_or(&2.into())
        .coerce_to_f64(activation)
        .map(|x| x.clamp(0.0, 255.0))?;

    if let Some(filter) = this.as_glow_filter_object() {
        filter.set_strength(activation.context.gc_context, strength);
    }

    Ok(Value::Undefined)
}

pub fn create_proto<'gc, B: Backend>(
    gc_context: MutationContext<'gc, '_>,
    proto: Object<'gc, B>,
    fn_proto: Object<'gc, B>,
) -> Object<'gc, B> {
    let glow_filter = GlowFilterObject::empty_object(gc_context, Some(proto));
    let object = glow_filter.as_script_object().unwrap();

    let PROTO_DECLS: &[Declaration<B>] = declare_properties! {
        "alpha" => property(alpha, set_alpha);
        "blurX" => property(blur_x, set_blur_x);
        "blurY" => property(blur_y, set_blur_y);
        "color" => property(color, set_color);
        "inner" => property(inner, set_inner);
        "knockout" => property(knockout, set_knockout);
        "quality" => property(quality, set_quality);
        "strength" => property(strength, set_strength);
    };
    define_properties_on(PROTO_DECLS, gc_context, object, fn_proto);

    glow_filter.into()
}
