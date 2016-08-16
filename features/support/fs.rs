use tempdir::TempDir;
use std::io::{self, Write};
use std::fs::{DirBuilder, File};
use std::env;
use std::process::Command;
use itertools::Itertools;

pub struct Project {
  dir: TempDir,
}

impl Project {
  pub fn set_steps(&mut self, steps: &str) {
    // TODO: Return result
    let full_file =
      format!("
        use cucumber::definitions::registration::CucumberRegistrar;
        use \
               cucumber::event::response::InvokeResponse;
        use cucumber::state::Cucumber;

        \
               #[allow(dead_code)]
        pub fn register_steps(c: &mut CucumberRegistrar<u32>) \
               {{
          {}
        }}
      ",
              steps);
    // TODO: handle
    let _ = File::create(self.dir.path().join("features/steps.rs"))
      .and_then(|mut file| file.write(full_file.as_bytes()));
  }

  pub fn compile(&mut self) -> Result<(), String> {
    Command::new("cargo")
      .arg("test")
      .arg("--no-run")
      .current_dir(self.dir.path())
      .output()
      .map_err(|err| err.to_string())
      .and_then(|output| {
        if !output.status.success() {
          let err_code: i32 = output.status.code().unwrap();
          let out_str: String = String::from_utf8(output.stdout).unwrap();
          let err_str: String = String::from_utf8(output.stderr).unwrap();
          Err(format!("Non-zero exit code for cargo build: {},\n stdout: {},\n stderr:{}",
                      err_code,
                      out_str,
                      err_str))
        } else {
          Ok(())
        }
      })
  }

  pub fn execute_feature(&mut self, scenario: &str, args: &str) -> Result<String, String> {
    File::create(self.dir.path().join("features/test.feature"))
      .and_then(|mut file| file.write(scenario.as_bytes()))
      .map_err(|err| err.to_string())
      .and_then(|_| {
        let mut cmd = Command::new("cargo");

        cmd.arg("test").arg("--test").arg("cuke").arg("--");

        args.split_whitespace().foreach(|a| {
          cmd.arg(a);
        });

        cmd.current_dir(self.dir.path())
          .output()
          .map_err(|err| err.to_string())
          .and_then(|output| {
            if !output.status.success() {
              let err_code: i32 = output.status.code().unwrap();
              let out_str: String = String::from_utf8(output.stdout).unwrap();
              let err_str: String = String::from_utf8(output.stderr).unwrap();
              Err(format!("Non-zero exit code for cargo test: {},\n stdout: {},\n stderr:{}",
                          err_code,
                          out_str,
                          err_str))
            } else {
              Ok(String::from_utf8(output.stdout).unwrap())
            }
          })
      })
  }
}

pub fn create_project() -> io::Result<Project> {
  TempDir::new("cuke_test_proj")
    .and_then(create_cargo_toml)
    .and_then(create_src)
    .and_then(create_features)
    .and_then(bootstrap_target)
    .map(build_project)
}

fn create_cargo_toml(dir: TempDir) -> io::Result<TempDir> {
  File::create(dir.path().join("Cargo.toml"))
    .and_then(|mut file| {
      file.write(format!("
        [package]
        name = \"test_cuke\"
        version = \"0.0.1\"

        [dev-dependencies]
        cucumber = {{ path = \"{}\" }}

        [[test]]
        name = \"cuke\"
        path = \"./features/cuke.rs\"
        harness = false
      ",
                         env::current_dir().unwrap().display())
        .as_bytes())

    })
    .map(|_| dir)
}

fn create_src(dir: TempDir) -> io::Result<TempDir> {
  DirBuilder::new()
    .create(dir.path().join("src"))
    .and_then(|_| File::create(dir.path().join("src/lib.rs")))
    .and_then(|mut file| {
      file.write(b"
        pub fn run() { println!(\"I ran\"); }
      ")
    })
    .map(|_| dir)
}

fn create_features(dir: TempDir) -> io::Result<TempDir> {
  DirBuilder::new()
    .create(dir.path().join("features"))
    .and_then(|_| File::create(dir.path().join("features/cuke.rs")))
    .and_then(|mut file| {
      file.write(b"
        extern crate test_cuke;

        #[macro_use]
        extern crate cucumber;

        mod steps;

        fn main() {
          let world: u32 = 0;
          cucumber::start_with_addr(\"0.0.0.0:7879\", world, &[&steps::register_steps]);
        }
      ")
    })
    .and_then(|_| DirBuilder::new().create(dir.path().join("features/step_definitions")))
    .and_then(|_| File::create(dir.path().join("features/step_definitions/cucumber_rust.wire")))
    .and_then(|mut file| {
      file.write(b"
        host: 0.0.0.0
        port: 7879
      ")
    })
    .map(|_| dir)
}

fn bootstrap_target(dir: TempDir) -> io::Result<TempDir> {
  // TODO: consider a solution using standard Rust
  Command::new("cp")
    .arg(env::current_dir().unwrap().join("target"))
    .arg(dir.path().join("target"))
    .arg("-r")
    .output()
    .map(|_| dir)
}

fn build_project(dir: TempDir) -> Project {
  Project { dir: dir }
}
