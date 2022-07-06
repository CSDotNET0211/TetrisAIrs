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
use std::{
    io::{self, Read},
    sync::Mutex,
    thread, time,
};

use thread_id;
use threadpool::ThreadPool;
use winconsole::console::{self, getch};

pub static WEIGHT: OnceCell<[f64; Evaluation::WEIGHT_COUNT as usize]> = OnceCell::new();
pub static THREAD_POOL: OnceCell<Mutex<ThreadPool>> = OnceCell::new();

fn main() {
    assert!(
        THREAD_POOL
            .set(Mutex::new(ThreadPool::new(num_cpus::get())))
            .is_ok(),
        "スレッドプールの初期化失敗"
    );

    assert!(
        WEIGHT
            .set([
                200.1597, 319.1632, -1149.735, 118.6968, 187.1296, -604.2106, -551.1594, -364.9467,
                -43.58047,
            ])
            .is_ok(),
        "err"
    );

    let mut environment = Environment::new();
    environment.init();

    // environment.now_mino.mino_kind = 4;
    //  environment.next = [4, 4, 4, 4, 4];

    println!("何かキーを入力して検索を開始");
    //   let key = getch(true).unwrap();
    let mut buf = String::new();
    //    io::stdin().read_line(&mut buf).unwrap();

    let sleeptime = time::Duration::from_millis(30);
    let frame_time = time::Duration::from_millis(1000 / 30);
    let mut GrobalData = GrobalData::new(num_cpus::get() as u32);

    // console::clear().unwrap();
    loop {
        print(&environment.get_field_ref(), &environment.now_mino);

        //   getch(true).unwrap();
        thread::sleep_ms(500);
        //io::stdin().read_line(&mut buf).unwrap();

        let mut result = environment.search();
        //println!("{}", result);
        let count = degit(result);

        for _i in 0..count {
            environment.user_input((result % 10).try_into().unwrap());
            result /= 10;
        }
        /*
        let key = getch(false).unwrap();

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
        } */
    }
}

fn degit(num: i64) -> i32 {
    if num == 0 {
        return 1;
    } else {
        let num = num as f64;
        return libm::log10(num) as i32 + 1;
    }
}
