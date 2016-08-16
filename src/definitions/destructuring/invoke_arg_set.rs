use event::request::InvokeArgument;
use super::FromInvokeArg;

pub trait FromInvokeArgSet: Sized {
  fn from_invoke_arg_set(Vec<InvokeArgument>) -> Result<Self, InvokeArgSetError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum InvokeArgSetError {
  // TODO: Investigate returning expected and actual type tokens here
  TypeMismatch {
    arg_idx: u32,
  },
  ArgCountMismatch {
    expected: usize,
    actual: usize,
  },
}

pub trait DestructurableSet: Sized {
  fn destructure_set<T: FromInvokeArgSet>(self) -> Result<T, InvokeArgSetError>;
}

impl DestructurableSet for Vec<InvokeArgument> {
  fn destructure_set<T: FromInvokeArgSet>(self) -> Result<T, InvokeArgSetError> {
    T::from_invoke_arg_set(self)
  }
}

macro_rules! auto_define_for_tuple {
  ($count:expr; [$($t: ident),+]) => {
    impl <$($t,)+> FromInvokeArgSet for ($($t,)+) where $($t: FromInvokeArg),+ {
// Ignore counter being set by the last tuple
      #[allow(unused_assignments)]
      fn from_invoke_arg_set(args: Vec<InvokeArgument>) -> Result<($($t,)+), InvokeArgSetError> {
        if args.len() != $count { return Err(InvokeArgSetError::ArgCountMismatch {expected: $count, actual: args.len() }) }

        let mut arg_iter = args.into_iter();

// Used for macro to know which arg a failure occurs on
//   Normally, this could be known statically, but macro can't count type args
        let mut counter = 0;

        Ok(( $({
            let res =
              $t::from_invoke_arg(arg_iter.next().unwrap())
                .map_err(|_| InvokeArgSetError::TypeMismatch {arg_idx: counter});
             counter = counter + 1;
             try!(res)
           },)+
         ))
      }
    }
  }
}

impl FromInvokeArgSet for () {
  fn from_invoke_arg_set(args: Vec<InvokeArgument>) -> Result<(), InvokeArgSetError> {
    if args.len() != 0 {
      return Err(InvokeArgSetError::ArgCountMismatch {
        expected: 0,
        actual: args.len(),
      });
    }
    Ok(())
  }
}

auto_define_for_tuple!(1;  [A]);
auto_define_for_tuple!(2;  [A, B]);
auto_define_for_tuple!(3;  [A, B, C]);
auto_define_for_tuple!(4;  [A, B, C, D]);
auto_define_for_tuple!(5;  [A, B, C, D, E]);
auto_define_for_tuple!(6;  [A, B, C, D, E, F]);
auto_define_for_tuple!(7;  [A, B, C, D, E, F, G]);
auto_define_for_tuple!(8;  [A, B, C, D, E, F, G, H]);
auto_define_for_tuple!(9;  [A, B, C, D, E, F, G, H, I]);
auto_define_for_tuple!(10; [A, B, C, D, E, F, G, H, I, J]);
auto_define_for_tuple!(11; [A, B, C, D, E, F, G, H, I, J, K]);
auto_define_for_tuple!(12; [A, B, C, D, E, F, G, H, I, J, K, L]);

#[cfg(test)]
mod test {
  use super::*;
  use event::request::InvokeArgument;

  #[test]
  fn tuple_1_string_can_be_destructured() {
    let res = vec![InvokeArgument::String("hello".to_owned())].destructure_set();

    let (x,): (String,) = res.unwrap();

    assert_eq!(&x, "hello");
  }

  #[test]
  fn tuple_1_bool_can_be_destructured() {
    let res = vec![InvokeArgument::String("false".to_owned())].destructure_set();

    let (x,): (bool,) = res.unwrap();

    assert_eq!(x, false);
  }

  #[test]
  fn tuple_3_can_be_destructured() {
    let res = vec![InvokeArgument::String("hello".to_owned()),
                   InvokeArgument::String("world".to_owned()),
                   InvokeArgument::String("hello".to_owned())]
      .destructure_set();

    let (x, y, z): (String, String, String) = res.unwrap();

    assert_eq!(&x, "hello");
    assert_eq!(&y, "world");
    assert_eq!(&z, "hello");
  }

  #[test]
  fn destructure_for_element_count_fails_correctly() {
    let res: Result<(String, String, String), InvokeArgSetError> =
      vec![InvokeArgument::String("hello".to_owned())].destructure_set();

    assert_eq!(res,
               Err(InvokeArgSetError::ArgCountMismatch {
                 expected: 3,
                 actual: 1,
               }));
  }

  #[test]
  fn destructure_for_type_mismatch_fails_correctly() {
    let res: Result<(bool, u32), InvokeArgSetError> =
      vec![InvokeArgument::String("true".to_owned()),
           InvokeArgument::String("not a u32".to_owned())]
        .destructure_set();

    assert_eq!(res, Err(InvokeArgSetError::TypeMismatch { arg_idx: 1 }));
  }
}
