#![doc(html_root_url = "https://acmcarther.github.io/cucumber-rs/")]

#![deny(missing_docs)]

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

/// Destructure a vector of [InvokeArgument](event/request/enum.InvokeArgument.html) into a tuple of values similar to normal try!
///
/// Will either short circult return an InvokeArgument::Fail, describing either improper arg count
/// or improper arg type, or will yield the tuple of values
///
/// Reminder: Tuple of one value is represented as `(t,): (Type,)`
///
/// # Example
///
/// ```
/// fn do_work() -> InvokeResponse {
///   let (x, y, z): (u32, u32, bool) = try_destructure!(vec![
///     InvokeArgument::from_str("1"),
///     InvokeArgument::from_str("2"),
///     InvokeArgument::None
///   ]);
///
///   if x == 1 && y == 2 and z == false {
///     InvokeResponse::Success
///   } else {
///     InvokeResponse::fail_from_str("Values did not match")
///   }
/// }
/// ```
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
            return InvokeResponse::fail_from_str(&format!("Argument in position [{}] did not have the correct type or was unparseable", arg_idx))
          },
          InvokeArgSetError::ArgCountMismatch {expected, actual} => {
            return InvokeResponse::fail_from_str(&format!("Expected [{}] arguments, but found [{}] in step definition", expected, actual))
          }
        }
      }
    }
  })
}

/// Add a Given step to a [CucumberRegistrar](definitions/registration/trait.CucumberRegistrar.html)
///
/// # Example
/// ```
/// pub fn register_steps(c: &mut CucumberRegistrar<u32>) {
///   Given!(c, "^I have (\\d+) coins$", |_, world: &mut u32, (coin_count,): (u32,)| {
///     *world = coin_count;
///     InvokeResponse::Success
///   });
/// }
/// ```
///
#[macro_export]
macro_rules! Given {
  ($cuke:expr, $regex:expr, $body:expr) => {{
    use $crate::regex;
    $cuke.given(file!(), line!(), regex::build($regex), Box::new(move |cuke, world, args| {
      ($body)(cuke, world, try_destructure!(args))
    }))
  }}
}

/// Add a When step to a [CucumberRegistrar](definitions/registration/trait.CucumberRegistrar.html)
///
/// # Example
/// ```
/// pub fn register_steps(c: &mut CucumberRegistrar<u32>) {
///   When!(c, "^I spend (\\d+) coins$", |_, world: &mut u32, (coin_count,): (u32,)| {
///     if *world - coin_count < 0 {
///       InvokeResponse::fail_from_str("Tried to spend more coins than were owned")
///     } {
///       *world = *world - coin_count;
///       InvokeResponse::Success
///     }
///   });
/// }
/// ```
///
#[macro_export]
macro_rules! When {
  ($cuke:expr, $regex:expr, $body:expr) => {{
    use $crate::regex;
    $cuke.when(file!(), line!(), regex::build($regex), Box::new(move |cuke, world, args| {
      ($body)(cuke, world, try_destructure!(args))
    }))
  }}
}

/// Add a Then step to a [CucumberRegistrar](definitions/registration/trait.CucumberRegistrar.html)
///
/// # Example
/// ```
/// pub fn register_steps(c: &mut CucumberRegistrar<u32>) {
///   Then!(c, "^I have (\\d+) coins left$", |_, world: &mut u32, (coin_count,): (u32,)| {
///     InvokeResponse::check_eq(*world, coin_count)
///   });
/// }
/// ```
///
#[macro_export]
macro_rules! Then {
  ($cuke:expr, $regex:expr, $body:expr) => {{
    use $crate::regex;
    $cuke.then(file!(), line!(), regex::build($regex), Box::new(move |cuke, world, args| {
      ($body)(cuke, world, try_destructure!(args))
    }))
  }}
}

