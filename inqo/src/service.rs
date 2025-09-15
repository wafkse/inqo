//! Module for PAM service clients.

use core::ffi::CStr;

/// A trait that represents an identifiable PAM service.
///
/// This has no other data or methods beyond the [`Service::SERVICE_NAME`] associated constant.
pub trait Service {
    /// The name of the service as it is known to the PAM.
    const SERVICE_NAME: &'static CStr;
}

/// The `login` PAM service.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Login {}

impl Service for Login {
    const SERVICE_NAME: &'static CStr = c"login";
}
