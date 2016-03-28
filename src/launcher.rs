use server::Server;
use definitions::registration::CucumberRegistrar;
use runner::WorldRunner;
use itertools::Itertools;

use std::process::{self, Command, Stdio};

#[allow(unused_variables)]
pub fn start<W: Send + 'static>(world: W, register_fns: &[&Fn(&mut CucumberRegistrar<W>)]) {
  let mut runner = WorldRunner::new(world);

  register_fns.iter().foreach(|fun| fun(&mut runner));

  let server = Server::new(runner);
  // NOTE: Unused stop_rx needs to be held, or it will drop and close the server
  let (handle, stop_rx) = server.start(Some("0.0.0.0:7878"));

  let status = ruby_command()
    .spawn()
    .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) })
    .wait().unwrap();

  // NOTE: Join disabled because of edge case when having zero tests
  //   In that case, ruby cuke will not make tcp connection. It is
  //   so far impossible to break from tcp::accept, so we must kill
  // TODO: Investigate MIO to resolve this
  // handle.join().unwrap();

  process::exit(status.code().unwrap());
}

pub fn ruby_command() -> Command {
  let mut command = Command::new("cucumber");
  command.stdout(Stdio::inherit());
  command.stderr(Stdio::inherit());
  command
}

