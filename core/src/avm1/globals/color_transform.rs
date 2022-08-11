//! flash.geom.ColorTransform object

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::object::color_transform_object::ColorTransformObject;
use crate::avm1::property_decl::{define_properties_on, Declaration};
use crate::avm1::{Object, TObject, Value};
use gc_arena::MutationContext;
use ruffle_types::backend::Backend;
use ruffle_types::color_transform::ColorTransform;
use ruffle_types::string::AvmString;
use swf::Fixed8;

pub fn constructor<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let red_multiplier = args.get(0).unwrap_or(&1.into()).coerce_to_f64(activation)?;
    let green_multiplier = args.get(1).unwrap_or(&1.into()).coerce_to_f64(activation)?;
    let blue_multiplier = args.get(2).unwrap_or(&1.into()).coerce_to_f64(activation)?;
    let alpha_multiplier = args.get(3).unwrap_or(&1.into()).coerce_to_f64(activation)?;
    let red_offset = args.get(4).unwrap_or(&0.into()).coerce_to_f64(activation)?;
    let green_offset = args.get(5).unwrap_or(&0.into()).coerce_to_f64(activation)?;
    let blue_offset = args.get(6).unwrap_or(&0.into()).coerce_to_f64(activation)?;
    let alpha_offset = args.get(7).unwrap_or(&0.into()).coerce_to_f64(activation)?;

    if let Some(ct) = this.as_color_transform_object() {
        ct.set_red_multiplier(activation.context.gc_context, red_multiplier);
        ct.set_green_multiplier(activation.context.gc_context, green_multiplier);
        ct.set_blue_multiplier(activation.context.gc_context, blue_multiplier);
        ct.set_alpha_multiplier(activation.context.gc_context, alpha_multiplier);
        ct.set_red_offset(activation.context.gc_context, red_offset);
        ct.set_green_offset(activation.context.gc_context, green_offset);
        ct.set_blue_offset(activation.context.gc_context, blue_offset);
        ct.set_alpha_offset(activation.context.gc_context, alpha_offset);
    }

    Ok(this.into())
}

pub fn object_to_color_transform<'gc, B: Backend>(
    object: Object<'gc, B>,
    activation: &mut Activation<'_, 'gc, '_, B>,
) -> Result<ColorTransform, Error<'gc, B>> {
    let red_multiplier = object
        .get("redMultiplier", activation)?
        .coerce_to_f64(activation)?;
    let green_multiplier = object
        .get("greenMultiplier", activation)?
        .coerce_to_f64(activation)?;
    let blue_multiplier = object
        .get("blueMultiplier", activation)?
        .coerce_to_f64(activation)?;
    let alpha_multiplier = object
        .get("alphaMultiplier", activation)?
        .coerce_to_f64(activation)?;
    let red_offset = object
        .get("redOffset", activation)?
        .coerce_to_i16(activation)?;
    let green_offset = object
        .get("greenOffset", activation)?
        .coerce_to_i16(activation)?;
    let blue_offset = object
        .get("blueOffset", activation)?
        .coerce_to_i16(activation)?;
    let alpha_offset = object
        .get("alphaOffset", activation)?
        .coerce_to_i16(activation)?;
    Ok(ColorTransform {
        r_mult: Fixed8::from_f64(red_multiplier),
        g_mult: Fixed8::from_f64(green_multiplier),
        b_mult: Fixed8::from_f64(blue_multiplier),
        a_mult: Fixed8::from_f64(alpha_multiplier),
        r_add: red_offset,
        g_add: green_offset,
        b_add: blue_offset,
        a_add: alpha_offset,
    })
}

pub fn color_transform_to_object<'gc, B: Backend>(
    color_transform: ColorTransform,
    activation: &mut Activation<'_, 'gc, '_, B>,
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let args = [
        color_transform.r_mult.to_f64().into(),
        color_transform.g_mult.to_f64().into(),
        color_transform.b_mult.to_f64().into(),
        color_transform.a_mult.to_f64().into(),
        color_transform.r_add.into(),
        color_transform.g_add.into(),
        color_transform.b_add.into(),
        color_transform.a_add.into(),
    ];
    let constructor = activation
        .context
        .avm1
        .prototypes
        .color_transform_constructor;
    let object = constructor.construct(activation, &args)?;
    Ok(object)
}

pub fn get_rgb<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    if let Some(ct) = this.as_color_transform_object() {
        let rgb = ((ct.get_red_offset() as u32) << 16)
            | ((ct.get_green_offset() as u32) << 8)
            | (ct.get_blue_offset() as u32);
        Ok(rgb.into())
    } else {
        Ok(Value::Undefined)
    }
}

pub fn set_rgb<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let new_rgb = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .coerce_to_u32(activation)?;
    let [b, g, r, _] = new_rgb.to_le_bytes();

    if let Some(ct) = this.as_color_transform_object() {
        ct.set_red_offset(activation.context.gc_context, r.into());
        ct.set_green_offset(activation.context.gc_context, g.into());
        ct.set_blue_offset(activation.context.gc_context, b.into());

        ct.set_red_multiplier(activation.context.gc_context, 0.0);
        ct.set_green_multiplier(activation.context.gc_context, 0.0);
        ct.set_blue_multiplier(activation.context.gc_context, 0.0);
    }

    Ok(Value::Undefined)
}

macro_rules! color_transform_value_accessor {
    ($([$get_ident: ident, $set_ident: ident],)*) => {
        $(
            pub fn $set_ident<'gc, B: Backend>(
                activation: &mut Activation<'_, 'gc, '_, B>,
                this: Object<'gc, B>,
                args: &[Value<'gc, B>],
            ) -> Result<Value<'gc, B>, Error<'gc, B>> {
                let new_val = args
                    .get(0)
                    .unwrap_or(&Value::Undefined)
                    .coerce_to_f64(activation)?;

                if let Some(ct) = this.as_color_transform_object() {
                    ct.$set_ident(activation.context.gc_context, new_val);
                }
                Ok(Value::Undefined.into())
            }

            pub fn $get_ident<'gc, B: Backend>(
                _activation: &mut Activation<'_, 'gc, '_, B>,
                this: Object<'gc, B>,
                _args: &[Value<'gc, B>],
            ) -> Result<Value<'gc, B>, Error<'gc, B>> {
                if let Some(ct) = this.as_color_transform_object() {
                    Ok(ct.$get_ident().into())
                } else {
                    Ok(Value::Undefined)
                }
            }
        )*
    }
}

color_transform_value_accessor!(
    [get_red_multiplier, set_red_multiplier],
    [get_green_multiplier, set_green_multiplier],
    [get_blue_multiplier, set_blue_multiplier],
    [get_alpha_multiplier, set_alpha_multiplier],
    [get_red_offset, set_red_offset],
    [get_green_offset, set_green_offset],
    [get_blue_offset, set_blue_offset],
    [get_alpha_offset, set_alpha_offset],
);

pub fn create_proto<'gc, B: Backend>(
    gc_context: MutationContext<'gc, '_>,
    proto: Object<'gc, B>,
    fn_proto: Object<'gc, B>,
) -> Object<'gc, B> {
    let color_transform_object =
        ColorTransformObject::empty_color_transform_object(gc_context, Some(proto));
    let object = color_transform_object.as_script_object().unwrap();

    let PROTO_DECLS: &[Declaration<B>] = declare_properties! {
        "alphaMultiplier" => property(get_alpha_multiplier, set_alpha_multiplier);
        "redMultiplier" => property(get_red_multiplier, set_red_multiplier);
        "greenMultiplier" => property(get_green_multiplier, set_green_multiplier);
        "blueMultiplier" => property(get_blue_multiplier, set_blue_multiplier);
        "alphaOffset" => property(get_alpha_offset, set_alpha_offset);
        "redOffset" => property(get_red_offset, set_red_offset);
        "greenOffset" => property(get_green_offset, set_green_offset);
        "blueOffset" => property(get_blue_offset, set_blue_offset);
        "rgb" => property(get_rgb, set_rgb);
        "concat" => method(concat);
        "toString" => method(to_string);
    };
    define_properties_on(PROTO_DECLS, gc_context, object, fn_proto);

    color_transform_object.into()
}

fn to_string<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    let formatted = format!("(redMultiplier={}, greenMultiplier={}, blueMultiplier={}, alphaMultiplier={}, redOffset={}, greenOffset={}, blueOffset={}, alphaOffset={})",
            this.get("redMultiplier", activation)?.coerce_to_string(activation)?,
            this.get("greenMultiplier", activation)?.coerce_to_string(activation)?,
            this.get("blueMultiplier", activation)?.coerce_to_string(activation)?,
            this.get("alphaMultiplier", activation)?.coerce_to_string(activation)?,
            this.get("redOffset", activation)?.coerce_to_string(activation)?,
            this.get("greenOffset", activation)?.coerce_to_string(activation)?,
            this.get("blueOffset", activation)?.coerce_to_string(activation)?,
            this.get("alphaOffset", activation)?.coerce_to_string(activation)?
    );

    Ok(AvmString::new_utf8(activation.context.gc_context, formatted).into())
}

fn concat<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Object<'gc, B>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error<'gc, B>> {
    // Without an arg, does nothing
    if let Some(arg) = _args.get(0) {
        // If given an invalid var, return undefined
        if arg == &Value::Undefined {
            return Ok(Value::Undefined);
        }
        let other = arg.coerce_to_object(activation);

        if let (Some(other_ct), Some(this_ct)) = (
            other.as_color_transform_object(),
            this.as_color_transform_object(),
        ) {
            let red_multiplier = other_ct.get_red_multiplier() * this_ct.get_red_multiplier();
            let green_multiplier = other_ct.get_green_multiplier() * this_ct.get_green_multiplier();
            let blue_multiplier = other_ct.get_blue_multiplier() * this_ct.get_blue_multiplier();
            let alpha_multiplier = other_ct.get_alpha_multiplier() * this_ct.get_alpha_multiplier();
            let red_offset = (other_ct.get_red_offset() * this_ct.get_red_multiplier())
                + this_ct.get_red_offset();
            let green_offset = (other_ct.get_green_offset() * this_ct.get_green_multiplier())
                + this_ct.get_green_offset();
            let blue_offset = (other_ct.get_blue_offset() * this_ct.get_blue_multiplier())
                + this_ct.get_blue_offset();
            let alpha_offset = (other_ct.get_alpha_offset() * this_ct.get_alpha_multiplier())
                + this_ct.get_alpha_offset();

            this_ct.set_red_multiplier(activation.context.gc_context, red_multiplier);
            this_ct.set_green_multiplier(activation.context.gc_context, green_multiplier);
            this_ct.set_blue_multiplier(activation.context.gc_context, blue_multiplier);
            this_ct.set_alpha_multiplier(activation.context.gc_context, alpha_multiplier);
            this_ct.set_red_offset(activation.context.gc_context, red_offset);
            this_ct.set_green_offset(activation.context.gc_context, green_offset);
            this_ct.set_blue_offset(activation.context.gc_context, blue_offset);
            this_ct.set_alpha_offset(activation.context.gc_context, alpha_offset);
        }
    }

    Ok(Value::Undefined)
}
