extern crate simple;

#[macro_use]
extern crate cucumber;

#[test]
fn main() {
  let world: u32 = 0;
  cucumber::start(world, &[]);
}
