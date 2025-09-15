//! PAM error representation.

use crate::ffi;

/// An error that has been encountered in a PAM operation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PamError {
    /// Some memory could not be allocated.
    BufErr = ffi::PAM_BUF_ERR as _,

    /// An error due to a system call.
    SysErr = ffi::PAM_SYSTEM_ERR as _,

    /// The authentication service cannot retrieve the user information.
    UserUnknown = ffi::PAM_USER_UNKNOWN as _,

    /// The authentication token is incorrect.
    AuthErr = ffi::PAM_AUTH_ERR as _,

    /// The authentication token has expired.
    CredExpired = ffi::PAM_CRED_EXPIRED as _,

    /// The account is expired.
    AcctExpired = ffi::PAM_ACCT_EXPIRED as _,

    /// The authentication service is unavailable.
    AuthInfoUnavailable = ffi::PAM_AUTHINFO_UNAVAIL as _,

    /// The authentication token is not set.
    NoModuleData = ffi::PAM_NO_MODULE_DATA as _,

    /// The conversation function is not set.
    ConvErr = ffi::PAM_CONV_ERR as _,

    /// The authentication service is not set.
    ServiceErr = ffi::PAM_SERVICE_ERR as _,

    /// The authentication token is not set.
    AuthTokErr = ffi::PAM_AUTHTOK_ERR as _,

    /// The authentication token is not set.
    AuthTokRecoveryErr = ffi::PAM_AUTHTOK_RECOVERY_ERR as _,

    /// The authentication token is not set.
    AuthTokLockBusy = ffi::PAM_AUTHTOK_LOCK_BUSY as _,

    /// The authentication token is not set.
    AuthTokDisableAging = ffi::PAM_AUTHTOK_DISABLE_AGING as _,

    /// The authentication token is not set.
    TryAgain = ffi::PAM_TRY_AGAIN as _,

    /// The authentication token is not set.
    Ignore = ffi::PAM_IGNORE as _,

    /// The authentication token is not set.
    Abort = ffi::PAM_ABORT as _,
}

impl PamError {
    /// Convert a raw error code into a [`PamError`].
    #[inline]
    #[must_use]
    pub const fn code(target_code: i32) -> Option<Self> {
        match target_code as u32 {
            ffi::PAM_BUF_ERR => Some(PamError::BufErr),
            ffi::PAM_SYSTEM_ERR => Some(PamError::SysErr),
            ffi::PAM_USER_UNKNOWN => Some(PamError::UserUnknown),
            ffi::PAM_AUTH_ERR => Some(PamError::AuthErr),
            ffi::PAM_CRED_EXPIRED => Some(PamError::CredExpired),
            ffi::PAM_ACCT_EXPIRED => Some(PamError::AcctExpired),
            ffi::PAM_AUTHINFO_UNAVAIL => Some(PamError::AuthInfoUnavailable),
            ffi::PAM_NO_MODULE_DATA => Some(PamError::NoModuleData),
            ffi::PAM_CONV_ERR => Some(PamError::ConvErr),
            ffi::PAM_SERVICE_ERR => Some(PamError::ServiceErr),
            ffi::PAM_AUTHTOK_ERR => Some(PamError::AuthTokErr),
            ffi::PAM_AUTHTOK_RECOVERY_ERR => Some(PamError::AuthTokRecoveryErr),
            ffi::PAM_AUTHTOK_LOCK_BUSY => Some(PamError::AuthTokLockBusy),
            ffi::PAM_AUTHTOK_DISABLE_AGING => Some(PamError::AuthTokDisableAging),
            ffi::PAM_TRY_AGAIN => Some(PamError::TryAgain),
            ffi::PAM_IGNORE => Some(PamError::Ignore),
            ffi::PAM_ABORT => Some(PamError::Abort),
            _ => None,
        }
    }
}
