use crate::avm2::activation::Activation;
use crate::avm2::object::{Object, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::avm2::{Namespace, QName};
use crate::display_object::TDisplayObject;
use ruffle_types::backend::Backend;
use swf::Twips;

/// Implements `stageX`'s getter.
pub fn get_stage_x<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        if let Some(evt) = this.as_event() {
            let local_x = this
                .get_property(
                    &QName::new(Namespace::public(), "localX").into(),
                    activation,
                )?
                .coerce_to_number(activation)?;

            if local_x.is_nan() {
                return Ok(Value::Number(local_x));
            } else if let Some(target) = evt.target().and_then(|t| t.as_display_object()) {
                let as_twips = Twips::from_pixels(local_x);
                let xformed = target.local_to_global((as_twips, Twips::ZERO)).0;

                return Ok(Value::Number(xformed.to_pixels()));
            } else {
                return Ok(Value::Number(local_x * 0.0));
            }
        }
    }

    Ok(Value::Undefined)
}

/// Implements `stageY`'s getter.
pub fn get_stage_y<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        if let Some(evt) = this.as_event() {
            let local_y = this
                .get_property(
                    &QName::new(Namespace::public(), "localY").into(),
                    activation,
                )?
                .coerce_to_number(activation)?;

            if local_y.is_nan() {
                return Ok(Value::Number(local_y));
            } else if let Some(target) = evt.target().and_then(|t| t.as_display_object()) {
                let as_twips = Twips::from_pixels(local_y);
                let xformed = target.local_to_global((Twips::ZERO, as_twips)).1;

                return Ok(Value::Number(xformed.to_pixels()));
            } else {
                return Ok(Value::Number(local_y * 0.0));
            }
        }
    }

    Ok(Value::Undefined)
}
