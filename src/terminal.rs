use std::{io::{self, Stdout}};

use crossterm::{terminal::{self, LeaveAlternateScreen, EnterAlternateScreen}, cursor::{Show, Hide}, ExecutableCommand};

pub fn configure_terminal() -> Stdout {
  let mut stdout = io::stdout();
  match terminal::enable_raw_mode() {
      Err(_e) => print!("Fallo"),
      _ => ()
  }
  match stdout.execute(EnterAlternateScreen) {
      Err(_e) => print!("Fallo"),
      _ => ()
  }
  match stdout.execute(Hide) {
      Err(_e) => print!("Fallo"),
      _ => ()
  }

  stdout
}

pub fn restore_terminal(stdout: &mut Stdout) {
  match stdout.execute(Show) {
      Err(_e) => print!("Fallo"),
      _ => ()
  }

  match stdout.execute(LeaveAlternateScreen) {
      Err(_e) => print!("Fallo"),
      _ => ()
  }

  match terminal::disable_raw_mode() {
      Err(_e) => print!("Fallo"),
      _ => ()
  }
}