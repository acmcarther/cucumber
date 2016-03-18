use std::fmt::{self};
use std::io::{BufRead, Write};
use std::str::FromStr;

pub struct Calculator {
  command_buffer: Vec<CalculatorCommand>
}

pub enum CalculatorOperation { Add, Subtract }
pub enum CalculatorCommand { Add, Minus, Number(i32) }
pub enum CalculatorPushResponse { Success, Failure(String) }

impl fmt::Display for CalculatorPushResponse {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &CalculatorPushResponse::Success => write!(f, "Success\n"),
      &CalculatorPushResponse::Failure(ref val) => write!(f, "Failure({})\n", val)
    }
  }
}


impl Calculator {
  pub fn new() -> Calculator {
    Calculator { command_buffer: Vec::new() }
  }

  pub fn clear(&mut self) {
    self.command_buffer.clear()
  }

  pub fn push_command(&mut self, command: CalculatorCommand) -> CalculatorPushResponse {
    match command {
      e @ CalculatorCommand::Number(_) => {
        if self.command_buffer.len() == 0 {
          CalculatorPushResponse::Failure("Input operator first".to_owned())
        } else {
          self.command_buffer.push(e);
          CalculatorPushResponse::Success
        }
      },
      e @ _ => {
        self.command_buffer.push(e);
        CalculatorPushResponse::Success
      }
    }
  }

  pub fn evaluate(&mut self) -> i32 {
    let state = (CalculatorOperation::Add, None);
    let (_, result) = self.command_buffer.iter().fold(state, |(op, val), cmd| {
      match cmd {
        &CalculatorCommand::Number(num) => {
          let new_val = val.map(|v| match op {
            CalculatorOperation::Add => v + num,
            CalculatorOperation::Subtract => v - num
          }).or(Some(num));
          (op, new_val)
        },
        &CalculatorCommand::Add => (CalculatorOperation::Add, val),
        &CalculatorCommand::Minus => (CalculatorOperation::Subtract, val)
      }
    });

    self.clear();
    result.unwrap_or(0)
  }
}

pub struct RWCalculator<R: BufRead, W: Write> {
  calculator: Calculator,
  reader: R,
  writer: W
}

impl <R: BufRead, W: Write> RWCalculator<R, W> {
  pub fn new(reader: R, writer: W) -> RWCalculator<R, W> {
    RWCalculator {
      calculator: Calculator::new(),
      reader: reader,
      writer: writer
    }
  }

  fn write(&mut self, str: String) {
    let _ = self.writer.write(str.as_bytes());
  }

  pub fn run(mut self) {
    let mut buffer = String::new();
    let mut running = true;
    while running {
      let res = self.reader.read_line(&mut buffer);
      if res.is_err() { return; }

      buffer.pop(); // Remove newline
      let message = match buffer.as_ref() {
        "exit" => {
          running = false;
          "exiting".to_owned()
        },
        "+" | "add" | "plus" => self.calculator.push_command(CalculatorCommand::Add).to_string(),
        "-" | "subtract" | "minus" => self.calculator.push_command(CalculatorCommand::Minus).to_string(),
        "equals" => self.calculator.evaluate().to_string() + "\n",
        other @ _ => {
          let number_res: Result<i32, <i32 as FromStr>::Err> = other.parse();
          match number_res {
            Err(_) => format!("Unknown command: {}\n", other),
            Ok(value) => self.calculator.push_command(CalculatorCommand::Number(value)).to_string()
          }
        }
      };

      self.write(message);

      buffer.clear();
    }
  }
}
