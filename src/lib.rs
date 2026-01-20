//! hyperlane-time
//!
//! A library for fetching the current time based on the system's locale settings.

pub(crate) mod r#enum;
pub(crate) mod r#fn;
pub(crate) mod r#impl;

#[cfg(test)]
mod test;

pub use r#fn::*;

pub(crate) use r#enum::*;

pub(crate) use std::{
    env, fmt,
    fmt::Write,
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
