use cucumber::InvokeArgument;
use super::FromInvokeArg;

pub trait FromInvokeArgSet: Sized {
  type Err;
  fn from_invoke_arg_set(Vec<InvokeArgument>) -> Result<Self, Self::Err>;
}

pub enum InvokeArgSetError {
  FailedInvokeArgConvert {arg_idx: u32},
  ImproperInvokeArgCount {expected_count: usize, actual_count: usize },
}


macro_rules! auto_destructure {
  ($args: ident; $count: expr; [$($t:ident),+]) => {
    {
      //
      if $args.len() != $count { return Err(InvokeArgSetError::ImproperInvokeArgCount {expected_count: $count, actual_count: $args.len() }) }

      // Used for macro to know which arg a failure occurs on
      let mut counter = 0;

      Ok((
        $({
          let res =
            $t::from_invoke_arg($args.pop().unwrap())
              .map_err(|_| InvokeArgSetError::FailedInvokeArgConvert {arg_idx: counter});
           counter = counter + 1;
           try!(res)
         },)+
       ))
     }
  }
}

macro_rules! define_for_tuple {
  ($count:expr; [$($t: ident),+]) => {
    impl <$($t,)+> FromInvokeArgSet for ($($t,)+) where $($t: FromInvokeArg),+ {
      type Err = InvokeArgSetError;

      #[allow(unused_assignments)]
      fn from_invoke_arg_set(mut args: Vec<InvokeArgument>) -> Result<($($t,)+), InvokeArgSetError> {
        auto_destructure!(args; $count; [$($t),+])
      }
    }
  }
}


impl <A> FromInvokeArgSet for (A,) where A: FromInvokeArg {
  type Err = InvokeArgSetError;

  #[allow(unused_assignments)]
  fn from_invoke_arg_set(mut args: Vec<InvokeArgument>) -> Result<(A,), InvokeArgSetError> {
    auto_destructure!(args; 1; [A])
  }
}

impl <A, B> FromInvokeArgSet for (A,B) where A: FromInvokeArg, B: FromInvokeArg {
  type Err = InvokeArgSetError;

  #[allow(unused_assignments)]
  fn from_invoke_arg_set(mut args: Vec<InvokeArgument>) -> Result<(A,B), InvokeArgSetError> {
    auto_destructure!(args; 2; [A, B])
  }
}

define_for_tuple!(3; [A, B, C]);
