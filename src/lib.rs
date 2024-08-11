#![deny(clippy::all)]

use napi::Either;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn print(value: Either<bool, String>) {
  println!("{:?}", value);
}
