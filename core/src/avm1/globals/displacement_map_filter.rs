//! flash.filters.DisplacementMapFilter object

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::object::displacement_map_filter::DisplacementMapFilterObject;
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
    set_map_bitmap(activation, this, args.get(0..1).unwrap_or_default())?;
    set_map_point(activation, this, args.get(1..2).unwrap_or_default())?;
    set_component_x(activation, this, args.get(2..3).unwrap_or_default())?;
    set_component_y(activation, this, args.get(3..4).unwrap_or_default())?;
    set_scale_x(activation, this, args.get(4..5).unwrap_or_default())?;
    set_scale_y(activation, this, args.get(5..6).unwrap_or_default())?;
    set_mode(activation, this, args.get(6..7).unwrap_or_default())?;
    set_color(activation, this, args.get(7..8).unwrap_or_default())?;
    set_alpha(activation, this, args.get(8..9).unwrap_or_default())?;

    Ok(this.into())
}

pub fn alpha<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_displacement_map_filter_object() {
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
        .unwrap_or(&0.into())
        .coerce_to_f64(activation)
        .map(|x| x.clamp(0.0, 1.0))?;

    if let Some(filter) = this.as_displacement_map_filter_object() {
        filter.set_alpha(activation.context.gc_context, alpha);
    }

    Ok(Value::Undefined)
}

pub fn color<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(object) = this.as_displacement_map_filter_object() {
        return Ok(object.color().into());
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
        .unwrap_or(&0x000000.into())
        .coerce_to_u32(activation)?;

    if let Some(object) = this.as_displacement_map_filter_object() {
        object.set_color(activation.context.gc_context, color & 0xFFFFFF);
    }

    Ok(Value::Undefined)
}

pub fn component_x<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(object) = this.as_displacement_map_filter_object() {
        return Ok(object.component_x().into());
    }

    Ok(Value::Undefined)
}

pub fn set_component_x<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let component = args.get(0).unwrap_or(&0.into()).coerce_to_i32(activation)?;

    if let Some(object) = this.as_displacement_map_filter_object() {
        object.set_component_x(activation.context.gc_context, component);
    }

    Ok(Value::Undefined)
}

pub fn component_y<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(object) = this.as_displacement_map_filter_object() {
        return Ok(object.component_y().into());
    }

    Ok(Value::Undefined)
}

pub fn set_component_y<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let component = args.get(0).unwrap_or(&0.into()).coerce_to_i32(activation)?;

    if let Some(object) = this.as_displacement_map_filter_object() {
        object.set_component_y(activation.context.gc_context, component);
    }

    Ok(Value::Undefined)
}

pub fn map_bitmap<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(object) = this.as_displacement_map_filter_object() {
        if let Some(map_bitmap) = object.map_bitmap() {
            return Ok(map_bitmap.into());
        }
    }

    Ok(Value::Undefined)
}

pub fn set_map_bitmap<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let bitmap = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .coerce_to_object(activation);

    if let Some(object) = this.as_displacement_map_filter_object() {
        if bitmap.as_bitmap_data_object().is_some() {
            object.set_map_bitmap(activation.context.gc_context, Some(bitmap));
        }
    }

    Ok(Value::Undefined)
}

pub fn map_point<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(object) = this.as_displacement_map_filter_object() {
        let (x, y) = object.map_point();

        let proto = activation.context.avm1.prototypes.point_constructor;
        let point = proto.construct(activation, &[x.into(), y.into()])?;
        return Ok(point);
    }

    Ok(Value::Undefined)
}

pub fn set_map_point<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let obj = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .coerce_to_object(activation);

    let x = obj.get("x", activation)?.coerce_to_i32(activation)?;
    let y = obj.get("y", activation)?.coerce_to_i32(activation)?;

    if let Some(object) = this.as_displacement_map_filter_object() {
        object.set_map_point(activation.context.gc_context, (x, y));
    }

    Ok(Value::Undefined)
}

pub fn mode<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(object) = this.as_displacement_map_filter_object() {
        let mode: &WStr = object.mode().into();
        return Ok(AvmString::from(mode).into());
    }

    Ok(Value::Undefined)
}

pub fn set_mode<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let mode = args
        .get(0)
        .unwrap_or(&"wrap".into())
        .coerce_to_string(activation)?;

    if let Some(object) = this.as_displacement_map_filter_object() {
        object.set_mode(activation.context.gc_context, mode.as_wstr().into());
    }

    Ok(Value::Undefined)
}

pub fn scale_x<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(object) = this.as_displacement_map_filter_object() {
        return Ok(object.scale_x().into());
    }

    Ok(Value::Undefined)
}

pub fn set_scale_x<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let scale = args.get(0).unwrap_or(&0.into()).coerce_to_f64(activation)?;

    if let Some(object) = this.as_displacement_map_filter_object() {
        object.set_scale_x(activation.context.gc_context, scale);
    }

    Ok(Value::Undefined)
}

pub fn scale_y<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(object) = this.as_displacement_map_filter_object() {
        return Ok(object.scale_y().into());
    }

    Ok(Value::Undefined)
}

pub fn set_scale_y<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let scale = args.get(0).unwrap_or(&0.into()).coerce_to_f64(activation)?;

    if let Some(object) = this.as_displacement_map_filter_object() {
        object.set_scale_y(activation.context.gc_context, scale);
    }

    Ok(Value::Undefined)
}

pub fn create_proto<'gc, B: Backend>(
    gc_context: MutationContext<'gc, '_>,
    proto: Object<'gc, B>,
    fn_proto: Object<'gc, B>,
) -> Object<'gc, B> {
    let filter = DisplacementMapFilterObject::empty_object(gc_context, Some(proto));
    let object = filter.as_script_object().unwrap();

    let PROTO_DECLS: &[Declaration<B>] = declare_properties! {
        "alpha" => property(alpha, set_alpha);
        "color" => property(color, set_color);
        "componentX" => property(component_x, set_component_x);
        "componentY" => property(component_y, set_component_y);
        "mapBitmap" => property(map_bitmap, set_map_bitmap);
        "mapPoint" => property(map_point, set_map_point);
        "mode" => property(mode, set_mode);
        "scaleX" => property(scale_x, set_scale_x);
        "scaleY" => property(scale_y, set_scale_y);
    };
    define_properties_on(PROTO_DECLS, gc_context, object, fn_proto);

    filter.into()
}
