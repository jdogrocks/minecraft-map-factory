use std::error::Error;

/// Version checking is disabled — this fork does not track upstream releases.
pub fn check_for_updates() -> Result<bool, Box<dyn Error>> {
    Ok(false)
}
