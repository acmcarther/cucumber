pub mod invoke_arg;
pub mod invoke_arg_set;

pub use self::invoke_arg::{
  FromInvokeArg,
  Destructurable,
};

pub use self::invoke_arg_set::{
  InvokeArgSetError,
  FromInvokeArgSet,
  DestructurableSet,
};

#[macro_export]
macro_rules! try_destructure {
  ($r: ident) => ({
    use $crate::InvokeResponse;
    use $crate::destructuring::{DestructurableSet};

    match $r.destructure_set() {
      Ok(e) => e,
      // TODO: Integrate destructure error information into invoke response
      //   Example: This is a very unfriendly error when FromStr types can't be parsed, ie,
      //   expected a u32 from a plain string of characters
      Err(_) => return InvokeResponse::with_fail_message("Arguments in regular expression did not match arguments in step defintion, in count or in type"),
    }
  })
}
