

// NOTE: The below crates will need pub in beta and nightly
extern crate cucumber_state;
extern crate cucumber_server;
extern crate cucumber_runner;
extern crate cucumber_definitions;
extern crate cucumber_event;
extern crate cucumber_regex;
extern crate itertools;

pub use cucumber_state as state;
pub use cucumber_server as server;
pub use cucumber_runner as runner;
pub use cucumber_definitions as definitions;
pub use cucumber_event as event;
pub use cucumber_regex as regex;

mod launcher;

pub use launcher::{
  start,
  start_with_addr,
  ruby_command
};

#[macro_export]
macro_rules! try_destructure {
  ($r: ident) => ({
    use $crate::event::response::InvokeResponse;
    use $crate::definitions::destructuring::{DestructurableSet, InvokeArgSetError};

    match $r.destructure_set() {
      Ok(e) => e,
      Err(error) => {
        match error {
          InvokeArgSetError::TypeMismatch {arg_idx} => {
            return InvokeResponse::with_fail_message(format!("Argument in position [{}] did not have the correct type or was unparseable", arg_idx))
          },
          InvokeArgSetError::ArgCountMismatch {expected, actual} => {
            return InvokeResponse::with_fail_message(format!("Expected [{}] arguments, but found [{}] in step definition", expected, actual))
          }
        }
      }
    }
  })
}

// NOTE: These are capitalized to follow Cucumber general conventions, rather than Rust
#[macro_export]
macro_rules! Given {
  ($cuke:expr; $regex:expr, $body:expr) => {{
    use $crate::regex;
    $cuke.given(file!(), line!(), regex::build($regex), Box::new(move |cuke, world, args| {
      ($body)(cuke, world, try_destructure!(args))
    }))
  }}
}

#[macro_export]
macro_rules! When {
  ($cuke:expr; $regex:expr, $body:expr) => {{
    use $crate::regex;
    $cuke.when(file!(), line!(), regex::build($regex), Box::new(move |cuke, world, args| {
      ($body)(cuke, world, try_destructure!(args))
    }))
  }}
}

#[macro_export]
macro_rules! Then {
  ($cuke:expr; $regex:expr, $body:expr) => {{
    use $crate::regex;
    $cuke.then(file!(), line!(), regex::build($regex), Box::new(move |cuke, world, args| {
      ($body)(cuke, world, try_destructure!(args))
    }))
  }}
}

