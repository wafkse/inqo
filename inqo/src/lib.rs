#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../../README.md")]
#![forbid(clippy::all, rustdoc::all)]
#![forbid(missing_docs)]

extern crate alloc;

pub use inqo_sys as ffi;

pub mod converse;

pub mod raw;

pub mod client;

pub mod error;

pub mod service;

pub mod prelude {
    //! Module for re-exporting common types and traits.

    pub use crate::{
        client::Client,
        converse::{Dialogate, Dialoge, DialogeStatus},
        error::PamError,
        raw::{RawMessage, RawResponse},
        service::Service,
    };
}
