//! `flash.events.Event` builtin/prototype

use crate::avm2::activation::Activation;
use crate::avm2::object::{Object, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use ruffle_types::backend::Backend;

pub use crate::avm2::object::event_allocator;

pub fn init<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    let this = this.unwrap();
    let mut evt = this.as_event_mut(activation.context.gc_context).unwrap();
    evt.set_event_type(
        args.get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_string(activation)?,
    );
    evt.set_bubbles(
        args.get(1)
            .cloned()
            .unwrap_or(Value::Bool(false))
            .coerce_to_boolean(),
    );
    evt.set_cancelable(
        args.get(2)
            .cloned()
            .unwrap_or(Value::Bool(false))
            .coerce_to_boolean(),
    );
    Ok(Value::Undefined)
}

/// Implements `bubbles` property's getter
pub fn get_bubbles<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(evt) = this.unwrap().as_event() {
        return Ok(evt.is_bubbling().into());
    }

    Ok(Value::Undefined)
}

/// Implements `cancelable` property's getter
pub fn get_cancelable<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(evt) = this.unwrap().as_event() {
        return Ok(evt.is_cancelable().into());
    }

    Ok(Value::Undefined)
}

/// Implements `type` property's getter
pub fn get_type<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(evt) = this.unwrap().as_event() {
        return Ok(evt.event_type().into());
    }

    Ok(Value::Undefined)
}

/// Implements `target` property's getter
pub fn get_target<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(evt) = this.unwrap().as_event() {
        return Ok(evt.target().map(|o| o.into()).unwrap_or(Value::Null));
    }

    Ok(Value::Undefined)
}

/// Implements `currentTarget` property's getter
pub fn get_current_target<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(evt) = this.unwrap().as_event() {
        return Ok(evt
            .current_target()
            .map(|o| o.into())
            .unwrap_or(Value::Null));
    }

    Ok(Value::Undefined)
}

/// Implements `eventPhase` property's getter
pub fn get_event_phase<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(evt) = this.unwrap().as_event() {
        let event_phase = evt.phase() as u32;
        return Ok(event_phase.into());
    }

    Ok(Value::Undefined)
}

/// Implements `isDefaultPrevented`
pub fn is_default_prevented<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(evt) = this.unwrap().as_event() {
        return Ok(evt.is_cancelled().into());
    }

    Ok(Value::Undefined)
}

/// Implements `preventDefault`
pub fn prevent_default<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mut evt) = this.unwrap().as_event_mut(activation.context.gc_context) {
        evt.cancel();
    }

    Ok(Value::Undefined)
}

/// Implements `stopPropagation`
pub fn stop_propagation<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mut evt) = this.unwrap().as_event_mut(activation.context.gc_context) {
        evt.stop_propagation();
    }

    Ok(Value::Undefined)
}

/// Implements `stopImmediatePropagation`
pub fn stop_immediate_propagation<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mut evt) = this.unwrap().as_event_mut(activation.context.gc_context) {
        evt.stop_immediate_propagation();
    }

    Ok(Value::Undefined)
}
