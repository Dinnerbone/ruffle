//! `flash.crypto` namespace

use crate::avm2::object::TObject;
use crate::avm2::{Activation, Error, Object, Value};
use rand::{rngs::OsRng, RngCore};
use ruffle_types::backend::Backend;

/// Implements `flash.crypto.generateRandomBytes`
pub fn generate_random_bytes<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    let length = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .coerce_to_u32(activation)?;
    if !(1..1025).contains(&length) {
        return Err("Error: Error #2004: One of the parameters is invalid".into());
    }

    let ba_class = activation.context.avm2.classes().bytearray;
    let ba = ba_class.construct(activation, &[])?;
    let mut ba_write = ba.as_bytearray_mut(activation.context.gc_context).unwrap();
    ba_write.set_length(length as usize);

    let mut rng = OsRng {};

    rng.fill_bytes(ba_write.bytes_mut());

    Ok(ba.into())
}
