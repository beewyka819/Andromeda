use lazy_static::lazy_static;
use std::sync::Mutex;
use super::Log;

lazy_static! {
    static ref LOG: Mutex<Log> = Mutex::new(Log::new());
}

/// Forces any unwritten log messages to be written by dropping the logger
pub fn destroy_log() {
    LOG.lock().unwrap().destroy();
}

/// Forces any unwritten log messages to be written by dropping the logger
/// before then reconstructing it.
pub fn flush_log() {
    LOG.lock().unwrap().flush();
}

#[doc(hidden)]
pub fn _ad_core_trace(args: std::fmt::Arguments) {
    let string = format!("{}", args);
    trace!(LOG.lock().unwrap().log().as_ref().unwrap(), "[ANDROMEDA] {}", string);
}

#[doc(hidden)]
pub fn _ad_core_debug(args: std::fmt::Arguments) {
    let string = format!("{}", args);
    debug!(LOG.lock().unwrap().log().as_ref().unwrap(), "[ANDROMEDA] {}", string);
}

#[doc(hidden)]
pub fn _ad_core_info(args: std::fmt::Arguments) {
    let string = format!("{}", args);
    info!(LOG.lock().unwrap().log().as_ref().unwrap(), "[ANDROMEDA] {}", string);
}

#[doc(hidden)]
pub fn _ad_core_warn(args: std::fmt::Arguments) {
    let string = format!("{}", args);
    warn!(LOG.lock().unwrap().log().as_ref().unwrap(), "[ANDROMEDA] {}", string);
}

#[doc(hidden)]
pub fn _ad_core_error(args: std::fmt::Arguments) {
    let string = format!("{}", args);
    error!(LOG.lock().unwrap().log().as_ref().unwrap(), "[ANDROMEDA] {}", string);
}

#[doc(hidden)]
pub fn _ad_core_crit(args: std::fmt::Arguments) {
    let string = format!("{}", args);
    crit!(LOG.lock().unwrap().log().as_ref().unwrap(), "[ANDROMEDA] {}", string);
}

#[doc(hidden)]
pub fn _ad_trace(args: std::fmt::Arguments) {
    let string = format!("{}", args);
    trace!(LOG.lock().unwrap().log().as_ref().unwrap(), "[APP] {}", string);
}

#[doc(hidden)]
pub fn _ad_debug(args: std::fmt::Arguments) {
    let string = format!("{}", args);
    debug!(LOG.lock().unwrap().log().as_ref().unwrap(), "[APP] {}", string);
}

#[doc(hidden)]
pub fn _ad_info(args: std::fmt::Arguments) {
    let string = format!("{}", args);
    info!(LOG.lock().unwrap().log().as_ref().unwrap(), "[APP] {}", string);
}

#[doc(hidden)]
pub fn _ad_warn(args: std::fmt::Arguments) {
    let string = format!("{}", args);
    warn!(LOG.lock().unwrap().log().as_ref().unwrap(), "[APP] {}", string);
}

#[doc(hidden)]
pub fn _ad_error(args: std::fmt::Arguments) {
    let string = format!("{}", args);
    error!(LOG.lock().unwrap().log().as_ref().unwrap(), "[APP] {}", string);
}

#[doc(hidden)]
pub fn _ad_crit(args: std::fmt::Arguments) {
    let string = format!("{}", args);
    crit!(LOG.lock().unwrap().log().as_ref().unwrap(), "[APP] {}", string);
}

#[macro_export]
macro_rules! ad_core_trace {
    ($($arg:tt)*) => ($crate::_ad_core_trace(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! ad_core_debug {
    ($($arg:tt)*) => ($crate::_ad_core_debug(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! ad_core_info {
    ($($arg:tt)*) => ($crate::_ad_core_info(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! ad_core_warn {
    ($($arg:tt)*) => ($crate::_ad_core_warn(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! ad_core_error {
    ($($arg:tt)*) => ($crate::_ad_core_error(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! ad_core_crit {
    ($($arg:tt)*) => ($crate::_ad_core_crit(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! ad_trace {
    ($($arg:tt)*) => ($crate::_ad_trace(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! ad_debug {
    ($($arg:tt)*) => ($crate::_ad_debug(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! ad_info {
    ($($arg:tt)*) => ($crate::_ad_info(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! ad_warn {
    ($($arg:tt)*) => ($crate::_ad_warn(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! ad_error {
    ($($arg:tt)*) => ($crate::_ad_error(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! ad_crit {
    ($($arg:tt)*) => ($crate::_ad_crit(format_args!($($arg)*)));
}