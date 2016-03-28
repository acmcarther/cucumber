#![cfg_attr(feature = "serde_macros", feature(custom_derive, plugin))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

extern crate serde;

#[cfg(test)]
extern crate serde_json;

pub mod request;
pub mod response;
