// TODO: Remove this module and move these to appropriate locations, [macro_export]ing therein

#[macro_export]
macro_rules! try_destructure {
  ($r: ident) => ({
    use $crate::cucumber::InvokeResponse;
    use $crate::cucumber::destructuring::{DestructurableSet};

    match $r.destructure_set() {
      Ok(e) => e,
      // TODO: Integrate destructure error information into invoke response
      //   Example: This is a very unfriendly error when FromStr types can't be parsed, ie,
      //   expected a u32 from a plain string of characters
      Err(_) => return InvokeResponse::fail("Arguments in regular expression did not match arguments in step defintion, in count or in type"),
    }
  })
}

// NOTE: These are capitalized to follow Cucumber general conventions, rather than Rust
#[macro_export]
macro_rules! Given {
  ($cuke:ident; $regex:expr, $body:expr) => {{
    use $crate::helpers::r;
    $cuke.given(file!(), line!(), r($regex), Box::new(move |cuke, world, args| {
      ($body)(cuke, world, try_destructure!(args))
    }))
  }}
}

#[macro_export]
macro_rules! When {
  ($cuke:ident; $regex:expr, $body:expr) => {{
    use $crate::helpers::r;
    $cuke.when(file!(), line!(), r($regex), Box::new(move |cuke, world, args| {
      ($body)(cuke, world, try_destructure!(args))
    }))
  }}
}

#[macro_export]
macro_rules! Then {
  ($cuke:ident; $regex:expr, $body:expr) => {{
    use $crate::helpers::r;
    $cuke.then(file!(), line!(), r($regex), Box::new(move |cuke, world, args| {
      ($body)(cuke, world, try_destructure!(args))
    }))
  }}
}
