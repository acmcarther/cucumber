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

#[cfg(test)]
mod test {

  #[test]
  fn fun() {
  }
}
