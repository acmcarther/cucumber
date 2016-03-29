use server::Server;
use definitions::registration::CucumberRegistrar;
use runner::WorldRunner;
use itertools::Itertools;

use std::process::{self, Command, Stdio};
use std::thread;
use std::time::Duration;

/// Starts a Cucumber server and the Ruby client
///
/// # Example
/// ```no_run
/// #[macro_use]
/// extern crate cucumber;
///
///
/// mod button_steps {
///   use cucumber::CucumberRegistrar;
///   pub fn register_steps(c: &mut CucumberRegistrar<u32>) {
///   }
/// }
///
/// mod widget_steps {
///   use cucumber::CucumberRegistrar;
///   pub fn register_steps(c: &mut CucumberRegistrar<u32>) {
///   }
/// }
///
/// fn main() {
///   let world: u32 = 0;
///
///   cucumber::start(
///     world,
///     &[
///       &button_steps::register_steps,
///       &widget_steps::register_steps,
///     ]
///   );
/// }
/// ```
///
pub fn start<W: Send + 'static>(world: W, register_fns: &[&Fn(&mut CucumberRegistrar<W>)]) {
  start_with_addr("0.0.0.0:7878", world, register_fns)
}

/// Start a Cucumber server, with an ip and port, see the [`start() method`][start].
/// [start]: fn.start.html
#[allow(unused_variables)]
pub fn start_with_addr<W: Send + 'static>(addr: &'static str, world: W, register_fns: &[&Fn(&mut CucumberRegistrar<W>)]) {
  let mut runner = WorldRunner::new(world);

  register_fns.iter().foreach(|fun| fun(&mut runner));

  let server = Server::new(runner);
  // NOTE: Unused stop_rx needs to be held, or it will drop and close the server
  let (handle, stop_rx) = server.start(Some(addr));

  let status = ruby_command()
    .spawn()
    .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) })
    .wait().unwrap();

  // NOTE: Join disabled because of edge case when having zero tests
  //   In that case, ruby cuke will not make tcp connection. It is
  //   so far impossible to break from tcp::accept, so we must kill
  // TODO: Investigate MIO to resolve this
  //handle.join().unwrap();
  // NOTE: Sleep is an interim solution, to allow the thread time to clean up in the typical case
  thread::sleep(Duration::new(2, 0));

  process::exit(status.code().unwrap());
}

/// Build a command to execute the Ruby Cucumber Server
pub fn ruby_command() -> Command {
  let mut command = Command::new("cucumber");
  command.stdout(Stdio::inherit());
  command.stderr(Stdio::inherit());
  command
}

