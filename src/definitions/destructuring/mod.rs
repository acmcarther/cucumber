//! Logic for dissassembling Invoke Arguments
//!
//! This module is not usually used directly -- rather, its work is handled by
//! the
//! [try_destructure! macro](../../macro.try_destructure!.html), which in turn
//! is handled mostly by
//! the [Given!](../../macro.Given!.html), [When!](../../macro.When!.html), and
//! [Then!](../../macro.Then!.html) macros.

pub mod invoke_arg;
pub mod invoke_arg_set;

pub use self::invoke_arg::{Destructurable, FromInvokeArg};

pub use self::invoke_arg_set::{DestructurableSet, FromInvokeArgSet, InvokeArgSetError};
