use crate::{frame::Drawable, background::{Background, BackgroundTile}, MAX_ROWS, MAX_COLS};

pub struct Hero {
  row: usize,
  col: usize,
  previous_row: usize,
  previous_col: usize,
  movement_started: bool,
}

pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Hero {
    pub fn new() -> Self {
      Self {
        row: 29,
        col: 10,
        previous_row: 29,
        previous_col: 10,
        movement_started: false,
      }
    }

    pub fn move_hero(&mut self, direction: Direction, background: &Background) {
      self.movement_started = true;
      self.previous_row = self.row;
      self.previous_col = self.col;
      match direction {
        Direction::Up => {
          if self.row > 0 {
            self.row -= 1;
          }
        },
        Direction::Down => {
          if self.row < MAX_ROWS - 1 {
            self.row += 1;
          }
        },
        Direction::Left => {
          if self.col > 0 {
            self.col -= 1;
          }
        },
        Direction::Right => {
          if self.col < MAX_COLS - 1 {
            self.col += 1;
          }
        }
      }
      if background.real_tiles[self.row][self.col] != BackgroundTile::Corridor {
        self.col = self.previous_col;
        self.row = self.previous_row;
      }
    }

    pub fn get_col(&self) -> usize {
      self.col
    }

    pub fn get_row(&self) -> usize {
      self.row
    }
}

impl Drawable for Hero {
  fn draw(&self, frame: &mut crate::frame::Frame) {
      if self.movement_started {
        frame.tiles[self.previous_row][self.previous_col] = ' ';
      }
      frame.tiles[self.row][self.col] = '*';
  }
}