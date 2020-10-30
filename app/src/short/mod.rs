pub mod error;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod service;
pub mod validate;

pub const URL_KEY: &str = "urls";
pub const URL_CHARACTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
pub const URL_IDEAL_LENGTH: usize = 5;
pub const URL_IDEAL_RETRIES: u32 = 3;
