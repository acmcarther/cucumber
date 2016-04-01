#![doc(html_root_url = "https://acmcarther.github.io/cucumber-rs/")]

#![cfg_attr(nightly, feature(custom_derive, plugin))]
#![cfg_attr(nightly, plugin(serde_macros))]

extern crate regex;
extern crate itertools;
extern crate serde;
extern crate serde_json;

/// Low level location of step functions and matcher logic
pub mod state;

/// External facing interface to other Gherkin implementations
pub mod server;

/// Coordinator logic between [server](server/index.html) and [state](state/index.html)
pub mod runner;

/// Business logic for step registration and invoke argument destructuring
pub mod definitions;

/// External facing interface for events
pub mod event;

/// Helpers for regular expressions
pub mod cucumber_regex;

mod launcher;

pub use launcher::{
  start,
  start_with_addr,
  ruby_command
};

pub use runner::{WorldRunner, CommandRunner};
pub use definitions::registration::CucumberRegistrar;
pub use state::{Cucumber, SendableStep};
pub use server::Server;
pub use event::request::InvokeArgument;
pub use event::response::InvokeResponse;

/// Destructure a vector of [InvokeArgument](event/request/enum.InvokeArgument.html) into a tuple of values, or a bad [InvokeResponse](event/response/enum.InvokeResponse.html), similar to normal try!
///
/// Will either short circult return an InvokeArgument::Fail, describing either improper arg count
/// or improper arg type, or will yield the tuple of values
///
/// Reminder: Tuple of one value is represented as `(t,): (Type,)`
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate cucumber;
///
/// use cucumber::{
///   InvokeArgument,
///   InvokeResponse
/// };
///
/// fn main() {
///   fn do_work() -> InvokeResponse {
///     let args = vec![
///       InvokeArgument::from_str("1"),
///       InvokeArgument::from_str("2"),
///       InvokeArgument::None
///     ];
///     let (x, y, z): (u32, u32, bool) = try_destructure!(args);
///
///     if x == 1 && y == 2 && z == false {
///       InvokeResponse::Success
///     } else {
///       InvokeResponse::fail_from_str("Values did not match")
///     }
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
/// #[macro_use]
/// extern crate cucumber;
///
/// use cucumber::{
///   InvokeResponse,
///   CucumberRegistrar,
///   Cucumber
/// };
///
/// pub fn main () {
///   let mut cucumber: Cucumber<u32> = Cucumber::new();
///
///   Given!(cucumber, "^I have (\\d+) coins$", |_, world: &mut u32, (coin_count,): (u32,)| {
///     *world = coin_count;
///     InvokeResponse::Success
///   });
/// }
/// ```
///
#[macro_export]
macro_rules! Given {
  ($cuke:expr, $regex:expr, $body:expr) => {{
    use $crate::cucumber_regex;
    $cuke.given(file!(), line!(), cucumber_regex::build($regex), Box::new(move |cuke, world, args| {
      ($body)(cuke, world, try_destructure!(args))
    }))
  }}
}

/// Add a When step to a [CucumberRegistrar](definitions/registration/trait.CucumberRegistrar.html)
///
/// # Example
/// ```
/// #[macro_use]
/// extern crate cucumber;
///
/// use cucumber::{
///   InvokeResponse,
///   CucumberRegistrar,
///   Cucumber
/// };
///
/// pub fn main () {
///   let mut cucumber: Cucumber<u32> = Cucumber::new();
///
///   When!(cucumber, "^I spend (\\d+) coins$", |_, world: &mut u32, (coin_count,): (u32,)| {
///     if *world - coin_count < 0 {
///       InvokeResponse::fail_from_str("Tried to spend more coins than were owned")
///     } else {
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
    use $crate::cucumber_regex;
    $cuke.when(file!(), line!(), cucumber_regex::build($regex), Box::new(move |cuke, world, args| {
      ($body)(cuke, world, try_destructure!(args))
    }))
  }}
}

/// Add a Then step to a [CucumberRegistrar](definitions/registration/trait.CucumberRegistrar.html)
///
/// # Example
/// ```
/// #[macro_use]
/// extern crate cucumber;
///
/// use cucumber::{
///   InvokeResponse,
///   CucumberRegistrar,
///   Cucumber
/// };
///
/// pub fn main () {
///   let mut cucumber: Cucumber<u32> = Cucumber::new();
///
///   Then!(cucumber, "^I have (\\d+) coins left$", |_, world: &mut u32, (coin_count,): (u32,)| {
///     InvokeResponse::check_eq(*world, coin_count)
///   });
/// }
/// ```
///
#[macro_export]
macro_rules! Then {
  ($cuke:expr, $regex:expr, $body:expr) => {{
    use $crate::cucumber_regex;
    $cuke.then(file!(), line!(), cucumber_regex::build($regex), Box::new(move |cuke, world, args| {
      ($body)(cuke, world, try_destructure!(args))
    }))
  }}
}

