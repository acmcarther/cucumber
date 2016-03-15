#[macro_export]
macro_rules! try_destructure {
  ($r: ident) => ({
    use $crate::cucumber::InvokeResponse;
    use $crate::cucumber::destructuring::{DestructurableSet};

    match $r.destructure_set() {
      Ok(e) => e,
      // TODO: Integrate destructure error information into invoke response
      //   On second thought, this might be impossible because the error is an associated type
      Err(_) => return InvokeResponse::fail("Arguments in regular expression did not match arguments in step defintion"),
    }
  })
}

// NOTE: These are capitalized to follow Cucumber general conventions, rather than Rust
#[macro_export]
macro_rules! Given {
  ($cuke:ident, $regex:expr, $body:expr) => {
    $cuke.given(file!(), line!(), $regex, $body)
  }
}


#[macro_export]
macro_rules! When {
  ($cuke:ident, $regex:expr, $body:expr) => {
    $cuke.when(file!(), line!(), $regex, $body)
  }
}


#[macro_export]
macro_rules! Then {
  ($cuke:ident, $regex:expr, $body:expr) => {
    $cuke.then(file!(), line!(), $regex, $body)
  }
}
