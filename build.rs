#[cfg(not(feature = "serde_macros"))]
mod inner {
  extern crate serde_codegen;
  extern crate itertools;

  use std::env;
  use std::path::Path;
  use std::fs;

  use inner::itertools::Itertools;

  pub fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let files = vec!["event/request.rs", "event/response.rs"];

    files.into_iter().foreach(|file| {
      let src_string = "src/".to_owned() + file +".in";
      let src = Path::new(&src_string);
      println!("src {:?}", src);
      let dst = Path::new(&out_dir).join(file);
      println!("dst {:?}", dst);

      // Don't care if directory already exists
      let _ = fs::create_dir(Path::new(&out_dir).join("event"));

      serde_codegen::expand(&src, &dst).unwrap();
    })
  }
}

#[cfg(feature = "serde_macros")]
mod inner {
    pub fn main() {}
}

fn main() {
    inner::main();
}
