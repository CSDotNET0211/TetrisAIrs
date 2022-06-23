use crate::environment::{self, Environment, Mino, Vector2};
use crossterm_cursor::{cursor, ExecutableCommand};
use std::io::{stdout, BufWriter, Write};

pub fn Print(field: &[bool; Environment::FIELD_HEIGHT * Environment::FIELD_WIDTH], mino: &Mino) {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    let mut cursor = cursor();
    cursor.goto(0, 0);

    let mut y = Environment::FIELD_HEIGHT as i32 - 1;
    while y >= 0 {
        for x in 0..Environment::FIELD_WIDTH {
            if field[x + y as usize * 10] {
                print!("■");
            } else {
                print!("□");
            }
        }
        print!("\n");

        y -= 1;
    }

    for i in 0..4 {
        let x = mino.GetPosition(i, true);
        let y = mino.GetPosition(i, false);

        cursor.goto(
            x as u16,
            (Environment::FIELD_HEIGHT - 1 - y as usize) as u16,
        );
        print!("■");
    }

    cursor.goto(100, 100);
}
