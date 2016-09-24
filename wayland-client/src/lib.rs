//! This doc was generate for the upcoming v0.7 of the wayland-client crate.
//! This new version is a complete change of the API. If you are looking for the 
//! documentation of the crate currently on crates.io, check
//! [on docs.rs](https://docs.rs/wayland-client/0.6.2/wayland_client/).

#[macro_use] extern crate bitflags;
#[macro_use] extern crate wayland_sys;
extern crate libc;

pub use generated::client as protocol;

use wayland_sys::client::wl_proxy;
use wayland_sys::common::{wl_interface, wl_argument};

mod display;
mod event_queue;
pub mod env;

pub use event_queue::{EventQueue, EventQueueHandle, StateGuard};
pub use display::{default_connect, ConnectError, FatalError};

/// Common routines for wayland proxy objects.
///
/// All wayland objects automatically implement this trait
/// as generated by the scanner.
///
/// It is mostly used for internal use by the library, and you
/// should only need these methods for interfacing with C library
/// working on wayland objects.
pub trait Proxy {
    /// Pointer to the underlying wayland proxy object
    fn ptr(&self) -> *mut wl_proxy;
    /// Create an instance from a wayland pointer
    ///
    /// The pointer must refer to a valid wayland proxy
    /// of the appropriate interface, but that have not
    /// yet been seen by the library.
    ///
    /// The library will take control of the object (notably
    /// overwrite its user_data).
    unsafe fn from_ptr_new(*mut wl_proxy) -> Self;
    /// Create an instance from a wayland pointer
    ///
    /// The pointer must refer to a valid wayland proxy
    /// of the appropriate interface, and have already been
    /// initialized by the library (it'll assume this proxy
    /// user_data contains a certain kind of data).
    unsafe fn from_ptr_initialized(*mut wl_proxy) -> Self;
    /// Pointer to the interface representation
    fn interface_ptr() -> *const wl_interface;
    /// Internal wayland name of this interface
    fn interface_name() -> &'static str;
    /// Max version of this interface supported
    fn supported_version() -> u32;
    /// Current version of the interface this proxy is instanciated with
    fn version(&self) -> u32;
    /// Check if the proxt behind this handle is actually still alive
    fn is_alive(&self) -> bool;
}

/// Possible outcome of the call of a request on a proxy
pub enum RequestResult<T> {
    /// Message has been buffered and will be sent to server
    Sent(T),
    /// This proxy is already destroyed, request has been ignored
    Destroyed
}

impl<T> RequestResult<T> {
    pub fn expect(self, error: &str) -> T {
        match self {
            RequestResult::Sent(v) => v,
            RequestResult::Destroyed => panic!("{}", error)
        }
    }
}

/// Generic handler trait
///
/// This trait is automatically implemented for objects that implement
/// the appropriate interface-specific `Handler` traits. It represents
/// the hability for a type to handle events directed to a given wayland
/// interface.
///
/// For example, implementing `wl_surface::Handler` for you type will
/// automatically provide it with an implementation of
/// `Handler<WlSurface>` as well. This is the only correct way
/// to implement this trait, and you should not attempt to implement it
/// yourself.
pub unsafe trait Handler<T: Proxy> {
    unsafe fn message(&mut self, evq: &mut EventQueueHandle, proxy: &T, opcode: u32, args: *const wl_argument) -> Result<(),()>;
}

mod generated {
    #![allow(dead_code,non_camel_case_types,unused_unsafe,unused_variables)]
    #![allow(non_upper_case_globals,non_snake_case,unused_imports)]

    pub mod interfaces {
        include!(concat!(env!("OUT_DIR"), "/wayland_interfaces.rs"));
    }

    pub mod client {
        // Imports that need to be available to submodules
        // but should not be in public API.
        // Will be fixable with pub(restricted).
        #[doc(hidden)] pub use {Proxy, Handler, RequestResult};
        #[doc(hidden)] pub use event_queue::EventQueueHandle;
        #[doc(hidden)] pub use super::interfaces;
        include!(concat!(env!("OUT_DIR"), "/wayland_api.rs"));
    }
}

pub mod sys {
    pub use wayland_sys::common::*;
    pub use wayland_sys::client::*;
}
