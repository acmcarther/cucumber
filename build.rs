extern crate syntex;
extern crate serde_codegen;
extern crate itertools;

use std::{env, fs};
use std::path::Path;

use itertools::Itertools;


pub fn main() {
  let out_dir = env::var_os("OUT_DIR").unwrap();

  let paths = vec!["cucumber/request.rs", "cucumber/response.rs"];

  // Don't care if directory already exists
  let _ = fs::create_dir(Path::new(&out_dir).join("cucumber"));

  paths.into_iter().foreach(|path| {
    let src_string = "src/".to_owned() + path + ".in";
    let src = Path::new(&src_string);
    let dst = Path::new(&out_dir).join(path);
    println!("src {}", src.to_str().unwrap());
    println!("dst {}", dst.to_str().unwrap());

    let mut registry = syntex::Registry::new();

    serde_codegen::register(&mut registry);
    registry.expand("cuke", &src, &dst).unwrap();
  });
}
