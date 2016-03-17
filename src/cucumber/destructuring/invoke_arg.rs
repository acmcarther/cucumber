use cucumber::InvokeArgument;

pub trait Destructurable: Sized {
  fn destructure<T: FromInvokeArg>(self) -> Result<T, T::Err>;
}

impl Destructurable for InvokeArgument {
  fn destructure<T: FromInvokeArg>(self) -> Result<T, T::Err> {
    T::from_invoke_arg(self)
  }
}

pub trait FromInvokeArg: Sized {
  type Err;
  fn from_invoke_arg(InvokeArgument) -> Result<Self, Self::Err>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImproperInvokeArgError { _priv: () }

// NOTE: This is a stopgap for impl specialization
//   The underlying issue is that FromInvokeArg can't be impld for all FromStr
//   because it creates a conflicting implementation with String and Boolean
//   Issue: https://github.com/rust-lang/rust/issues/31844
macro_rules! impl_for_from_str {
  ($t: ty) => {
    impl FromInvokeArg for $t {
      type Err = ImproperInvokeArgError;

      fn from_invoke_arg(arg: InvokeArgument) -> Result<$t, ImproperInvokeArgError> {
        match arg {
          InvokeArgument::String(val) => {
            val.parse().map_err(|_| ImproperInvokeArgError { _priv: ()})
          },
          _ => Err(ImproperInvokeArgError { _priv: () })
        }
      }
    }
  }
}

impl_for_from_str!(f32);
impl_for_from_str!(f64);
impl_for_from_str!(isize);
impl_for_from_str!(i16);
impl_for_from_str!(i32);
impl_for_from_str!(i64);
impl_for_from_str!(usize);
impl_for_from_str!(u8);
impl_for_from_str!(u16);
impl_for_from_str!(u32);

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

#[cfg(test)]
mod test {
  use super::*;
  use cucumber::InvokeArgument;

  #[test]
  fn wrong_type_destructure_fails_correctly() {
    let res: Result<bool, ImproperInvokeArgError> = InvokeArgument::String("hello".to_owned()).destructure();

    assert_eq!(res, Err(ImproperInvokeArgError { _priv: () }));
  }

  #[test]
  fn string_can_be_destructured() {
    let res: String = InvokeArgument::String("hello".to_owned()).destructure().unwrap();

    assert_eq!(&res, "hello");
  }


  #[test]
  fn bool_can_be_destructured() {
    let res: bool = InvokeArgument::Boolean(true).destructure().unwrap();

    assert_eq!(res, true);
  }

  #[test]
  fn table_can_be_destructured() {
    let res: Vec<Vec<String>> = InvokeArgument::Table(vec![vec!["hello".to_owned()]]).destructure().unwrap();

    assert_eq!(res, vec![vec!["hello".to_owned()]]);
  }
}
