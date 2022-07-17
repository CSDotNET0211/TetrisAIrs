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
use geneticalgorithm::GeneticAlgorithm;
use num_cpus;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use std::time::{Duration, Instant};
use std::{
    io::{self, Read},
    sync::Mutex,
    thread, time,
};
use threadpool::ThreadPool;

use crate::environment::MinoKind;

pub static mut WEIGHT: Lazy<[f64; Evaluation::WEIGHT_COUNT as usize]> = Lazy::new(|| {
    let m = [0.0; Evaluation::WEIGHT_COUNT as usize];
    m
});
pub static THREAD_POOL: OnceCell<Mutex<ThreadPool>> = OnceCell::new();

//デバッグ用でスレッド数変えてる
fn main() {
    assert!(
        THREAD_POOL
            .set(Mutex::new(ThreadPool::new(num_cpus::get())))
            .is_ok(),
        "スレッドプールの初期化失敗"
    );

    unsafe {
        *WEIGHT = [
            -5628.7173895928445,
            -2450.9359534946434,
            12769.975461312846,
            -22231.72228143262,
            -11297.76613876014,
            4526.680276187824,
            -2796.0433150603876,
            492.1132806920541,
            3502.358375632634,
        ];
    }

    //  GeneticAlgorithm::learn();
    let mut mino = Environment::create_mino_1(MinoKind::I);
    println!(
        "{}:{}",
        BeemSearch::get_hash_for_position(mino.mino_kind, mino.rotation, &mino.position),
        &mino.position
    );
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut environment = Environment::new();
    environment.init();

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

            print(
                &environment.get_field_ref(),
                &environment.now_mino,
                elapsed_time,
            );

            thread::sleep_ms(500);
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
