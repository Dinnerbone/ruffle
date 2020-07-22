//! flash.geom.Point

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::function::{Executable, FunctionObject};
use crate::avm1::property::Attribute;
use crate::avm1::{AvmString, Object, ScriptObject, TObject, Value};
use enumset::EnumSet;
use gc_arena::MutationContext;
use std::f64::NAN;

pub fn point_to_object<'gc>(
    point: (f64, f64),
    activation: &mut Activation<'_, '_, 'gc, '_>,
) -> Result<Object<'gc>, Error<'gc>> {
    let args = [point.0.into(), point.1.into()];
    construct_new_point(&args, activation)
}

pub fn construct_new_point<'gc>(
    args: &[Value<'gc>],
    activation: &mut Activation<'_, '_, 'gc, '_>,
) -> Result<Object<'gc>, Error<'gc>> {
    let proto = activation.context.system_prototypes.point;
    let object = proto.new(activation, proto, &args)?;
    let _ = constructor(activation, object, &args)?;
    Ok(object)
}

pub fn value_to_point<'gc>(
    value: Value<'gc>,
    activation: &mut Activation<'_, '_, 'gc, '_>,
) -> Result<(f64, f64), Error<'gc>> {
    let x = value
        .coerce_to_object(activation)
        .get("x", activation)?
        .coerce_to_f64(activation)?;
    let y = value
        .coerce_to_object(activation)
        .get("y", activation)?
        .coerce_to_f64(activation)?;
    Ok((x, y))
}

pub fn object_to_point<'gc>(
    object: Object<'gc>,
    activation: &mut Activation<'_, '_, 'gc, '_>,
) -> Result<(f64, f64), Error<'gc>> {
    let x = object.get("x", activation)?.coerce_to_f64(activation)?;
    let y = object.get("y", activation)?.coerce_to_f64(activation)?;
    Ok((x, y))
}

fn constructor<'gc>(
    activation: &mut Activation<'_, '_, 'gc, '_>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if args.is_empty() {
        this.set("x", 0.into(), activation)?;
        this.set("y", 0.into(), activation)?;
    } else {
        this.set(
            "x",
            args.get(0).unwrap_or(&Value::Undefined).to_owned(),
            activation,
        )?;
        this.set(
            "y",
            args.get(1).unwrap_or(&Value::Undefined).to_owned(),
            activation,
        )?;
    }

    Ok(Value::Undefined)
}

fn clone<'gc>(
    activation: &mut Activation<'_, '_, 'gc, '_>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let proto = activation.context.system_prototypes.point;
    let args = [this.get("x", activation)?, this.get("y", activation)?];
    let cloned = proto.new(activation, proto, &args)?;
    let _ = constructor(activation, cloned, &args)?;

    Ok(cloned.into())
}

fn equals<'gc>(
    activation: &mut Activation<'_, '_, 'gc, '_>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(other) = args.get(0) {
        let this_x = this.get("x", activation)?;
        let this_y = this.get("y", activation)?;
        let other = other.coerce_to_object(activation);
        let other_x = other.get("x", activation)?;
        let other_y = other.get("y", activation)?;
        return Ok((this_x == other_x && this_y == other_y).into());
    }

    Ok(false.into())
}

fn add<'gc>(
    activation: &mut Activation<'_, '_, 'gc, '_>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let this_x = this.get("x", activation)?.coerce_to_f64(activation)?;
    let this_y = this.get("y", activation)?.coerce_to_f64(activation)?;
    let other = value_to_point(
        args.get(0).unwrap_or(&Value::Undefined).to_owned(),
        activation,
    )?;
    let object = point_to_object((this_x + other.0, this_y + other.1), activation)?;
    Ok(object.into())
}

fn subtract<'gc>(
    activation: &mut Activation<'_, '_, 'gc, '_>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let this_x = this.get("x", activation)?.coerce_to_f64(activation)?;
    let this_y = this.get("y", activation)?.coerce_to_f64(activation)?;
    let other = value_to_point(
        args.get(0).unwrap_or(&Value::Undefined).to_owned(),
        activation,
    )?;
    let object = point_to_object((this_x - other.0, this_y - other.1), activation)?;
    Ok(object.into())
}

fn distance<'gc>(
    activation: &mut Activation<'_, '_, 'gc, '_>,
    _this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if args.len() < 2 {
        return Ok(NAN.into());
    }

    let a = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .coerce_to_object(activation);
    let b = args.get(1).unwrap_or(&Value::Undefined);
    let delta = a.call_method("subtract", &[b.to_owned()], activation)?;
    Ok(delta
        .coerce_to_object(activation)
        .get("length", activation)?)
}

fn polar<'gc>(
    activation: &mut Activation<'_, '_, 'gc, '_>,
    _this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let length = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .coerce_to_f64(activation)?;
    let angle = args
        .get(1)
        .unwrap_or(&Value::Undefined)
        .coerce_to_f64(activation)?;
    let point = point_to_object((length * angle.cos(), length * angle.sin()), activation)?;
    Ok(point.into())
}

fn interpolate<'gc>(
    activation: &mut Activation<'_, '_, 'gc, '_>,
    _this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if args.len() < 3 {
        return Ok(point_to_object((NAN, NAN), activation)?.into());
    }

    let a = value_to_point(args.get(0).unwrap().to_owned(), activation)?;
    let b = value_to_point(args.get(1).unwrap().to_owned(), activation)?;
    let f = args.get(2).unwrap().coerce_to_f64(activation)?;
    let result = (b.0 - (b.0 - a.0) * f, b.1 - (b.1 - a.1) * f);
    Ok(point_to_object(result, activation)?.into())
}

fn to_string<'gc>(
    activation: &mut Activation<'_, '_, 'gc, '_>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let x = this.get("x", activation)?;
    let y = this.get("y", activation)?;

    Ok(AvmString::new(
        activation.context.gc_context,
        format!(
            "(x={}, y={})",
            x.coerce_to_string(activation)?,
            y.coerce_to_string(activation)?
        ),
    )
    .into())
}

fn length<'gc>(
    activation: &mut Activation<'_, '_, 'gc, '_>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let point = value_to_point(this.into(), activation)?;
    let length = (point.0 * point.0 + point.1 * point.1).sqrt();
    Ok(length.into())
}

fn normalize<'gc>(
    activation: &mut Activation<'_, '_, 'gc, '_>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let current_length = this.get("length", activation)?.coerce_to_f64(activation)?;
    if current_length.is_finite() {
        let point = object_to_point(this, activation)?;
        let new_length = args
            .get(0)
            .unwrap_or(&Value::Undefined)
            .coerce_to_f64(activation)?;
        let (x, y) = if current_length == 0.0 {
            (point.0 * new_length, point.1 * new_length)
        } else {
            (
                point.0 / current_length * new_length,
                point.1 / current_length * new_length,
            )
        };

        this.set("x", x.into(), activation)?;
        this.set("y", y.into(), activation)?;
    }

    Ok(Value::Undefined)
}

fn offset<'gc>(
    activation: &mut Activation<'_, '_, 'gc, '_>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let point = value_to_point(this.into(), activation)?;
    let dx = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .coerce_to_f64(activation)?;
    let dy = args
        .get(1)
        .unwrap_or(&Value::Undefined)
        .coerce_to_f64(activation)?;

    this.set("x", (point.0 + dx).into(), activation)?;
    this.set("y", (point.1 + dy).into(), activation)?;

    Ok(Value::Undefined)
}

pub fn create_point_object<'gc>(
    gc_context: MutationContext<'gc, '_>,
    point_proto: Option<Object<'gc>>,
    fn_proto: Option<Object<'gc>>,
) -> Object<'gc> {
    let point = FunctionObject::function(
        gc_context,
        Executable::Native(constructor),
        fn_proto,
        point_proto,
    );
    let mut object = point.as_script_object().unwrap();

    object.force_set_function("distance", distance, gc_context, EnumSet::empty(), fn_proto);
    object.force_set_function("polar", polar, gc_context, EnumSet::empty(), fn_proto);
    object.force_set_function(
        "interpolate",
        interpolate,
        gc_context,
        EnumSet::empty(),
        fn_proto,
    );

    point
}

pub fn create_proto<'gc>(
    gc_context: MutationContext<'gc, '_>,
    proto: Object<'gc>,
    fn_proto: Object<'gc>,
) -> Object<'gc> {
    let mut object = ScriptObject::object(gc_context, Some(proto));

    object.force_set_function(
        "toString",
        to_string,
        gc_context,
        EnumSet::empty(),
        Some(fn_proto),
    );

    object.force_set_function("clone", clone, gc_context, EnumSet::empty(), Some(fn_proto));

    object.force_set_function(
        "equals",
        equals,
        gc_context,
        EnumSet::empty(),
        Some(fn_proto),
    );

    object.force_set_function("add", add, gc_context, EnumSet::empty(), Some(fn_proto));

    object.force_set_function(
        "subtract",
        subtract,
        gc_context,
        EnumSet::empty(),
        Some(fn_proto),
    );

    object.force_set_function(
        "normalize",
        normalize,
        gc_context,
        EnumSet::empty(),
        Some(fn_proto),
    );

    object.force_set_function(
        "offset",
        offset,
        gc_context,
        EnumSet::empty(),
        Some(fn_proto),
    );

    object.add_property(
        gc_context,
        "length",
        Executable::Native(length),
        None,
        Attribute::ReadOnly.into(),
    );

    object.into()
}
