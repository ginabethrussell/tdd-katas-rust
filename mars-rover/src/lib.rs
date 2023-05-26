pub struct Rover {}

impl Rover {
  pub fn new() -> Self {
    Self {}
  }
  pub fn execute(&self, _commands: &str) -> String {
    String::from("0:0:N")
  }
}
