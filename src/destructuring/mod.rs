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
    use $crate::response::InvokeResponse;
    use $crate::destructuring::{DestructurableSet, InvokeArgSetError};

    match $r.destructure_set() {
      Ok(e) => e,
      // TODO: Integrate destructure error information into invoke response
      //   Example: This is a very unfriendly error when FromStr types can't be parsed, ie,
      //   expected a u32 from a plain string of characters
      Err(error) => {
        match error {
          InvokeArgSetError::TypeMismatch {arg_idx} => {
            return InvokeResponse::with_fail_message(format!("Argument in position [{}] did not have the correct type", arg_idx))
          },
          InvokeArgSetError::ArgCountMismatch {expected, actual} => {
            return InvokeResponse::with_fail_message(format!("Expected [{}] arguments, but found [{}] in step definition", expected, actual))
          }
        }
      }
    }
  })
}
