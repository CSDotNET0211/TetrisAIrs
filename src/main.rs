mod draw;
mod environment;
use ::console::Key;
use draw::print;
use environment::{Action, Environment};
use std::{thread, time};
use winconsole::{
    console::{self, getch},
    input::{self, KeyCode},
};

fn main() {
    let mut environment = Environment::new();
    environment.init();
    let sleeptime = time::Duration::from_millis(30);
    let frame_time = time::Duration::from_millis(1000 / 30);
    console::clear().unwrap();
    loop {
        print(&environment.get_field_ref(), &environment.now_mino);

        let mut pressed;
        loop {
            thread::sleep(frame_time);
            pressed = input::get_pressed_keys().unwrap();

            if pressed.len() != 0 {
                break;
            } else {
                thread::sleep(sleeptime);
            }
        }

        match pressed[0] {
            KeyCode::Right => environment.user_input(Action::MOVE_RIGHT),
            KeyCode::Left => environment.user_input(Action::MOVE_LEFT),
            KeyCode::Up => environment.user_input(Action::HARD_DROP),
            KeyCode::Down => environment.user_input(Action::SOFT_DROP),
            KeyCode::Control => environment.user_input(Action::HOLD),
            KeyCode::X => environment.user_input(Action::ROTATE_RIGHT),
            KeyCode::Z => environment.user_input(Action::ROTATE_LEFT),
            _ => panic!("a"),
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
