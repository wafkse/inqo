//! Handles to PAM sessions.
//!
//! This module provides a safe wrapper around the [`inqo_sys::pam_handle_t`]
//! type. It is not safe to use directly, but can be build upon.

use alloc::ffi::CString;

use inqo_sys::{PAM_SILENT, pam_authenticate, pam_end, pam_start};

use crate::{
    converse::Dialoge,
    error::PamError,
    raw::{RawConv, RawHandle},
    service::Service,
};

/// An error that can occur when creating a new [`Client`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ClientError {
    /// A PAM error occurred.
    Pam(PamError),

    /// The target username is invalid.
    InvalidUsername,
}

/// A client in a PAM session.
#[derive(Debug)]
pub struct Client<'a, C>
where
    C: Dialoge,
{
    /// The handle that is currently in use.
    client_handle: *mut RawHandle,

    /// The backing dialoge handler.
    client_backend: &'a mut C,

    /// The recentmost error that occurred in the client.
    client_status: Option<PamError>,
}

impl<'a, C> Client<'a, C>
where
    C: Dialoge,
{
    /// Determine the [`RawHandle`] of the client.
    #[inline]
    #[must_use]
    pub const fn raw_handle(&self) -> *mut RawHandle {
        let &Self { client_handle, .. } = self;

        client_handle
    }

    /// Determine the backend dialoge handler.
    #[inline]
    pub const fn backend(&mut self) -> &mut &'a mut C {
        let &mut Self {
            ref mut client_backend,
            ..
        } = self;

        client_backend
    }

    /// Determine the recentmost error that occurred in the client.
    #[inline]
    #[must_use]
    pub const fn recentmost_error(&self) -> Option<PamError> {
        let &Self {
            client_status: prev_status,
            ..
        } = self;

        prev_status
    }
}

impl<'a, C> Client<'a, C>
where
    C: Dialoge,
{
    /// Create a new client session from the target dialoge handler.
    #[inline]
    pub fn session<S: Service>(client_backend: &'a mut C, user: &str) -> Result<Self, ClientError> {
        {
            let mut pam_handle = core::ptr::null_mut();

            let raw_conv = RawConv::raw(client_backend);

            let user = CString::new(user).map_err(|_| ClientError::InvalidUsername)?;

            // SAFETY: conversation and handle pointers are guaranteed to be valid.
            let pam_status = unsafe {
                pam_start(
                    S::SERVICE_NAME.as_ptr(),
                    user.as_ptr(),
                    raw_conv.inner(),
                    &mut pam_handle,
                )
            };

            if pam_status != inqo_sys::PAM_SUCCESS as i32 {
                match pam_status as u32 {
                    inqo_sys::PAM_ABORT => return Err(ClientError::Pam(PamError::Abort)),
                    inqo_sys::PAM_BUF_ERR => return Err(ClientError::Pam(PamError::BufErr)),
                    inqo_sys::PAM_SYSTEM_ERR => return Err(ClientError::Pam(PamError::SysErr)),
                    _ => unreachable!(),
                }
            }

            // SAFETY: Bare-bones `pam_handle` is not used again.
            let client_handle = unsafe { RawHandle::point(pam_handle) };

            Ok(Self {
                client_handle,
                client_backend,
                client_status: None,
            })
        }
    }

    /// Attempt to authenticate the client.
    #[inline]
    pub fn authenticate(&mut self) -> Result<(), PamError> {
        let &mut Self {
            client_handle,
            ref mut client_status,
            ..
        } = self;

        // SAFETY: client handle is guaranteed to be valid
        let pam_status = unsafe { pam_authenticate(client_handle.cast(), PAM_SILENT as i32) };

        if pam_status == inqo_sys::PAM_SUCCESS as i32 {
            Ok(())
        } else {
            let target_error = PamError::code(pam_status);

            *client_status = target_error;

            Err(target_error.unwrap_or(PamError::SysErr))
        }
    }
}

impl<'a, C> Drop for Client<'a, C>
where
    C: Dialoge,
{
    fn drop(&mut self) {
        let &mut Self {
            client_handle,
            client_status,
            ..
        } = self;

        // SAFETY: the handle is always non-null
        unsafe {
            // NOTE: we do not check this for errors
            // since the handle is guaranteed to be valid
            // and we are not a PAM module.. so, theoretically
            // this is an infallible operation
            let _ = pam_end(
                client_handle.cast(),
                client_status
                    .map(|target_variant| target_variant as i32)
                    .unwrap_or(inqo_sys::PAM_SUCCESS as i32),
            );
        }
    }
}
