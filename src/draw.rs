//! コンソール出力用

use crate::environment::{Environment, Mino, Vector2};
use crossterm::{
    cursor::{self, DisableBlinking, Hide},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io::{stdout, Write};

pub fn print(
    field: &[bool; Environment::FIELD_HEIGHT * Environment::FIELD_WIDTH],
    mino: &Mino,
    time: u128,
) {
    //    enable_raw_mode().unwrap();

    let mut stdout = stdout();

    execute!(
        stdout,
        Hide,
        DisableBlinking,
        cursor::MoveTo(0, 0),
        //Clear()
    )
    .unwrap();
    queue!(stdout, cursor::MoveTo(0, 0)).unwrap();

    let mut y = Environment::FIELD_HEIGHT as i32 - 1;
    while y >= 0 {
        for x in 0..Environment::FIELD_WIDTH {
            if field[x + y as usize * 10] {
                queue!(stdout, Print("■")).unwrap();
            } else {
                queue!(stdout, Print("□")).unwrap();
            }
        }
        queue!(stdout, Print("\r\n")).unwrap();

        y -= 1;
    }

    queue!(stdout, Print(time)).unwrap();

    let mut quickdrop_value = 0;

    loop {
        if !Environment::check_valid_pos(
            &field,
            &mino,
            &Vector2 {
                x: 0,
                y: quickdrop_value,
            },
            0,
        ) {
            quickdrop_value += 1;
            break;
        }

        quickdrop_value -= 1;
    }

    for _i in 0..4 as i32 {
        let x = mino.get_position(_i, true);
        let y = mino.get_position(_i, false) + quickdrop_value;
        queue!(
            stdout,
            cursor::MoveTo(
                x as u16 * 2,
                (Environment::FIELD_HEIGHT - 1 - y as usize) as u16,
            )
        )
        .unwrap();

        queue!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print("■"),
            ResetColor
        )
        .unwrap();
    }

    for _i in 0..4 as i32 {
        let x = mino.get_position(_i, true);
        let y = mino.get_position(_i, false);
        queue!(
            stdout,
            cursor::MoveTo(
                x as u16 * 2,
                (Environment::FIELD_HEIGHT - 1 - y as usize) as u16,
            )
        )
        .unwrap();

        queue!(
            stdout,
            SetForegroundColor(Color::DarkBlue),
            Print("■"),
            ResetColor
        )
        .unwrap();
    }

    stdout.flush().unwrap();
}

#[allow(dead_code)]
pub fn print_debug(field: &[bool], _mino: &Mino, _move_valuee: i64, eval: f64) {
    let mut stdout = stdout();

    execute!(stdout, Hide, DisableBlinking, cursor::MoveTo(0, 0)).unwrap();
    queue!(stdout, cursor::MoveTo(0, 0)).unwrap();

    let mut y = Environment::FIELD_HEIGHT as i32 - 1;
    while y >= 0 {
        for x in 0..Environment::FIELD_WIDTH {
            if field[x + y as usize * 10] {
                queue!(stdout, Print("■")).unwrap();
            } else {
                queue!(stdout, Print("□")).unwrap();
            }
        }
        queue!(stdout, Print("\r\n")).unwrap();

        y -= 1;
    }

    queue!(stdout, Print("\r\n")).unwrap();
    queue!(stdout, Print("eval = ")).unwrap();
    queue!(stdout, Print(eval)).unwrap();
    /*
    for _i in 0..4 as i32 {
        let x = mino.get_position(_i, true);
        let y = mino.get_position(_i, false);
        queue!(
            stdout,
            cursor::MoveTo(
                x as u16 * 2,
                (Environment::FIELD_HEIGHT - 1 - y as usize) as u16,
            )
        )
        .unwrap();

        queue!(stdout, Print("■"));
    } */
}
