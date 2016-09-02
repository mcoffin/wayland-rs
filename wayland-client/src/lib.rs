#[macro_use] extern crate wayland_sys;

pub use sys::client as protocol;

use wayland_sys::client::wl_proxy;
use wayland_sys::common::{wl_interface, wl_argument};

pub mod event_queue;

use event_queue::EventQueueHandle;

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
    /// Create an instance from point a wayland pointer
    ///
    /// The pointer must refer to a valid wayland proxy
    /// of the appropriate interface.
    unsafe fn from_ptr(*mut wl_proxy) -> Self;
    /// Pointer to the interface representation
    fn interface_ptr() -> *const wl_interface;
    /// Internal wayland name of this interface
    fn interface_name() -> &'static str;
    /// Max version of this interface supported
    fn supported_version() -> u32;
    /// Current version of the interface this proxy is instanciated with
    fn version(&self) -> u32;
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

mod sys {
    #![allow(dead_code,non_camel_case_types,unused_unsafe,unused_variables)]
    #![allow(non_upper_case_globals,non_snake_case,unused_imports)]

    pub mod interfaces {
        include!(concat!(env!("OUT_DIR"), "/wayland_interfaces.rs"));
    }

    pub mod client {
        // Imports that need to be available to submodules
        // but should not be in public API.
        // Will be fixable with pub(restricted).
        #[doc(hidden)] pub use {Proxy, Handler};
        #[doc(hidden)] pub use event_queue::EventQueueHandle;
        #[doc(hidden)] pub use super::interfaces;
        include!(concat!(env!("OUT_DIR"), "/wayland_api.rs"));
    }
}
