use calculator::{Calculator, CalculatorPushResponse};

pub struct CalculatorWorld {
  pub calculator: Calculator,
  pub last_response: Option<CalculatorPushResponse>,
}

impl CalculatorWorld {
  pub fn new() -> CalculatorWorld {
    CalculatorWorld {
      calculator: Calculator::new(),
      last_response: None,
    }
  }
}
