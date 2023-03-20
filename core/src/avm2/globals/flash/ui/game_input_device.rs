use crate::avm2::{Activation, Error, Object, TObject, Value};
use crate::string::AvmString;
use crate::{avm2_stub_getter, avm2_stub_method, avm2_stub_setter};

// Note that Flash allows anyone to `new GameInputDevice()`.
// The object created is fairly useless, so the methods will return nonsense values,
// but we can't just assume that `this` will always be a `GamepadObject`.

/// Implements `enabled`'s getter
pub fn get_enabled<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_getter!(activation, "flash.ui.GameInputDevice", "enabled");
    Ok(Value::Bool(false))
}

/// Implements `enabled`'s setter
pub fn set_enabled<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_setter!(activation, "flash.ui.GameInputDevice", "enabled");
    Ok(Value::Undefined)
}

/// Implements `sampleInterval`'s getter
pub fn get_sample_interval<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_getter!(activation, "flash.ui.GameInputDevice", "sampleInterval");
    Ok(Value::Integer(0))
}

/// Implements `sampleInterval`'s setter
pub fn set_sample_interval<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_setter!(activation, "flash.ui.GameInputDevice", "sampleInterval");
    Ok(Value::Undefined)
}

/// Implements `id`'s getter
pub fn get_id<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(gamepad) = this.and_then(|o| o.as_gamepad_object()) {
        let handle = gamepad.handle().0;
        return Ok(handle.into());
    }
    Ok(Value::Null)
}

/// Implements `name`'s getter
pub fn get_name<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(gamepad) = this.and_then(|o| o.as_gamepad_object()) {
        let handle = gamepad.handle();
        if let Some(name) = activation.context.ui.gamepad_name(handle) {
            return Ok(AvmString::new_utf8(activation.context.gc_context, name).into());
        }
    }
    Ok(Value::Null)
}

/// Implements `numControls`'s getter
pub fn get_num_controls<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_getter!(activation, "flash.ui.GameInputDevice", "numControls");
    Ok(Value::Integer(0))
}

/// Implements `getCachedSamples`
pub fn get_cached_samples<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_method!(activation, "flash.ui.GameInputDevice", "getCachedSamples");
    Ok(Value::Integer(0))
}

/*
/// Implements `getControlAt`
pub fn get_control_at<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_method!(activation, "flash.ui.GameInputDevice", "getControlAt");
    Ok(Value::Undefined)
}
*/

/// Implements `startCachingSamples`
pub fn start_caching_samples<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_method!(
        activation,
        "flash.ui.GameInputDevice",
        "startCachingSamples"
    );
    Ok(Value::Undefined)
}

/// Implements `stopCachingSamples`
pub fn stop_caching_samples<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_method!(activation, "flash.ui.GameInputDevice", "stopCachingSamples");
    Ok(Value::Undefined)
}
