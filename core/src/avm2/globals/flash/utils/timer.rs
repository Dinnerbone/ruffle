//! `flash.utils.Timer` native methods

use crate::avm2::activation::Activation;
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::TObject;
use crate::avm2::value::Value;
use crate::avm2::{Error, Object};
use crate::timer::TimerCallback;
use ruffle_types::backend::Backend;

/// Implements `Timer.stop`
pub fn stop<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    let mut this = this.expect("`this` should be set in native method!");
    let id = this
        .get_property(
            &QName::new(Namespace::Private("".into()), "_timerId").into(),
            activation,
        )
        .unwrap()
        .coerce_to_i32(activation)?;

    if id != -1 {
        activation.context.timers.remove(id);
        this.set_property(
            &QName::new(Namespace::Private("".into()), "_timerId").into(),
            (-1).into(),
            activation,
        )?;
    }

    Ok(Value::Undefined)
}

/// Implements `Timer.start`
pub fn start<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    let mut this = this.expect("`this` should be set in native method!");
    let id = this
        .get_property(
            &QName::new(Namespace::Private("".into()), "_timerId").into(),
            activation,
        )
        .unwrap()
        .coerce_to_i32(activation)?;

    let delay = this
        .get_property(
            &QName::new(Namespace::Private("".into()), "_delay").into(),
            activation,
        )
        .unwrap()
        .coerce_to_number(activation)?;

    if id == -1 {
        let on_update = this
            .get_property(
                &QName::new(Namespace::Private("".into()), "onUpdate").into(),
                activation,
            )?
            .coerce_to_object(activation)?;
        let id = activation.context.timers.add_timer(
            TimerCallback::Avm2Callback(on_update),
            delay as _,
            false,
        );
        this.set_property(
            &QName::new(Namespace::Private("".into()), "_timerId").into(),
            id.into(),
            activation,
        )?;
    }
    Ok(Value::Undefined)
}
