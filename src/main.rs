mod beemsearch;
mod draw;
mod environment;
mod evaluation;
mod grobaldata;
mod threadpool;

use draw::print;
use environment::{Action, Environment};
use evaluation::Evaluation;
use grobaldata::GrobalData;
use num_cpus;
use once_cell::sync::OnceCell;
use std::{thread, time};
use thread_id;
use threadpool::ThreadPool;
use winconsole::console::{self, getch};

fn test(index: usize) {
    thread::sleep_ms(1500);
    println!("{}番目のやつ終わり\r\nidは", index);
}

pub static WEIGHT: OnceCell<[f64; Evaluation::WEIGHT_COUNT as usize]> = OnceCell::new();

fn main() {
    let pool = ThreadPool::new(2);
    for i in 0..10 {
        pool.execute(move || {
            test(i);
        });
    }

    getch(true).unwrap();

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
