#![feature(custom_derive, plugin, type_ascription)]
#![plugin(serde_macros)]

extern crate regex;
extern crate hyper;
extern crate serde;
extern crate serde_json;

pub mod cucumber;
pub mod runner;
mod server;
pub mod helpers;

pub use cucumber::{ Step, Cucumber, CucumberRegistrar, InvokeResponse, InvokeArgument, FailMessage};
pub use runner::{ WorldRunner };
pub use server::{ Server };

/* Note to me
 * The below facilitates auto step destructuring, consider expanding later 
 *
 *
 *
 */

#[macro_export]
macro_rules! destructured_with_cuke {
  ([$($i:ident: $t:ty),*], $e: expr) => {
    Box::new(|cuke: CucumberState, state: State, args: Vec<InvokeArgument>| {
      $(
      let $i = "hello".to_owned();
      )*
      $e(cuke, state, $($i),*)
    })
  }
}

#[macro_export]
macro_rules! destructured {
  ([$($i:ident: $t:ty),*], $e: expr) => {
    Box::new(|_: CucumberState, state: State, args: Vec<InvokeArgument>| {
      $(
      let $i = "hello".to_owned();
      )*
      $e(state, $($i),*)
    })
  }
}


#[cfg(test)]
mod test{
  pub use cucumber::InvokeArgument;
  pub use cucumber::CucumberState;

  type State = u32;

  #[test]
  fn whatever() {
    let shit = ("sheeeit".to_owned() + (1 + 1 + 2 + 1.5 ): &str): String;
    destructured_with_cuke!( [a: String, b: String], |cuke: CucumberState, state: State, a: String, b: String| {
      println!("{}", state);
    });
    destructured!( [a: String, b: String], |state: State, a: String, b: String| {
      println!("{}", state);
    });
  }
}


#[macro_export]
macro_rules! cuke_pop_string {
  ($caps:ident) => {
    match $caps.pop() {
      Some($crate::cucumber::InvokeArgument::String(val)) => val,
      None => return $crate::cucumber::InvokeResponse::fail("Unexpected argument missing in invoke call -- verify step definition arguments near cuke_pop_string!"),
      _ => return $crate::cucumber::InvokeResponse::fail("Unexpected argument type in invoke call, expected String -- verify step definition arguments near cuke_pop_string!")
    }
  }
}

#[macro_export]
macro_rules! cuke_pop_boolean {
  ($caps:ident) => {
    match $caps.pop() {
      Some($crate::cucumber::InvokeArgument::Boolean(val)) => val,
      None => return $crate::cucumber::InvokeResponse::fail("Unexpected argument missing in invoke call -- verify step definition arguments near cuke_pop_boolean!"),
      _ => return $crate::cucumber::InvokeResponse::fail("Unexpected argument type in invoke call, expected bool -- verify step definition arguments near cuke_pop_boolean!")
    }
  }
}

#[macro_export]
macro_rules! cuke_pop_table {
  ($caps:ident) => {
    match $caps.pop() {
      Some($crate::cucumber::InvokeArgument::Table(val)) => val,
      None => return $crate::cucumber::InvokeResponse::fail("Unexpected argument missing in invoke call -- verify step definition arguments near cuke_pop_table!"),
      _ => return $crate::cucumber::InvokeResponse::fail("Unexpected argument type in invoke call, expected Table -- verify step definition arguments near cuke_pop_table!")
    }
  }
}


#[macro_export]
macro_rules! cuke_extract_tuple {
  ($caps:ident, $($p:pat),+) => {
    ($(
      cuke_extract!($caps, $p)
    ),+)
  }
}

#[macro_export]
macro_rules! Given {
  ($cuke:ident, $regex:expr, $body:expr) => {
    $cuke.given(file!(), line!(), $regex, $body)
  }
}

#[macro_export]
macro_rules! When {
  ($cuke:ident, $regex:expr, $body:expr) => {
    $cuke.when(file!(), line!(), $regex, $body)
  }
}

#[macro_export]
macro_rules! Then {
  ($cuke:ident, $regex:expr, $body:expr) => {
    $cuke.then(file!(), line!(), $regex, $body)
  }
}
