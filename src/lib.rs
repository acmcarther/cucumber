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

#[macro_use]
mod macros;

pub use cucumber::{ Step, Cucumber, CucumberRegistrar, InvokeResponse, InvokeArgument, FailMessage};
pub use runner::{ WorldRunner };
pub use server::{ Server };

pub use macros::*;

/* Note to me
 * The below facilitates auto step destructuring, consider expanding later 
 *
 *
 *
 */

#[cfg(test)]
mod test{
  pub use cucumber::InvokeArgument;
  pub use cucumber::CucumberState;

  type State = u32;

  #[test]
  fn whatever() {
    destructured_with_cuke!( [a: String, b: String], |cuke: CucumberState, state: State, a: String, b: String| {
      println!("{}", state);
    });
    destructured!( [a: String, b: String], |state: State, a: String, b: String| {
      println!("{}", state);
    });
  }
}



