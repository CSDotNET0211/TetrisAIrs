use crate::environment::{Environment, Mino};
use std::io::{stdout, BufWriter};
extern crate winconsole;
use winconsole::console;

pub fn print(field: &[bool; Environment::FIELD_HEIGHT * Environment::FIELD_WIDTH], mino: &Mino) {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    let mut newfield = field.clone();

    for i in 0..4 {
        let x = mino.get_position(i, true);
        let y = mino.get_position(i, false);

        newfield[(x + y * 10) as usize] = true;
    }

    console::set_cursor_position(0, 0).unwrap();
    let mut y = Environment::FIELD_HEIGHT as i32 - 1;
    while y >= 0 {
        for x in 0..Environment::FIELD_WIDTH {
            if newfield[x + y as usize * 10] {
                print!("■");
            } else {
                print!("□");
            }
        }
        print!("\r\n");

        y -= 1;
    }

    // console::set_cursor_position(100, 100).unwrap();
}
