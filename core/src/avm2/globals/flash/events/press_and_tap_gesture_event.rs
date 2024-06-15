use crate::avm2::activation::Activation;
use crate::avm2::globals::flash::events::mouse_event;
use crate::avm2::object::Object;
use crate::avm2::value::Value;
use crate::avm2::Error;

pub fn get_tap_stage_x<'gc>(
    activation: &mut Activation<'_, '_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    mouse_event::local_to_stage_x(activation, this, "localTapX", "localTapY")
}

pub fn get_tap_stage_y<'gc>(
    activation: &mut Activation<'_, '_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    mouse_event::local_to_stage_y(activation, this, "localTapX", "localTapY")
}
