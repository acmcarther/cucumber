#[cfg(feature = "serde_macros")]
include!("response.rs.in");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/cucumber/response.rs"));
