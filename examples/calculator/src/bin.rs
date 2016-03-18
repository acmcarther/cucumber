extern crate calculator;

use calculator::RWCalculator;
use std::io::{self, BufReader};

fn main() {
  let calc = RWCalculator::new(BufReader::new(io::stdin()), io::stdout());
  calc.run()
}
