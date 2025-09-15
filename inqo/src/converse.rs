//! Module-to-client communication for the Inqo library.

use core::{ffi::CStr, mem::MaybeUninit, ptr::NonNull};

use inqo_sys::{
    PAM_BUF_ERR, PAM_CONV_ERR, PAM_ERROR_MSG, PAM_PROMPT_ECHO_OFF, PAM_PROMPT_ECHO_ON, PAM_SUCCESS,
    PAM_TEXT_INFO,
};

use crate::raw::{RawMessage, RawResponse};

/// The message styles that are possible in a normal conversation.
///
/// Note that this type deliberately omits any other styles. To use a custom
/// style, implement the [`Dialoge`] trait and use the [`Dialogate::stub`]
/// function.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum Normal {
    /// Obtain a string without echoing any text.
    EchoOff = PAM_PROMPT_ECHO_OFF,

    /// Obtain a string whilst echoing text.
    EchoOn = PAM_PROMPT_ECHO_ON,

    /// Display an error message.
    ErrorMsg = PAM_ERROR_MSG,

    /// Display some text.
    TextInfo = PAM_TEXT_INFO,
}

impl Style for Normal {
    #[inline]
    fn try_identify(message_style: BareStyle) -> Option<Self>
    where
        Self: Sized,
    {
        const NORMAL_ECHO_OFF: i32 = PAM_PROMPT_ECHO_OFF as i32;
        const NORMAL_ECHO_ON: i32 = PAM_PROMPT_ECHO_ON as i32;
        const NORMAL_ERROR_MSG: i32 = PAM_ERROR_MSG as i32;
        const NORMAL_TEXT_INFO: i32 = PAM_TEXT_INFO as i32;

        match message_style.value() {
            NORMAL_ECHO_OFF => Some(Normal::EchoOff),
            NORMAL_ECHO_ON => Some(Normal::EchoOn),
            NORMAL_ERROR_MSG => Some(Normal::ErrorMsg),
            NORMAL_TEXT_INFO => Some(Normal::TextInfo),
            _ => None,
        }
    }
}

/// A raw normal message.
///
/// This is the default type of message style, but does not process its content.
#[derive(Debug, Clone, Copy)]
pub struct RawNormal {
    /// The style of the message.
    style: Normal,

    /// The raw message content.
    raw_message: RawMessage,
}

impl RawNormal {
    /// Determine the message style for this [`RawNormal`].
    #[inline]
    pub const fn style(&self) -> Normal {
        let &Self { style, .. } = self;

        style
    }

    /// Fetch the raw message body from this [`RawNormal`].
    #[inline]
    pub const fn message(&self) -> RawMessage {
        let &Self { raw_message, .. } = self;

        raw_message
    }
}

impl MessageContent<'_, Normal> for RawNormal {
    #[inline]
    fn parse(style: Normal, &raw_message: &RawMessage) -> Self {
        RawNormal { style, raw_message }
    }
}

/// Determine whether input should be echoed back or not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Echo {
    /// The message should be echoed back.
    On,

    /// The message should not be echoed back.
    Off,
}

impl Echo {
    /// Determine the [`Echo`] variant based on the target state.
    #[inline]
    pub const fn bool(target_state: bool) -> Self {
        if target_state { Echo::On } else { Echo::Off }
    }
}

/// A normal message.
///
/// This is a processed message, and thus can be consumed directly.
///
/// # Remarks
///
/// Note that the lifetime parameter `'a` is extra, since the message lifetime
/// is actually `'static`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NormalMessage<'a> {
    /// A prompt for user input.
    Prompt {
        /// The text of the prompt.
        prompt_text: &'a CStr,

        /// Whether the prompt should be echoed back.
        prompt_echo: Echo,
    },

    /// An information message.
    Info {
        /// The information message.
        info_message: &'a CStr,
    },

    /// An error message.
    Error {
        /// The error message.
        error_message: &'a CStr,
    },
}

impl<'a> MessageContent<'a, Normal> for NormalMessage<'a> {
    #[inline]
    fn parse(target_style: Normal, raw_message: &'a RawMessage) -> Self {
        match target_style {
            Normal::EchoOff | Normal::EchoOn => {
                let &inqo_sys::pam_message {
                    msg_style: _,
                    msg: prompt_text,
                } = raw_message.inner();

                // SAFETY: The pointer should be valid.
                let prompt_text = unsafe { CStr::from_ptr(prompt_text) };

                let prompt_echo = match target_style {
                    Normal::EchoOff => Echo::Off,
                    Normal::EchoOn => Echo::On,
                    _ => unreachable!(),
                };

                NormalMessage::Prompt {
                    prompt_text,
                    prompt_echo,
                }
            }
            Normal::TextInfo => {
                let &inqo_sys::pam_message {
                    msg_style: _,
                    msg: info_message,
                } = raw_message.inner();

                // SAFETY: The pointer should be valid.
                let info_message = unsafe { CStr::from_ptr(info_message) };

                NormalMessage::Info { info_message }
            }
            Normal::ErrorMsg => {
                let &inqo_sys::pam_message {
                    msg_style: _,
                    msg: error_message,
                } = raw_message.inner();

                // SAFETY: The pointer should be valid.
                let error_message = unsafe { CStr::from_ptr(error_message) };

                NormalMessage::Error { error_message }
            }
        }
    }
}

impl<'a> NormalMessage<'a> {
    /// Determine the prompt to use for this message, regardless of the variant.
    #[inline]
    pub const fn prompt(&self) -> &'a CStr {
        let &(Self::Prompt { prompt_text, .. }
        | Self::Info {
            info_message: prompt_text,
            ..
        }
        | Self::Error {
            error_message: prompt_text,
            ..
        }) = self;

        prompt_text
    }
}

/// A trait for identifying message styles.
///
/// This is analogous to [`TryFrom`], but posesses more concise semantics.
pub trait Style {
    /// Try to identify some message style.
    /// Returns `None` if the style is not recognized.
    fn try_identify(message_style: BareStyle) -> Option<Self>
    where
        Self: Sized;
}

/// A trait that represents the dialog between the module and the client.
///
/// This is used to tell the client what they can expect from the module.
///
/// # FFI Usage
///
/// If you wish to use implementors of this trait in `FFI`, you can use the
/// [`Dialogate::stub`] function, particularly, its monomorphized version
/// for the target type, e.g: `Dialogate::stub::<MyDialoge>`
pub trait Dialoge {
    /// The message style to be used.
    ///
    /// This is an associated type to allow for use of PAM's callback mechanism
    /// for motives that are out of scope for this crate.
    ///
    /// However, in most cases, this can be [`Normal`].
    type MessageStyle: Style;

    /// The message type to be used for this [`Dialoge`].
    ///
    ///
    /// In most cases, this can be a [`NormalMessage`].
    type Message<'message>: MessageContent<'message, Self::MessageStyle>;

    /// Perform one conversation step.
    ///
    /// A call to this function is not equivalent to a full end-to-end
    /// conversation, but rather a single message-to-response interaction.
    fn converse<'context, 'message>(
        &'context self,
        message_style: Self::Message<'message>,
    ) -> &'context CStr;
}

/// A trait representing the contents of a message for some [`Style`] `S`.
pub trait MessageContent<'message, S>
where
    S: Style,
{
    /// Parse a raw message into a message content.
    ///
    /// This must be infallible.
    fn parse(target_style: S, raw_message: &'message RawMessage) -> Self;
}

/// The status enumeration for a dialog between the module and the client.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum DialogeStatus {
    /// The operation was successful.
    Success = PAM_SUCCESS,

    /// The operation failed due to a buffer error.
    BufErr = PAM_BUF_ERR,

    /// The operation failed due to a conversation error.
    ConvErr = PAM_CONV_ERR,
}

/// A bare message style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BareStyle(i32);

impl BareStyle {
    /// Create a new bare message style from the target [`u32`].
    #[inline]
    #[must_use]
    pub const fn raw(target_style: i32) -> Self {
        Self(target_style)
    }

    /// Fetch the inner [`i32`] value.
    #[inline]
    #[must_use]
    pub const fn value(&self) -> i32 {
        let &Self(value) = self;

        value
    }
}

/// A marker enum that represents the dialog between the module and the client.
pub enum Dialogate {}

impl Dialogate {
    /// The dialog stub for some particular [`Dialoge`] `D`.
    ///
    /// # Safety
    ///
    /// The following arguments must be valid:
    ///
    /// - `message_count` must be non-negative and non-zero.
    /// - `message_list` must be a valid pointer to an array of `message_count`
    ///   elements.
    /// - `response_list` must be non-null and a valid location.
    /// - `target_data` must be a non-null, valid pointer to the [`Dialoge`]
    ///   `D`. Additionally, it must be convertible to a mutable reference to `D`.
    #[inline]
    pub unsafe extern "C" fn stub<'a, D: Dialoge + 'a>(
        message_count: libc::c_int,
        message_list: *mut *mut RawMessage,
        response_data: *mut *mut RawResponse,
        target_data: NonNull<libc::c_void>,
    ) -> DialogeStatus {
        let message_count = message_count as usize;

        // SAFETY: The caller must ensure that `message_list` is a valid pointer to an
        // array of `message_count` elements.
        let message_list = unsafe {
            core::slice::from_raw_parts_mut(message_list.cast::<&mut RawMessage>(), message_count)
        };

        // SAFETY: The returned pointer is made sure to be non-null and aligned.
        let mut response_list = unsafe {
            let target_raw = libc::malloc(message_count * core::mem::size_of::<RawResponse>())
                .cast::<MaybeUninit<RawResponse>>();

            match NonNull::new(target_raw) {
                Some(target_list) => target_list,
                None => return DialogeStatus::BufErr,
            }
        };

        // SAFETY: pointer is conversible due to upstream safety constraints.
        let target_dialogue: &'a mut D = unsafe { target_data.cast::<D>().as_mut() };

        for (target_index, target_message) in message_list.into_iter().enumerate() {
            // SAFETY: pointer is in bounds due to caller safety constraints.
            let target_response = unsafe { response_list.offset(target_index as isize) };

            let &inqo_sys::pam_message {
                msg_style: message_style,
                ..
            } = target_message.inner();

            let message_style = BareStyle::raw(message_style);

            match D::MessageStyle::try_identify(message_style) {
                Some(message_style) => {
                    let target_content = D::Message::parse(message_style, target_message);

                    let target_input = match D::converse(target_dialogue, target_content) {
                        target_value if target_value.is_empty() => core::ptr::null_mut(),
                        target_value => {
                            // SAFETY: the target value is guaranteed to be valid.
                            unsafe { libc::strdup(target_value.as_ptr()) }
                        }
                    };

                    // SAFETY: pointer is valid and aligned.
                    unsafe {
                        let resp = target_input;
                        let resp_retcode = 0;

                        core::ptr::write(
                            target_response.as_ptr().cast::<RawResponse>(),
                            RawResponse::ffi(inqo_sys::pam_response { resp, resp_retcode }),
                        )
                    };
                }
                None => {
                    // SAFETY: pointer is valid and aligned.
                    unsafe {
                        core::ptr::write(
                            target_response.as_ptr().cast::<RawResponse>(),
                            RawResponse::failure(),
                        )
                    };

                    return DialogeStatus::ConvErr;
                }
            }
        }

        // SAFETY: `response_data` is a valid pointer due to safety contract
        unsafe { core::ptr::write(response_data, response_list.as_mut().assume_init_mut()) };

        DialogeStatus::Success
    }
}
