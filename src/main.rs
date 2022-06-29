mod beemsearch;
mod draw;
mod environment;
mod evaluation;
mod grobaldata;

use draw::print;
use environment::{Action, Environment};
use grobaldata::GrobalData;
use num_cpus;
use std::time;
use winconsole::console::{self, getch};

fn main() {
    let mut environment = Environment::new();
    environment.init();
    let sleeptime = time::Duration::from_millis(30);
    let frame_time = time::Duration::from_millis(1000 / 30);
    let mut GrobalData = GrobalData::new(num_cpus::get() as u32);

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
    }
}
