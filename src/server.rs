use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

use std::thread;
use std::thread::{JoinHandle};

use std::net::TcpListener;

use serde_json;

use runner::CommandRunner;

use std::sync::mpsc::{Sender, channel, TryRecvError};
use std::sync::{Barrier, Arc};
use std::time::Duration;

use cucumber::Request;

#[allow(dead_code)]
pub struct Server<R: CommandRunner + Send> {
  runner: R,
}

pub struct ServerHandle {
  kill_sender: Sender<()>,
  handle: JoinHandle<()>
}

impl ServerHandle {
  #[allow(dead_code)]
  pub fn stop(&mut self) {
    let _ = self.kill_sender.send(()).unwrap();
  }

  pub fn some_other_function(&self) {}

  #[allow(dead_code)]
  pub fn waits(&mut self) {
    //self.handle.join().unwrap();
  }
}

impl <R: CommandRunner + Send> Server<R> {

  #[allow(dead_code)]
  pub fn new(runner: R) -> Server<R> {
    Server {
      runner: runner
    }
  }

  #[allow(dead_code)]
  pub fn start(mut self, addr: Option<&'static str>) -> ServerHandle
    where R: 'static {
    let addr = addr.unwrap_or("0.0.0.0:7878");
    let (stop_tx, stop_rx) = channel();
    let main_barrier = Arc::new(Barrier::new(2));
    let tcp_barrier = main_barrier.clone();

    let handle = thread::spawn(move || {
      let listener = TcpListener::bind(addr).unwrap();

      // Let the main thread know we're ready
      tcp_barrier.wait();

      // Configure tcp stream
      let (mut stream, _) = listener.accept().unwrap();
      stream.set_read_timeout(Some(Duration::new(1,0))).unwrap();
      let mut buffered_reader = BufReader::new(stream.try_clone().unwrap());

      let mut running = true;
      while running {
        // Check for recv stop signal
        match stop_rx.try_recv() {
          Ok(()) | Err(TryRecvError::Disconnected) => {
            running = false
          },
          _ => {
            // Read request from wire
            let mut body = String::new();
            let _ = buffered_reader.read_line(&mut body).map(|size| {
              if size == 0 {
                running = false
              } else {
                let request = serde_json::from_str::<Request>(&body);

                match request {
                  Ok(req_body) => {
                    let response = self.runner.execute_cmd(req_body);

                    let _ = stream.write(format!("{}\n", serde_json::to_string(&response).unwrap()).as_bytes());
                  }
                  _ => {}
                }
              }
            });
          }
        }
      }
    });

    // Wait for the server thread to have started the TcpListener
    main_barrier.wait();

    ServerHandle { handle: handle, kill_sender: stop_tx }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  use std::net::TcpStream;
  use std::io::Write;
  use std::io::BufReader;
  use std::io::BufRead;

  use cucumber::{Request, Response, InvokeResponse, StepMatchesResponse};

  #[test]
  fn it_makes_a_server() {
    let server = Server::new(|_| {Response::BeginScenario});
    let mut handle = server.start(Some("0.0.0.0:1234"));
    let _ = TcpStream::connect("0.0.0.0:1234").unwrap();

    handle.stop();
    handle.waits();
  }

  #[test]
  fn it_relays_commands_to_the_runner() {
    let server = Server::new(|req| {
      match req {
        Request::BeginScenario(_) => Response::BeginScenario,
        Request::Invoke(_) => Response::Invoke(InvokeResponse::Success),
        Request::StepMatches(_) => Response::StepMatches(StepMatchesResponse::NoMatch),
        Request::EndScenario(_) => Response::EndScenario,
        Request::SnippetText(_) => Response::SnippetText("Snippet".to_owned()),
      }
    });
    let mut handle = server.start(Some("0.0.0.0:1235"));
    let mut stream = TcpStream::connect("0.0.0.0:1235").unwrap();

    {
      stream.write(b"[\"begin_scenario\"]\n").unwrap();
      let mut buffered_reader = BufReader::new(stream.try_clone().unwrap());
      let mut body = String::new();
      buffered_reader.read_line(&mut body).unwrap();
      assert_eq!(body, "[\"success\"]\n");
    }

    {
      stream.write(b"[\"end_scenario\"]\n").unwrap();
      let mut buffered_reader = BufReader::new(stream.try_clone().unwrap());
      let mut body = String::new();
      buffered_reader.read_line(&mut body).unwrap();
      assert_eq!(body, "[\"success\"]\n");
    }

    {
      stream.write(b"[\"invoke\", {\"id\": \"1\", \"args\": []}]\n").unwrap();
      let mut buffered_reader = BufReader::new(stream.try_clone().unwrap());
      let mut body = String::new();
      buffered_reader.read_line(&mut body).unwrap();
      assert_eq!(body, "[\"success\"]\n");
    }

    {
      stream.write(b"[\"step_matches\", {\"name_to_match\": \"test\"}]\n").unwrap();
      let mut buffered_reader = BufReader::new(stream.try_clone().unwrap());
      let mut body = String::new();
      buffered_reader.read_line(&mut body).unwrap();
      assert_eq!(body, "[\"success\",[]]\n");
    }

    {
      stream.write(b"[\"snippet_text\", {\"step_keyword\": \"Given\", \"multiline_arg_class\": \"\", \"step_name\": \"test\"}]\n").unwrap();
      let mut buffered_reader = BufReader::new(stream.try_clone().unwrap());
      let mut body = String::new();
      buffered_reader.read_line(&mut body).unwrap();
      assert_eq!(body, "[\"success\",\"Snippet\"]\n");
    }

    handle.stop();
    handle.waits();
  }
}
