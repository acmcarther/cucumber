mod invoke_arg_set;

pub use self::invoke_arg_set::{
  InvokeArgSetError,
  FromInvokeArgSet,
};

use cucumber::InvokeArgument;

pub trait FromInvokeArg: Sized {
  type Err;
  fn from_invoke_arg(InvokeArgument) -> Result<Self, Self::Err>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImproperInvokeArgError { _priv: () }

impl FromInvokeArg for String {
  type Err = ImproperInvokeArgError;

  fn from_invoke_arg(arg: InvokeArgument) -> Result<String, ImproperInvokeArgError> {
    match arg {
      InvokeArgument::String(val) => Ok(val),
      _ => Err(ImproperInvokeArgError { _priv: () })
    }
  }
}

impl FromInvokeArg for Vec<Vec<String>> {
  type Err = ImproperInvokeArgError;

  fn from_invoke_arg(arg: InvokeArgument) -> Result<Vec<Vec<String>>, ImproperInvokeArgError> {
    match arg {
      InvokeArgument::Table(val) => Ok(val),
      _ => Err(ImproperInvokeArgError { _priv: () })
    }
  }
}

impl FromInvokeArg for bool {
  type Err = ImproperInvokeArgError;

  fn from_invoke_arg(arg: InvokeArgument) -> Result<bool, ImproperInvokeArgError> {
    match arg {
      InvokeArgument::Boolean(val) => Ok(val),
      _ => Err(ImproperInvokeArgError { _priv: () })
    }
  }
}
