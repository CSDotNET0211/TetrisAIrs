mod draw;
mod environment;
use console::{Key, Term};
use draw::Print;
use environment::{Action, Environment};
use std::thread;
use std::time::Duration;

fn main() {
    let mut environment = Environment::new();
    environment.Init();

    loop {
        //     let mut value =
        /*
        if Keycode::Right.is_pressed() {
            environment.UserInput(Action::MoveRight);
        } else if Keycode::Left.is_pressed() {
            environment.UserInput(Action::MoveLeft);
        } else if Keycode::Up.is_pressed() {
            environment.UserInput(Action::HardDrop);
        } else if Keycode::Down.is_pressed() {
            environment.UserInput(Action::SoftDrop);
        } else if Keycode::X.is_pressed() {
            environment.UserInput(Action::RotateRight);
        } else if Keycode::Z.is_pressed() {
            environment.UserInput(Action::RotateLeft);
        }*/

        Print(&environment.GetFieldRef(), &environment.nowMino);
        let term = Term::stdout();
        let key = match term.read_key() {
            Ok(key) => key,
            Err(e) => panic!("{}", e),
        };
        if key == Key::Char(97 as char) {
            environment.UserInput(Action::MoveRight);
        }

        //    println!(value);
    }
}
