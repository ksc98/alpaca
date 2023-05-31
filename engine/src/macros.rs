use log::{debug, error, info, warn};
use std::env;

macro_rules! default_env {
    ($name:expr, $default:expr) => {
        env::var($name).unwrap_or_else(|_| String::from($default))
    };
}
