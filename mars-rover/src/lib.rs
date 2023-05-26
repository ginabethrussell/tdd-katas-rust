use crate::geolocation::{Cardinal, Coordinates};
mod geolocation;

pub struct Rover {
  cardinal: Cardinal,
  coordinates: Coordinates
}

impl Rover {
  pub fn new() -> Self {
    Self {
      cardinal: Cardinal::North,
      coordinates: Coordinates::new()
    }
  }
  pub fn execute(&mut self, commands: &str) -> String {
    for command in commands.chars() {
      match command {
        'R' => {
          self.cardinal = self.cardinal.right();
        },
        'L' => {
          self.cardinal = self.cardinal.left()
        },
        'M' => {
          self.coordinates = self.coordinates.move_towards(&self.cardinal)
        },
        _ => todo!("Unknown command")
      }
    }
    format!("{}:{}", self.coordinates, self.cardinal)
  }
}
