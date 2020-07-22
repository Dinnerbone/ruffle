//! FSCommand handling

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;

/// Parse an FSCommand URL.
pub fn parse(url: &str) -> Option<&str> {
    log::info!("Checking {}", url);
    if url.to_lowercase().starts_with("fscommand:") {
        Some(&url["fscommand:".len()..])
    } else {
        None
    }
}

/// TODO: FSCommand URL handling
pub fn handle<'gc>(
    fscommand: &str,
    _activation: &mut Activation<'_, '_, 'gc, '_>,
) -> Result<(), Error<'gc>> {
    log::warn!("Unhandled FSCommand: {}", fscommand);

    //This should be an error.
    Ok(())
}
