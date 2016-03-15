#[cfg(feature = "serde_macros")]
include!("request.rs.in");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/cucumber/request.rs"));
