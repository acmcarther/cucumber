use server::Server;
use definitions::registration::CucumberRegistrar;
use runner::WorldRunner;
use itertools::Itertools;

use std::process::{self, Command, Stdio};
use std::thread;
use std::time::Duration;
use std::env;

#[derive(Default)]
#[must_use = "This struct has to be consumed by start method!"]
pub struct CucumberConfig<'a, W: Send + 'static> {
  world: W,
  addr: &'static str,
  registrar_fns: Vec<&'a Fn(&mut CucumberRegistrar<W>)>,
  args: Vec<&'static str>,
}

/// Build a command to execute the Ruby Cucumber Server.
/// Takes a list of extra command line arguments to the server
/// NOTE: This command also passes the command line arguments passed to this executable.
///
/// # Example
/// ```no_run
/// #[macro_use]
/// extern crate cucumber;
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
///   cucumber::create_config(world)
///             .address("0.0.0.0:12345")
///             .registrar_fn(&button_steps::register_steps)
///             .registrar_fn(&widget_steps::register_steps)
///             .args(&["--format", "pretty", "--expand"])
///             .start();
/// }
/// ```
///
pub fn create_config<'a, W: Send + 'static>(world: W) -> CucumberConfig<'a, W> {
  CucumberConfig {
    world: world,
    addr: "127.0.0.1:7878",
    registrar_fns: Vec::new(),
    args: Vec::new(),
  }
}

impl<'a, W: Send + 'static> CucumberConfig<'a, W> {
  /// Adds a custom ip and port, that will replace the default of localhost:7878
  pub fn address(mut self, address: &'static str) -> CucumberConfig<'a, W> {
    self.addr = address;
    self
  }

  /// Adds a slice of registrar functions.
  pub fn registrar_fns(mut self,
                       registrars: &'a [&Fn(&mut CucumberRegistrar<W>)])
                       -> CucumberConfig<'a, W> {
    self.registrar_fns.extend_from_slice(registrars);
    self
  }

  /// Adds a single registrar functions
  pub fn registrar_fn(mut self,
                      registrar: &'a Fn(&mut CucumberRegistrar<W>))
                      -> CucumberConfig<'a, W> {
    self.registrar_fns.push(registrar);
    self
  }

  /// Adds a slice of arguments that is passed to the ruby client
  pub fn args(mut self, args: &'a [&'static str]) -> CucumberConfig<'a, W> {
    self.args.extend_from_slice(args);
    self
  }

  /// Adds a single argument that is passed to the ruby client
  pub fn arg(mut self, arg: &'static str) -> CucumberConfig<'a, W> {
    self.args.push(arg);
    self
  }

  /// Starts Cucumber and the ruby client using the defined settings.
  #[allow(unused_variables)]
  pub fn start(self) {
    let mut runner = WorldRunner::new(self.world);

    self.registrar_fns.iter().foreach(|fun| fun(&mut runner));

    let server = Server::new(runner);
    // NOTE: Unused stop_rx needs to be held, or it will drop and close the server
    let (handle, stop_rx) = server.start(Some(self.addr));

    let status = ruby_command(self.args)
      .spawn()
      .unwrap_or_else(|e| panic!("failed to execute process: {}. Is Cucumber on path?", e))
      .wait()
      .unwrap();

    // NOTE: Join disabled because of edge case when having zero tests
    //   In that case, ruby cuke will not make tcp connection. It is
    //   so far impossible to break from tcp::accept, so we must kill
    // TODO: Investigate MIO to resolve this
    // handle.join().unwrap();
    // NOTE: Sleep is an interim solution, to allow the thread time to clean up in
    // the typical case
    thread::sleep(Duration::new(2, 0));

    process::exit(status.code().unwrap());
  }
}

/// Build a command to execute the Ruby Cucumber Server
pub fn ruby_command(args: Vec<&'static str>) -> Command {
  let cucumber_bin = if cfg!(target_os = "windows") {
    "cucumber.bat"
  } else {
    "cucumber"
  };

  let mut command = Command::new(cucumber_bin);
  command.stdout(Stdio::inherit());
  command.stderr(Stdio::inherit());
  // Skip the name of the executable, but pass the rest
  env::args().skip(1).foreach(|a| {
    command.arg(a);
  });
  command.args(args.as_slice());
  command
}
