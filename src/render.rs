use std::io::{Stdout, Write};

use crossterm::{cursor::MoveTo, QueueableCommand, style::{Color, SetBackgroundColor}};

use crate::{frame::Frame, MAX_COLS, MAX_ROWS};

pub fn render(stdout: &mut Stdout, current_frame: &Frame, last_frame: &Frame) {
    stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    for col in 0..MAX_COLS {
        for row in 0..MAX_ROWS {
          if current_frame.tiles[row][col] != last_frame.tiles[row][col] {
            stdout.queue(MoveTo(col as u16, row as u16)).unwrap();
            print!("{}", current_frame.tiles[row][col]);   
          }
        }
    }

    let points_string: String = format!("Puntos: {}", current_frame.get_points());
    stdout.queue(MoveTo(((MAX_COLS -  points_string.len()) / 2) as u16, (MAX_ROWS + 2) as u16)).unwrap();
    print!("{}", points_string);

    stdout.flush().unwrap();
}

pub fn render_background(stdout: &mut Stdout, bakcground: &Frame) {
  for col in 0..MAX_COLS {
    for row in 0..MAX_ROWS {
      stdout.queue(MoveTo(col as u16, row as u16)).unwrap();
      if bakcground.tiles[row][col] == '*' {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        print!(" ");
      }
      else {
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
        print!("{}", bakcground.tiles[row][col]);
      }
    }
  }
}
