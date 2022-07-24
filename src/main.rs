mod beemsearch;
mod consttable;
mod draw;
mod environment;
mod evaluation;
mod threadpool;

use consttable::TSPIN_DOUBLE_TABLE;
use consttable::TSPIN_MINI_DOUBLE_TABLE;
use consttable::TSPIN_MINI_SINGLE_TABLE;
use consttable::TSPIN_SINGLE_TABLE;
use consttable::TSPIN_TRIPLE_TABLE;
use draw::print;
use environment::Environment;
use evaluation::Evaluation;
use num_cpus;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use std::time::Instant;
use std::{io, sync::Mutex};
use threadpool::ThreadPool;

use crate::consttable::{AttackTable, QUAD_TABLE};

pub static mut WEIGHT: Lazy<[f64; Evaluation::WEIGHT_COUNT as usize]> = Lazy::new(|| {
    let m = [0.0; Evaluation::WEIGHT_COUNT as usize];
    m
});
pub static THREAD_POOL: OnceCell<Mutex<ThreadPool>> = OnceCell::new();
/*
#[link(name = "TestDllForRust", kind = "static")]
extern "C" {
    fn Test1();
    fn Test2(value: i32) -> i32;
    fn Test3(value: &mut i32);
    fn Test5(value: &&mut i8) -> bool;
    fn Test6(value: fn(i32));
    fn Test7(value: &Struct7) -> i32;
}
struct Struct7 {
    x: i32,
    y: i32,
}
struct MinoState {
    mino_type: u8,
    x: i32,
    y: i32,
    rotation: i32,
    tspin: i32,
}*/

//デバッグ用でスレッド数変えてる
fn main() {
    assert!(
        THREAD_POOL
            .set(Mutex::new(ThreadPool::new(num_cpus::get())))
            .is_ok(),
        "スレッドプールの初期化失敗"
    );

    init_values();

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
    fn test(value: i32) {
        println!("{}が入力されたよ！", value);
    }

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut environment = Environment::new();
    environment.init();

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

            /*     print(
                &environment.get_field_ref(),
                &environment.now_mino,
                elapsed_time,
            ); */

            // thread::sleep_ms(500);
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

fn init_values() {
    QUAD_TABLE
        .set([
            &AttackTable::QUAD,
            &AttackTable::B2B1QUAD,
            &AttackTable::B2B2QUAD,
        ])
        .unwrap();

    TSPIN_MINI_SINGLE_TABLE
        .set([
            &AttackTable::TSPINMINISINGLE,
            &AttackTable::B2B1TSPINMINISINGLE,
            &AttackTable::B2B2TSPINMINISINGLE,
        ])
        .unwrap();

    TSPIN_SINGLE_TABLE
        .set([
            &AttackTable::TSPINSINGLE,
            &AttackTable::B2B1TSPINSINGLE,
            &AttackTable::B2B2TSPINSINGLE,
        ])
        .unwrap();

    TSPIN_MINI_DOUBLE_TABLE
        .set([
            &AttackTable::TSPINMINIDOUBLE,
            &AttackTable::B2B1TSPINMINIDOUBLE,
            &AttackTable::B2B2TSPINMINIDOUBLE,
        ])
        .unwrap();

    TSPIN_DOUBLE_TABLE
        .set([
            &AttackTable::TSPINDOUBLE,
            &AttackTable::B2B1TSPINDOUBLE,
            &AttackTable::B2B2TSPINDOUBLE,
        ])
        .unwrap();

    TSPIN_TRIPLE_TABLE
        .set([
            &AttackTable::TSPINTRIPLE,
            &AttackTable::B2B1TSPINTRIPLE,
            &AttackTable::B2B2TSPINTRIPLE,
        ])
        .unwrap();
}
