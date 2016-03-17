extern crate syntex;
extern crate serde_codegen;
extern crate itertools;

use std::{env, fs};
use std::path::Path;

use itertools::Itertools;

// Custom build script using Syntex for using compiler plugins on Stable
pub fn main() {
  let out_dir = env::var_os("OUT_DIR").unwrap();

  // Files that contain some compiler plugin using code
  let paths = vec!["cucumber/request.rs", "cucumber/response.rs"];

  // Don't care if directory already exists
  let _ = fs::create_dir(Path::new(&out_dir).join("cucumber"));

  paths.into_iter().foreach(|path| {
    let src_string = "src/".to_owned() + path + ".in";
    let src = Path::new(&src_string);
    let dst = Path::new(&out_dir).join(path);

    let mut registry = syntex::Registry::new();

    serde_codegen::register(&mut registry);
    registry.expand("cuke", &src, &dst).unwrap();
  });
}