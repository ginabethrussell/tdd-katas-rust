use std::fmt::{Display, Formatter, Result};

pub struct Coordinates {
  x: u8,
  y: u8,
}

const GRID_SIZE: u8 = 10;

impl Coordinates {
  pub fn new() -> Self {
    Self {
      x: 0,
      y: 0,
    }
  }
  pub fn move_towards(&self, cardinal: &Cardinal) -> Coordinates {
    match cardinal {
      Cardinal::North => {
        Self {
          x: self.x,
          y: if self.y == GRID_SIZE {
            0
          } else {
            self.y + 1
          }
        }
      },
      Cardinal::South => {
        Self {
          x: self.x,
          y: if self.y == 0{
            GRID_SIZE
          } else {
            self.y - 1
          }
        }
      },
      Cardinal::East => {
        Self {
          x: if self.x == GRID_SIZE {
            0
          } else {
            self.x + 1
          },
          y: self.y
        }
      },
      Cardinal::West => {
        Self {
          x: if self.x == 0 {
            GRID_SIZE
          } else {
            self.x - 1
          },
          y: self.y
        }
      },
    }
  }
}

impl Display for Coordinates {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}:{}",self.x, self.y)
  }
}

pub enum Cardinal {
  North,
  West,
  East,
  South,
}

impl Cardinal {
  pub fn right(&self) -> Self {
    match self {
      Cardinal::North => Cardinal::East,
      Cardinal::East => Cardinal::South,
      Cardinal::South => Cardinal::West,
      Cardinal::West => Cardinal::North,
    }
  }
  pub fn left(&self) -> Self {
    match self {
      Cardinal::North => Cardinal::West,
      Cardinal::East => Cardinal::North,
      Cardinal::South => Cardinal::East,
      Cardinal::West => Cardinal::South,
    }
  }
}

impl Display for Cardinal {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", match *self {
      Cardinal::North => 'N',
      Cardinal::East => 'E',
      Cardinal::South => 'S',
      Cardinal::West => 'W',
    })
  }
}
