#![feature(core_intrinsics)]

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

mod application;
mod log;
mod event;
mod key_codes;
mod mouse_codes;
mod window;
mod layer;

pub use application::Application;
pub use log::*;
pub use layer::*;
pub use event::*;
pub use key_codes::KeyCode;
pub use mouse_codes::MouseCode;

#[macro_export]
macro_rules! ad_core_assert {
    ($($exp:expr)*, $($arg:tt)*) => {
        if !$($exp)* {
            ad_core_error!("Assertion failed: {}", format_args!($($arg)*));
            flush_log();
            unsafe { std::intrinsics::breakpoint() };
        }
    }
}