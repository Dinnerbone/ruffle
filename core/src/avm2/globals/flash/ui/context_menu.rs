use crate::avm2::activation::Activation;
use crate::avm2::object::Object;
use crate::avm2::value::Value;
use crate::avm2::Error;
use ruffle_types::backend::Backend;

pub fn hide_built_in_items<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    // TODO: replace this by a proper implementation.
    log::warn!("flash.ui.ContextMenu is a stub");
    activation
        .context
        .stage
        .set_show_menu(&mut activation.context, false);

    Ok(Value::Undefined)
}
