use crate::avm2::{Activation, Error, Object, Value};

/// Implements `init`, which is called from the constructor
pub fn init<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this {
        activation.context.gamepad_listeners.push(this);
    }
    Ok(Value::Undefined)
}

/// Implements `isSupported`'s getter
pub fn get_is_supported<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    Ok(activation.context.ui.supports_gamepads().into())
}
