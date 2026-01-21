//! hyperlane-time
//!
//! A library for fetching the current time based on the system's locale settings.

mod r#enum;
mod r#fn;
mod r#impl;
#[cfg(test)]
mod test;

pub use r#fn::*;

use r#enum::*;

use std::{
    env, fmt,
    fmt::Write,
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
