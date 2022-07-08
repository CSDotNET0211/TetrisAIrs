mod beemsearch;
mod draw;
mod environment;
mod evaluation;
mod geneticalgorithm;
mod threadpool;

use beemsearch::BeemSearch;
use draw::print;
use environment::{Action, Environment};
use evaluation::Evaluation;
use num_cpus;
use once_cell::sync::OnceCell;
use std::time::{Duration, Instant};
use std::{
    io::{self, Read},
    sync::Mutex,
    thread, time,
};
use threadpool::ThreadPool;

use crate::environment::MinoKind;
pub static WEIGHT: OnceCell<[f64; Evaluation::WEIGHT_COUNT as usize]> = OnceCell::new();
pub static THREAD_POOL: OnceCell<Mutex<ThreadPool>> = OnceCell::new();

//デバッグ用でスレッド数変えてる
fn main() {
    assert!(
        THREAD_POOL
            .set(Mutex::new(ThreadPool::new(num_cpus::get() + 10)))
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

    println!("何かキーを入力して検索を開始");
    let mut buf = String::new();

    let mut timer;
    let mut elapsed_time = 0;
    loop {
        print(
            &environment.get_field_ref(),
            &environment.now_mino,
            elapsed_time,
        );

        timer = Instant::now();

        let mut result = environment.search();

        //    getch(false).unwrap();

        elapsed_time += timer.elapsed().as_millis();
        if elapsed_time != 0 {
            elapsed_time /= 2;
        }

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
