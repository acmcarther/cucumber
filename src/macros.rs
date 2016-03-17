// TODO: Remove this module and move these to appropriate locations, [macro_export]ing therein

#[macro_export]
macro_rules! try_destructure {
  ($r: ident) => ({
    use $crate::cucumber::InvokeResponse;
    use $crate::cucumber::destructuring::{DestructurableSet};

    match $r.destructure_set() {
      Ok(e) => e,
      // TODO: Integrate destructure error information into invoke response
      Err(_) => return InvokeResponse::fail("Arguments in regular expression did not match arguments in step defintion"),
    }
  })
}

// NOTE: These are capitalized to follow Cucumber general conventions, rather than Rust
#[macro_export]
macro_rules! Given {
  ($cuke:ident; $regex:expr, $body:expr) => {{
    use $crate::helpers::r;
    $cuke.given(file!(), line!(), r($regex), Box::new($body))
  }}
}

#[macro_export]
macro_rules! When {
  ($cuke:ident; $regex:expr, $body:expr) => {{
    use $crate::helpers::r;
    $cuke.when(file!(), line!(), r($regex), Box::new($body))
  }}
}

#[macro_export]
macro_rules! Then {

  ($cuke:ident; $regex:expr, $body:expr) => {{
    use $crate::helpers::r;
    $cuke.then(file!(), line!(), r($regex), Box::new($body))
  }}
}
