use std::{time::Duration, error::Error, thread::{self, JoinHandle}, sync::mpsc::{Sender, self}, io};

use crossterm::event::{self, Event, KeyCode};
use labyrinth::{frame::{Frame, Drawable}, terminal::{configure_terminal, restore_terminal}, render::{render, render_background}, background::Background, hero::{Hero, self}, chaser::Chaser};

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = configure_terminal();
    let background = Background::new();
    let mut hero = Hero::new();
    let mut chaser = Chaser::new();
    let mut points = 0;

    let (render_tx, _render_handle) = background_render_process();

    let mut initial_frame = Frame::new();
    background.draw(&mut initial_frame);
    render_tx.send((initial_frame, true)).unwrap();

    'mainloop: loop {
        let mut current_frame = Frame::new();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'mainloop;
                    },
                    KeyCode::Up => {
                        hero.move_hero(hero::Direction::Up, &background);
                    },
                    KeyCode::Down => {
                        hero.move_hero(hero::Direction::Down, &background);
                    },
                    KeyCode::Left => {
                        hero.move_hero(hero::Direction::Left, &background);
                    },
                    KeyCode::Right => {
                        hero.move_hero(hero::Direction::Right, &background);
                    },
                    _ => {}
                }
                chaser.move_chaser(&hero, &background);
                points += 1;
            }

        }

        hero.draw(&mut current_frame);
        chaser.draw(&mut current_frame);
        current_frame.set_points(points);

        let _ = render_tx.send((current_frame, false)).unwrap();

        if chaser.hero_reached(&hero) {
            break 'mainloop;
        }
        
        thread::sleep(Duration::from_millis(2));
    }

    drop(render_tx);

    restore_terminal(&mut terminal);

    println!("Has conseguido {points} puntos.");

    Ok(())
}

fn background_render_process() -> (Sender<(Frame, bool)>, JoinHandle<()>) {
    let (render_tx, render_rx) = mpsc::channel();

    let render_handle = thread::spawn(move || {
        let mut last_frame = Frame::new();
        let mut stdout = io::stdout();
        render(&mut stdout, &last_frame, &last_frame);
        loop {
            let (current_frame, draw_background) = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            if draw_background {
                render_background(&mut stdout, &current_frame);
            }
            else {
                render(&mut stdout, &current_frame, &last_frame);
                last_frame = current_frame;
            }
        }
    });

    (render_tx, render_handle)
}