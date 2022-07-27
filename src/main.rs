mod beemsearch;
mod consttable;
mod draw;
mod environment;
mod evaluation;
mod geneticalgorithm;
mod threadpool;
use consttable::*;
use core::panic;
use core::time;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use draw::print;
use environment::Environment;
use evaluation::Evaluation;
use num_cpus;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use std::io::stdout;
use std::io::Read;
use std::thread;
use std::time::Instant;
use std::{io, sync::Mutex};
use threadpool::ThreadPool;

use crate::consttable::{AttackTable, QUAD_TABLE};
use crate::environment::Action;
use crate::geneticalgorithm::GeneticAlgorithm;

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
struct MinoState2 {
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

    //  GeneticAlgorithm::learn();

    unsafe {
        *WEIGHT = [
            -5104.508150699315,
            21812.731734085493,
            -56578.2761999506,
            -27001.756285058043,
            31255.35315222146,
            -2569.0062025903776,
            535.7389604004768,
        ];
    }

    println!("モードを選択してください。");
    println!("1.学習");
    println!("2.手動操作");
    println!("3.AI操作");

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    match buf.trim() {
        "1" => GeneticAlgorithm::learn(),
        "2" => {
            enable_raw_mode().unwrap();
            let mut stdout = stdout();
            let mut environment = Environment::new();
            environment.init();

            loop {
                print(&environment.get_field_ref(), &environment.now_mino, 0);

                match crossterm::event::read().unwrap() {
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::NONE,
                    }) => environment.user_input(Action::HOLD),
                    Event::Key(KeyEvent {
                        code: KeyCode::Right,
                        modifiers: KeyModifiers::NONE,
                    }) => environment.user_input(Action::MOVE_RIGHT),
                    Event::Key(KeyEvent {
                        code: KeyCode::Left,
                        modifiers: KeyModifiers::NONE,
                    }) => environment.user_input(Action::MOVE_LEFT),
                    Event::Key(KeyEvent {
                        code: KeyCode::Up,
                        modifiers: KeyModifiers::NONE,
                    }) => environment.user_input(Action::HARD_DROP),
                    Event::Key(KeyEvent {
                        code: KeyCode::Down,
                        modifiers: KeyModifiers::NONE,
                    }) => environment.user_input(Action::SOFT_DROP),
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('x'),
                        modifiers: KeyModifiers::NONE,
                    }) => environment.user_input(Action::ROTATE_RIGHT),
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('z'),
                        modifiers: KeyModifiers::NONE,
                    }) => environment.user_input(Action::ROTATE_LEFT),
                    _ => (),
                }
            }
        }
        "3" => {
            let mut environment = Environment::new();
            environment.init();

            let mut timer;
            let mut elapsed_time = 0;
            loop {
                timer = Instant::now();

                let mut result = environment.search();

                //    getch(false).unwrap();

                elapsed_time += timer.elapsed().as_millis();
                if elapsed_time != 0 {
                    elapsed_time /= 2;
                }

                let count = degit(result);

                for _i in 0..count {
                    print(
                        &environment.get_field_ref(),
                        &environment.now_mino,
                        elapsed_time,
                    );

                    environment.user_input((result % 10).try_into().unwrap());
                    result /= 10;
                    thread::sleep_ms(100);
                }

                if environment.dead_flag {
                    draw::dead();
                    break;
                }
            }
        }
        _ => panic!("違法な入力"),
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
