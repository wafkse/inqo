//! Raw, Rust-native representation of raw PAM structures.

use core::ptr::NonNull;

use inqo_sys::PAM_CONV_ERR;

use crate::converse::{Dialogate, Dialoge, DialogeStatus};

/// A raw message in a PAM session.
///
/// This is a simple new-type wrapper around a [`inqo_sys::pam_message`].
/// Therefore, this type is not safe to use directly, but can be build upon.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct RawMessage(inqo_sys::pam_message);

impl RawMessage {
    /// Create a new [`RawMessage`] from a [`inqo_sys::pam_message`].
    ///
    /// # Safety
    ///
    /// The [`inqo_sys::pam_message`] must be valid and not be used after this
    /// function is called.
    #[inline]
    #[must_use]
    pub const unsafe fn ffi(message: inqo_sys::pam_message) -> Self {
        Self(message)
    }

    /// Create a new [`RawMessage`] from a mutable pointer to a
    /// [`inqo_sys::pam_message`].
    ///
    /// # Safety
    ///
    /// The [`inqo_sys::pam_message`] must be valid and not be used after this
    /// function is called.
    #[inline]
    #[must_use]
    pub const unsafe fn point(message: *mut inqo_sys::pam_message) -> *mut Self {
        message as *mut Self
    }
}

impl RawMessage {
    /// Fetch the inner [`inqo_sys::pam_message`].
    #[inline]
    #[must_use]
    pub const fn inner(&self) -> &inqo_sys::pam_message {
        let &Self(ref ptr) = self;

        ptr
    }
}

/// A raw response to a PAM message.
///
/// This is a simple new-type wrapper around a [`inqo_sys::pam_response`].
/// Therefore, this type is not safe to use directly, but can be build upon.
#[repr(transparent)]
pub struct RawResponse(inqo_sys::pam_response);

impl RawResponse {
    /// Create a new [`RawResponse`] from a [`inqo_sys::pam_response`].
    ///
    /// # Safety
    ///
    /// The [`inqo_sys::pam_response`] must be valid and not be used after this
    /// function is called.
    #[inline]
    #[must_use]
    pub const fn ffi(response: inqo_sys::pam_response) -> Self {
        Self(response)
    }

    /// Create a new [`RawResponse`] from a mutable pointer to a
    /// [`inqo_sys::pam_response`].
    ///
    /// # Safety
    ///
    /// The [`inqo_sys::pam_response`] must be valid and not be used after this
    /// function is called.
    #[inline]
    #[must_use]
    pub const unsafe fn point(response: *mut inqo_sys::pam_response) -> *mut Self {
        response as *mut Self
    }

    /// Create a new [`RawResponse`] that indicates an unspecified failure
    /// condition.
    #[inline]
    #[must_use]
    pub const fn failure() -> Self {
        let (resp, resp_retcode) = (core::ptr::null_mut(), PAM_CONV_ERR as i32);

        Self(inqo_sys::pam_response { resp, resp_retcode })
    }
}

impl RawResponse {
    /// Fetch the inner [`inqo_sys::pam_response`].
    #[inline]
    #[must_use]
    pub const fn inner(&self) -> &inqo_sys::pam_response {
        let &Self(ref ptr) = self;

        ptr
    }
}

/// A raw PAM conversation handler.
///
/// This type is not safe to use directly, but can be built upon.
#[derive(Debug, Clone, Copy)]
pub struct RawConv(inqo_sys::pam_conv);

impl RawConv {
    /// Create a new [`RawConv`] from a target callback function.
    #[inline]
    #[must_use]
    pub const fn raw<'a, C: Dialoge + 'a>(target_value: &'a C) -> Self {
        let target_item = Dialogate::stub::<'a, C>
            as unsafe extern "C" fn(
                i32,
                *mut *mut RawMessage,
                *mut *mut RawResponse,
                NonNull<libc::c_void>,
            ) -> DialogeStatus;

        // SAFETY: the arguments to each function have the same ABI
        let conv = Some(unsafe { core::mem::transmute(target_item) });

        // SAFETY: references are always non-null
        let appdata_ptr = unsafe {
            NonNull::new_unchecked((target_value as *const C).cast_mut())
                .as_ptr()
                .cast()
        };

        Self(inqo_sys::pam_conv { conv, appdata_ptr })
    }

    /// Create a new [`RawConv`] from a [`inqo_sys::pam_conv`].
    ///
    /// # Safety
    ///
    /// The [`inqo_sys::pam_conv`] must be valid and not be used after this
    /// function is called.
    #[inline]
    #[must_use]
    pub const fn ffi(response: inqo_sys::pam_conv) -> Self {
        Self(response)
    }

    /// Create a new [`RawConv`] from a mutable pointer to a
    /// [`inqo_sys::pam_conv`].
    ///
    /// # Safety
    ///
    /// The [`inqo_sys::pam_conv`] must be valid and not be used after this
    /// function is called.
    #[inline]
    #[must_use]
    pub const unsafe fn point(conv: *mut inqo_sys::pam_conv) -> *mut Self {
        conv as *mut Self
    }
}

impl RawConv {
    /// Fetch the inner [`inqo_sys::pam_conv`].
    #[inline]
    #[must_use]
    pub const fn inner(&self) -> &inqo_sys::pam_conv {
        let &Self(ref ptr) = self;

        ptr
    }
}

/// A raw handle to a PAM session.
///
/// This is a simple new-type wrapper around a [`inqo_sys::pam_handle_t`].
/// Therefore, this type is not safe to use directly, but can be build upon.
#[derive(Debug)]
#[repr(transparent)]
pub struct RawHandle(inqo_sys::pam_handle_t);

impl RawHandle {
    /// Create a new [`RawHandle`] from a [`inqo_sys::pam_handle`].
    ///
    /// # Safety
    ///
    /// The [`inqo_sys::pam_handle`] must be valid and not be used after this
    /// function is called.
    #[inline]
    #[must_use]
    pub const unsafe fn ffi(handle: inqo_sys::pam_handle_t) -> Self {
        Self(handle)
    }

    /// Create a new [`RawHandle`] from a mutable pointer to a
    /// [`inqo_sys::pam_handle`].
    ///
    /// # Safety
    ///
    /// The [`inqo_sys::pam_handle`] must be valid and not be used after this
    /// function is called.
    #[inline]
    #[must_use]
    pub const unsafe fn point(handle: *mut inqo_sys::pam_handle_t) -> *mut Self {
        handle as *mut Self
    }
}

impl RawHandle {
    /// Fetch the inner [`inqo_sys::pam_handle_t`].
    ///
    /// # Safety
    ///
    /// This function is safe to call, but the returned
    /// [`inqo_sys::pam_handle_t`] is not safe to use directly, and thus must be
    /// handled with care.
    #[inline]
    #[must_use]
    pub const unsafe fn inner(&self) -> inqo_sys::pam_handle_t {
        let &Self(ptr) = self;

        ptr
    }
}
