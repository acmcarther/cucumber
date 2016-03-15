#![cfg_attr(feature = "serde_macros", feature(custom_derive, plugin))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

extern crate regex;
extern crate hyper;
extern crate serde;
extern crate serde_json;

#[macro_use]
mod macros;

pub mod cucumber;
pub mod runner;
mod server;
pub mod helpers;

pub use cucumber::{ Step, Cucumber, CucumberRegistrar, InvokeResponse, InvokeArgument, FailMessage};
pub use runner::{ WorldRunner };
pub use server::{ Server };

pub use macros::*;

#[cfg(test)]
mod test {
  use super::Server;
  use super::cucumber::Response;
  use std::net::TcpStream;

  #[test]
  fn it_makes_a_server() {
    let server = Server::new(|_| {Response::BeginScenario});
    let mut handle = server.start(Some("0.0.0.0:1234"));
    let _ = TcpStream::connect("0.0.0.0:1234").unwrap();

    handle.stop();
    handle.wait();
  }

}
