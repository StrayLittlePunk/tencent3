mod tmt;
mod utils;

pub use tmt::*;

const JSON_MIME: &str = "application/json";

/// MethodCall return Type need implement this trait
pub trait CallOutput {}
