//! flash.filters.BevelFilter object

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::object::bevel_filter::{BevelFilterObject, BevelFilterType};
use crate::avm1::property_decl::{define_properties_on, Declaration};
use crate::avm1::{Object, TObject, Value};
use gc_arena::MutationContext;
use ruffle_types::backend::Backend;
use ruffle_types::string::{AvmString, WStr};

pub fn constructor<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    set_distance(activation, this, args.get(0..1).unwrap_or_default())?;
    set_angle(activation, this, args.get(1..2).unwrap_or_default())?;
    set_highlight_color(activation, this, args.get(2..3).unwrap_or_default())?;
    set_highlight_alpha(activation, this, args.get(3..4).unwrap_or_default())?;
    set_shadow_color(activation, this, args.get(4..5).unwrap_or_default())?;
    set_shadow_alpha(activation, this, args.get(5..6).unwrap_or_default())?;
    set_blur_x(activation, this, args.get(6..7).unwrap_or_default())?;
    set_blur_y(activation, this, args.get(7..8).unwrap_or_default())?;
    set_strength(activation, this, args.get(8..9).unwrap_or_default())?;
    set_quality(activation, this, args.get(9..10).unwrap_or_default())?;
    set_type(activation, this, args.get(10..11).unwrap_or_default())?;
    set_knockout(activation, this, args.get(11..12).unwrap_or_default())?;

    Ok(this.into())
}

pub fn distance<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_bevel_filter_object() {
        return Ok(filter.distance().into());
    }

    Ok(Value::Undefined)
}

pub fn set_distance<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let distance = args.get(0).unwrap_or(&4.into()).coerce_to_f64(activation)?;

    if let Some(filter) = this.as_bevel_filter_object() {
        filter.set_distance(activation.context.gc_context, distance);
    }

    Ok(Value::Undefined)
}

pub fn angle<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_bevel_filter_object() {
        return Ok(filter.angle().into());
    }

    Ok(Value::Undefined)
}

pub fn set_angle<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let angle = args
        .get(0)
        .unwrap_or(&44.9999999772279.into())
        .coerce_to_f64(activation)?;

    let clamped_angle = if angle.is_sign_negative() {
        -(angle.abs() % 360.0)
    } else {
        angle % 360.0
    };

    if let Some(filter) = this.as_bevel_filter_object() {
        filter.set_angle(activation.context.gc_context, clamped_angle);
    }

    Ok(Value::Undefined)
}

pub fn highlight_color<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_bevel_filter_object() {
        return Ok(filter.highlight_color().into());
    }

    Ok(Value::Undefined)
}

pub fn set_highlight_color<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let highlight_color = args
        .get(0)
        .unwrap_or(&0xFFFFFF.into())
        .coerce_to_u32(activation)?;

    if let Some(filter) = this.as_bevel_filter_object() {
        filter.set_highlight_color(activation.context.gc_context, highlight_color & 0xFFFFFF);
    }

    Ok(Value::Undefined)
}

pub fn highlight_alpha<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_bevel_filter_object() {
        return Ok(filter.highlight_alpha().into());
    }

    Ok(Value::Undefined)
}

pub fn set_highlight_alpha<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let highlight_alpha = args
        .get(0)
        .unwrap_or(&1.into())
        .coerce_to_f64(activation)
        .map(|x| x.clamp(0.0, 1.0))?;

    if let Some(filter) = this.as_bevel_filter_object() {
        filter.set_highlight_alpha(activation.context.gc_context, highlight_alpha);
    }

    Ok(Value::Undefined)
}

pub fn shadow_color<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_bevel_filter_object() {
        return Ok(filter.shadow_color().into());
    }

    Ok(Value::Undefined)
}

pub fn set_shadow_color<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let color = args
        .get(0)
        .unwrap_or(&0x000000.into())
        .coerce_to_u32(activation)?;

    if let Some(filter) = this.as_bevel_filter_object() {
        filter.set_shadow_color(activation.context.gc_context, color & 0xFFFFFF);
    }

    Ok(Value::Undefined)
}

pub fn shadow_alpha<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_bevel_filter_object() {
        return Ok(filter.shadow_alpha().into());
    }

    Ok(Value::Undefined)
}

pub fn set_shadow_alpha<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let shadow_alpha = args
        .get(0)
        .unwrap_or(&1.into())
        .coerce_to_f64(activation)
        .map(|x| x.clamp(0.0, 1.0))?;

    if let Some(filter) = this.as_bevel_filter_object() {
        filter.set_shadow_alpha(activation.context.gc_context, shadow_alpha);
    }

    Ok(Value::Undefined)
}

pub fn quality<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_bevel_filter_object() {
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

    if let Some(filter) = this.as_bevel_filter_object() {
        filter.set_quality(activation.context.gc_context, quality);
    }

    Ok(Value::Undefined)
}

pub fn strength<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_bevel_filter_object() {
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
        .unwrap_or(&1.into())
        .coerce_to_f64(activation)
        .map(|x| x.clamp(0.0, 255.0))?;

    if let Some(filter) = this.as_bevel_filter_object() {
        filter.set_strength(activation.context.gc_context, strength);
    }

    Ok(Value::Undefined)
}

pub fn knockout<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_bevel_filter_object() {
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
        .unwrap_or(&false.into())
        .as_bool(activation.swf_version());

    if let Some(filter) = this.as_bevel_filter_object() {
        filter.set_knockout(activation.context.gc_context, knockout);
    }

    Ok(Value::Undefined)
}

pub fn blur_x<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_bevel_filter_object() {
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
        .unwrap_or(&4.into())
        .coerce_to_f64(activation)
        .map(|x| x.clamp(0.0, 255.0))?;

    if let Some(filter) = this.as_bevel_filter_object() {
        filter.set_blur_x(activation.context.gc_context, blur_x);
    }

    Ok(Value::Undefined)
}

pub fn blur_y<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_bevel_filter_object() {
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
        .unwrap_or(&4.into())
        .coerce_to_f64(activation)
        .map(|x| x.clamp(0.0, 255.0))?;

    if let Some(filter) = this.as_bevel_filter_object() {
        filter.set_blur_y(activation.context.gc_context, blur_y);
    }

    Ok(Value::Undefined)
}

pub fn get_type<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_bevel_filter_object() {
        let type_: &WStr = filter.get_type().into();
        return Ok(AvmString::from(type_).into());
    }

    Ok(Value::Undefined)
}

pub fn set_type<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let type_: BevelFilterType = args
        .get(0)
        .unwrap_or(&"inner".into())
        .coerce_to_string(activation)
        .map(|s| s.as_wstr().into())?;

    if let Some(filter) = this.as_bevel_filter_object() {
        filter.set_type(activation.context.gc_context, type_);
    }

    Ok(Value::Undefined)
}

pub fn create_proto<'gc, B: Backend>(
    gc_context: MutationContext<'gc, '_>,
    proto: Object<'gc, B>,
    fn_proto: Object<'gc, B>,
) -> Object<'gc, B> {
    let object = BevelFilterObject::empty_object(gc_context, Some(proto));
    let script_object = object.as_script_object().unwrap();

    let PROTO_DECLS: &[Declaration<B>] = declare_properties! {
        "distance" => property(distance, set_distance);
        "angle" => property(angle, set_angle);
        "highlightColor" => property(highlight_color, set_highlight_color);
        "highlightAlpha" => property(highlight_alpha, set_highlight_alpha);
        "shadowColor" => property(shadow_color, set_shadow_color);
        "shadowAlpha" => property(shadow_alpha, set_shadow_alpha);
        "quality" => property(quality, set_quality);
        "strength" => property(strength, set_strength);
        "knockout" => property(knockout, set_knockout);
        "blurX" => property(blur_x, set_blur_x);
        "blurY" => property(blur_y, set_blur_y);
        "type" => property(get_type, set_type);
    };
    define_properties_on(PROTO_DECLS, gc_context, script_object, fn_proto);

    object.into()
}
