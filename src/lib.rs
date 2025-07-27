//! hyperlane-time
//!
//! A library for fetching the current time based on the system's locale settings.

pub(crate) mod time;

pub use time::r#fn::*;

pub(crate) use time::r#enum::from_env_var;

pub(crate) use std::{
    env, fmt,
    fmt::Write,
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
