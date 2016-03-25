extern crate syntex;
extern crate serde_codegen;
extern crate itertools;

use std::env;
use std::path::Path;

use itertools::Itertools;

// Custom build script using Syntex for using compiler plugins on Stable
pub fn main() {
  let out_dir = env::var_os("OUT_DIR").unwrap();

  let files = vec!["request.rs", "response.rs"];

  files.into_iter().foreach(|file| {
    let src_string = "src/".to_owned() + file +".in";
    let src = Path::new(&src_string);
    let dst = Path::new(&out_dir).join(file);

    let mut registry = syntex::Registry::new();

    serde_codegen::register(&mut registry);
    registry.expand("cuke_event", &src, &dst).unwrap();
  })
}
