//! flash.geom.Point

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::function::{Executable, ExecutionReason, FunctionObject};
use crate::avm1::property_decl::{define_properties_on, Declaration};
use crate::avm1::{Object, ScriptObject, TObject, Value};
use gc_arena::MutationContext;
use ruffle_types::backend::Backend;
use ruffle_types::string::AvmString;

pub fn point_to_object<'gc, B: Backend>(
    point: (f64, f64),
    activation: &mut Activation<'_, 'gc, '_, B>,
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let args = [point.0.into(), point.1.into()];
    construct_new_point(&args, activation)
}

pub fn construct_new_point<'gc, B: Backend>(
    args: &[Value<'gc, B>],
    activation: &mut Activation<'_, 'gc, '_, B>,
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let constructor = activation.context.avm1.prototypes.point_constructor;
    let object = constructor.construct(activation, args)?;
    Ok(object)
}

pub fn value_to_point<'gc, B: Backend>(
    value: Value<'gc, B>,
    activation: &mut Activation<'_, 'gc, '_, B>,
) -> Result<(f64, f64), Error<'gc, B>> {
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

pub fn object_to_point<'gc, B: Backend>(
    object: Object<'gc, B>,
    activation: &mut Activation<'_, 'gc, '_, B>,
) -> Result<(f64, f64), Error<'gc, B>> {
    let x = object.get("x", activation)?.coerce_to_f64(activation)?;
    let y = object.get("y", activation)?.coerce_to_f64(activation)?;
    Ok((x, y))
}

fn constructor<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
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

    Ok(this.into())
}

fn clone<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let args = [this.get("x", activation)?, this.get("y", activation)?];
    let constructor = activation.context.avm1.prototypes.point_constructor;
    let cloned = constructor.construct(activation, &args)?;

    Ok(cloned)
}

fn equals<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
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

fn add<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let this_x = this.get("x", activation)?.coerce_to_f64(activation)?;
    let this_y = this.get("y", activation)?.coerce_to_f64(activation)?;
    let other = value_to_point(
        args.get(0).unwrap_or(&Value::Undefined).to_owned(),
        activation,
    )?;
    let object = point_to_object((this_x + other.0, this_y + other.1), activation)?;
    Ok(object)
}

fn subtract<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let this_x = this.get("x", activation)?.coerce_to_f64(activation)?;
    let this_y = this.get("y", activation)?.coerce_to_f64(activation)?;
    let other = value_to_point(
        args.get(0).unwrap_or(&Value::Undefined).to_owned(),
        activation,
    )?;
    let object = point_to_object((this_x - other.0, this_y - other.1), activation)?;
    Ok(object)
}

fn distance<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if args.len() < 2 {
        return Ok(f64::NAN.into());
    }

    let a = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .coerce_to_object(activation);
    let b = args.get(1).unwrap_or(&Value::Undefined);
    let delta = a.call_method(
        "subtract".into(),
        &[b.to_owned()],
        activation,
        ExecutionReason::FunctionCall,
    )?;
    delta.coerce_to_object(activation).get("length", activation)
}

fn polar<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let length = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .coerce_to_f64(activation)?;
    let angle = args
        .get(1)
        .unwrap_or(&Value::Undefined)
        .coerce_to_f64(activation)?;
    let point = point_to_object((length * angle.cos(), length * angle.sin()), activation)?;
    Ok(point)
}

fn interpolate<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if args.len() < 3 {
        return point_to_object((f64::NAN, f64::NAN), activation);
    }

    let a = value_to_point(args.get(0).unwrap().to_owned(), activation)?;
    let b = value_to_point(args.get(1).unwrap().to_owned(), activation)?;
    let f = args.get(2).unwrap().coerce_to_f64(activation)?;
    let result = (b.0 - (b.0 - a.0) * f, b.1 - (b.1 - a.1) * f);
    point_to_object(result, activation)
}

fn to_string<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let x = this.get("x", activation)?;
    let y = this.get("y", activation)?;

    Ok(AvmString::new_utf8(
        activation.context.gc_context,
        format!(
            "(x={}, y={})",
            x.coerce_to_string(activation)?,
            y.coerce_to_string(activation)?
        ),
    )
    .into())
}

fn length<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let point = value_to_point(this.into(), activation)?;
    let length = (point.0 * point.0 + point.1 * point.1).sqrt();
    Ok(length.into())
}

fn normalize<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
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

fn offset<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
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

pub fn create_point_object<'gc, B: Backend>(
    gc_context: MutationContext<'gc, '_>,
    point_proto: Object<'gc, B>,
    fn_proto: Object<'gc, B>,
) -> Object<'gc, B> {
    let point = FunctionObject::constructor(
        gc_context,
        Executable::Native(constructor),
        constructor_to_fn!(constructor),
        Some(fn_proto),
        point_proto,
    );
    let object = point.as_script_object().unwrap();

    let OBJECT_DECLS: &[Declaration<B>] = declare_properties! {
        "distance" => method(distance);
        "polar" => method(polar);
        "interpolate" => method(interpolate);
    };
    define_properties_on(OBJECT_DECLS, gc_context, object, fn_proto);

    point
}

pub fn create_proto<'gc, B: Backend>(
    gc_context: MutationContext<'gc, '_>,
    proto: Object<'gc, B>,
    fn_proto: Object<'gc, B>,
) -> Object<'gc, B> {
    let object = ScriptObject::object(gc_context, Some(proto));

    let PROTO_DECLS: &[Declaration<B>] = declare_properties! {
        "toString" => method(to_string);
        "clone" => method(clone);
        "equals" => method(equals);
        "add" => method(add);
        "subtract" => method(subtract);
        "normalize" => method(normalize);
        "offset" => method(offset);
        "length" => property(length; READ_ONLY);
    };
    define_properties_on(PROTO_DECLS, gc_context, object, fn_proto);

    object.into()
}
