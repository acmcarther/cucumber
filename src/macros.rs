#[macro_export]
macro_rules! destructured_with_cuke {
  ([$($i:ident: $t:ty),*], $e: expr) => {
    Box::new(|cuke: CucumberState, state: State, args: Vec<InvokeArgument>| {
      $(
      let $i = "hello".to_owned();
      )*
      $e(cuke, state, $($i),*)
    })
  }
}


#[macro_export]
macro_rules! destructured {
  ([$($i:ident: $t:ty),*], $e: expr) => {
    Box::new(|_: CucumberState, state: State, args: Vec<InvokeArgument>| {
      $(
      let $i = "hello".to_owned();
      )*
      $e(state, $($i),*)
    })
  }
}


#[macro_export]
macro_rules! cuke_pop_string {
  ($caps:ident) => {
    match $caps.pop() {
      Some($crate::cucumber::InvokeArgument::String(val)) => val,
      None => return $crate::cucumber::InvokeResponse::fail("Unexpected argument missing in invoke call -- verify step definition arguments near cuke_pop_string!"),
      _ => return $crate::cucumber::InvokeResponse::fail("Unexpected argument type in invoke call, expected String -- verify step definition arguments near cuke_pop_string!")
    }
  }
}


#[macro_export]
macro_rules! cuke_pop_boolean {
  ($caps:ident) => {
    match $caps.pop() {
      Some($crate::cucumber::InvokeArgument::Boolean(val)) => val,
      None => return $crate::cucumber::InvokeResponse::fail("Unexpected argument missing in invoke call -- verify step definition arguments near cuke_pop_boolean!"),
      _ => return $crate::cucumber::InvokeResponse::fail("Unexpected argument type in invoke call, expected bool -- verify step definition arguments near cuke_pop_boolean!")
    }
  }
}


#[macro_export]
macro_rules! cuke_pop_table {
  ($caps:ident) => {
    match $caps.pop() {
      Some($crate::cucumber::InvokeArgument::Table(val)) => val,
      None => return $crate::cucumber::InvokeResponse::fail("Unexpected argument missing in invoke call -- verify step definition arguments near cuke_pop_table!"),
      _ => return $crate::cucumber::InvokeResponse::fail("Unexpected argument type in invoke call, expected Table -- verify step definition arguments near cuke_pop_table!")
    }
  }
}


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
