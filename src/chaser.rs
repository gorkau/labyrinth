use crate::{hero::Hero, background::{Background, BackgroundTile}, MAX_ROWS, frame::Drawable, MAX_COLS};

pub struct Chaser {
  row: usize,
  col: usize,
  previous_row: usize,
  previous_col: usize,
  movement_started: bool,
}

impl Chaser {

  pub fn new() -> Self {
    Self {
      row: 19,
      col: 1,
      previous_row: 21,
      previous_col: 1,
      movement_started: false,
    }
  }

  pub fn move_chaser(&mut self, hero: &Hero, background: &Background) {
    self.previous_row = self.row;
    self.previous_col = self.col;
    let mut moved = false;

    if self.row < hero.get_row() && self.row < MAX_ROWS - 1 {
      self.row += 1;
      moved = true;
    }
    else if self.row > hero.get_row() && self.row > 0 {
      self.row -= 1;
      moved = true;
    }

    if background.real_tiles[self.row][self.col] != BackgroundTile::Corridor {
      self.row = self.previous_row;
      moved = false;
    }

    if !moved && self.col < hero.get_col() && self.col < MAX_COLS - 1 {
      self.col += 1;
    }
    else if !moved && self.col > hero.get_col() && self.col > 0 {
      self.col -= 1;
    }

    if background.real_tiles[self.row][self.col] != BackgroundTile::Corridor {
      self.col = self.previous_col;
    }

  }

  pub fn hero_reached(&self, hero :&Hero) -> bool {
    hero.get_col() == self.col && hero.get_row() == self.row
  }
}

impl Drawable for Chaser {
  fn draw(&self, frame: &mut crate::frame::Frame) {
    if self.movement_started {
      frame.tiles[self.previous_row][self.previous_col] = ' ';
    }
    frame.tiles[self.row][self.col] = 'A';
  }
}
