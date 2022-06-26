use crate::environment::{self, Environment, Mino};
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

    execute!(
        stdout,
        Hide,
        DisableBlinking,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .unwrap();

    return;
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

    for _i in 0..4 as i32 {
        let x = mino.get_position(_i, true);
        let y = mino.get_position(_i, false);
        queue!(
            stdout,
            cursor::MoveTo(
                x as u16,
                (Environment::FIELD_HEIGHT - 1 - y as usize) as u16,
            )
        )
        .unwrap();

        queue!(
            stdout,
            SetForegroundColor(Color::Blue),
            Print("■"),
            ResetColor
        )
        .unwrap();
    }

    queue!(stdout, cursor::MoveTo(0, 0));
    stdout.flush().unwrap();
}
