use crate::{MAX_COLS, MAX_ROWS};

pub type FrameArray = [[char; MAX_COLS]; MAX_ROWS];

pub struct Frame {
    pub tiles: FrameArray,
    points: usize,
}

impl Frame {
    pub fn new() -> Frame {
        let mut tiles = [[' '; MAX_COLS]; MAX_ROWS];

        for row in 0..MAX_ROWS {
            for col in 0..MAX_COLS {
                tiles[row][col] = format!(" ").chars().nth(0).expect("string is empty");
                // Replace this with ' '
            }
        }
        Frame { tiles, points: 0 }
    }

    pub fn tile_content(s: String) -> char {
        format!("{}", s).chars().nth(0).expect("string is empty")
    }

    pub fn set_points(&mut self, points: usize) {
        self.points = points;
    }

    pub fn get_points(&self) -> usize {
        self.points
    }
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
