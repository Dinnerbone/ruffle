//! FSCommand handling

use crate::avm1::{Avm1, Error, UpdateContext};
use crate::backend::Backends;

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
pub fn handle<B: Backends>(fscommand: &str, _avm: &mut Avm1<B>, _ac: &mut UpdateContext<B>) -> Result<(), Error> {
    log::warn!("Unhandled FSCommand: {}", fscommand);

    //This should be an error.
    Ok(())
}
