//! flash.filters.BlurFilter object

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::object::blur_filter::BlurFilterObject;
use crate::avm1::property_decl::{define_properties_on, Declaration};
use crate::avm1::{Object, TObject, Value};
use gc_arena::MutationContext;
use ruffle_types::backend::Backend;

pub fn constructor<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    set_blur_x(activation, this, args.get(0..1).unwrap_or_default())?;
    set_blur_y(activation, this, args.get(1..2).unwrap_or_default())?;
    set_quality(activation, this, args.get(2..3).unwrap_or_default())?;

    Ok(this.into())
}

pub fn blur_x<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_blur_filter_object() {
        return Ok(filter.blur_x().into());
    }

    Ok(this.into())
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

    if let Some(filter) = this.as_blur_filter_object() {
        filter.set_blur_x(activation.context.gc_context, blur_x);
    }

    Ok(Value::Undefined)
}

pub fn get_blur_y<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_blur_filter_object() {
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

    if let Some(filter) = this.as_blur_filter_object() {
        filter.set_blur_y(activation.context.gc_context, blur_y);
    }

    Ok(Value::Undefined)
}

pub fn get_quality<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(filter) = this.as_blur_filter_object() {
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

    if let Some(filter) = this.as_blur_filter_object() {
        filter.set_quality(activation.context.gc_context, quality);
    }

    Ok(Value::Undefined)
}

pub fn create_proto<'gc, B: Backend>(
    gc_context: MutationContext<'gc, '_>,
    proto: Object<'gc, B>,
    fn_proto: Object<'gc, B>,
) -> Object<'gc, B> {
    let blur_filter = BlurFilterObject::empty_object(gc_context, Some(proto));
    let object = blur_filter.as_script_object().unwrap();

    let PROTO_DECLS: &[Declaration<B>] = declare_properties! {
        "blurX" => property(blur_x, set_blur_x);
        "blurY" => property(get_blur_y, set_blur_y);
        "quality" => property(get_quality, set_quality);
    };
    define_properties_on(PROTO_DECLS, gc_context, object, fn_proto);

    blur_filter.into()
}
