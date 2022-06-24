mod Search;
mod draw;
mod environment;
mod search;

use draw::print;
use environment::{Action, Environment};
use std::time;
use winconsole::console::{self, getch};

fn main() {
    let mut environment = Environment::new();
    environment.init();
    let sleeptime = time::Duration::from_millis(30);
    let frame_time = time::Duration::from_millis(1000 / 30);

    console::clear().unwrap();
    loop {
        print(&environment.get_field_ref(), &environment.now_mino);
        let key = getch(true).unwrap();

        match key {
            '\\' => environment.user_input(Action::MOVE_RIGHT),
            '.' => environment.user_input(Action::MOVE_LEFT),
            ';' => environment.user_input(Action::HARD_DROP),
            '/' => environment.user_input(Action::SOFT_DROP),
            'c' => environment.user_input(Action::HOLD),
            'x' => environment.user_input(Action::ROTATE_RIGHT),
            'z' => environment.user_input(Action::ROTATE_LEFT),
            'r' => {
                environment = Environment::new();
                environment.init();
            }

            _ => continue,
        }
        //   thread::sleep(sleeptime);
        /*
        if input::is_key_down(input::KeyCode::Right) {
            environment.user_input(Action::MOVE_RIGHT);
        } else if input::is_key_down(input::KeyCode::Left) {
            environment.user_input(Action::MOVE_LEFT);
        } else if input::is_key_down(input::KeyCode::Up) {
            environment.user_input(Action::HARD_DROP);
        } else if input::is_key_down(input::KeyCode::Down) {
            environment.user_input(Action::SOFT_DROP);
        } else if input::is_key_down(input::KeyCode::X) {
            environment.user_input(Action::ROTATE_RIGHT);
        } else if input::is_key_down(input::KeyCode::Z) {
            environment.user_input(Action::ROTATE_LEFT);
        } */

        //    println!(value);
    }
}
