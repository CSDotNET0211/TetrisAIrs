use crate::environment::{self, Environment, Mino, Vector2};
use crossterm::{
    cursor::{self, DisableBlinking, Hide},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, enable_raw_mode, is_raw_mode_enabled, Clear, ClearType},
};
use std::{
    fmt::write,
    io::{stdout, Write},
};

pub fn print(field: &[bool; Environment::FIELD_HEIGHT * Environment::FIELD_WIDTH], mino: &Mino) {
    enable_raw_mode().unwrap();

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

pub fn print_debug(
    field: &[bool; Environment::FIELD_HEIGHT * Environment::FIELD_WIDTH],
    mino: &Mino,
    move_value: i64,
    eval: f64,
) {
}

fn get_mino_form(minokind: i8) {}
