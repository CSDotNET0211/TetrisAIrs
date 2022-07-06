use std::{cell::RefCell, io};

use winconsole::console::getch;

use crate::{
    draw,
    environment::{Environment, Mino},
    WEIGHT,
};

pub struct Evaluation {}

thread_local! {
    static ROW_HEIGHT:RefCell<[i32;Environment::FIELD_WIDTH]>={let m=[0;Environment::FIELD_WIDTH]; RefCell::new(m)};
    static HEIGHTS_WITHOUT_IDO:RefCell<Vec<i32>>={let m=Vec::new();RefCell::new(m)};
}

impl Evaluation {
    pub const WEIGHT_COUNT: i32 = 9;

    pub fn evaluate(field: &[bool], mino: &Mino, cleared_line: i32) -> f64 {
        //   HEIGHTS_WITHOUT_IDO.with(|value| value.borrow_mut().clear());
        //ROW_HEIGHT.with(|value|{value.borrow_mut().cle})

        let cleared_value;
        let weight = WEIGHT.get().unwrap();

        match cleared_line {
            0 => cleared_value = 0.0,
            1 => cleared_value = weight[1],
            2 => cleared_value = weight[2],
            3 => cleared_value = weight[3],
            4 => cleared_value = weight[4],
            _ => panic!("0~4ライン消しじゃないよ"),
        }
        let mut rowheight_len = 0;

        let mut smallest_index = -1 as i32;
        let mut smallest = 50 as i32;
        for x in 0..Environment::FIELD_WIDTH {
            let mut flag = true;

            let mut y = Environment::FIELD_HEIGHT as isize - 1;

            ROW_HEIGHT.with(|value| {
                let mut mutvalue = value.borrow_mut();
                rowheight_len = mutvalue.len();

                while y >= 0 {
                    if field[x + y as usize * 10] {
                        if smallest > y as i32 {
                            smallest = y as i32;
                            smallest_index = x as i32;
                        }
                        mutvalue[x] = y as i32 + 1;
                        flag = false;

                        break;
                    }
                    y -= 1;
                }

                if flag {
                    smallest_index = -1;
                    smallest = 50;
                    mutvalue[x] = 0;
                }
            });
        }
        {
            HEIGHTS_WITHOUT_IDO.with(|value| {
                let mut height_without_ido = value.borrow_mut();
                height_without_ido.clear();

                ROW_HEIGHT
                    .with(|rowheight| height_without_ido.extend(rowheight.borrow().iter().clone()));

                if smallest_index != -1 && false {
                    height_without_ido.remove(smallest_index as usize);
                }
            });
        }

        let mut sum_of_height = 0;
        ROW_HEIGHT.with(|value| sum_of_height = value.borrow().iter().sum::<i32>());
        let mut hole_count = 0;

        let mut y = Environment::FIELD_HEIGHT - 1;
        while y >= 1 {
            for x in 0..Environment::FIELD_WIDTH {
                if field[x + y * 10] && !field[x + (y - 1) * 10] {
                    hole_count += 1;
                }
            }

            y -= 1;
        }

        let mut bump = 0;
        HEIGHTS_WITHOUT_IDO.with(|value| {
            let value = value.borrow();

            for i in 0..rowheight_len - 1 - 1 {
                bump += (value[i] - value[i + 1]).abs();
            }
        });
        /*
               let eval = (weight[0] * sum_of_height as f64)
                   + cleared_value
                   + (weight[5] * hole_count as f64)
                   + (weight[6] * bump as f64)
                   + (weight[7] * (hole_count * sum_of_height * sum_of_height) as f64)
                   + (weight[8]
                       * (bump as isize * sum_of_height as isize * sum_of_height as isize) as f64);
        */

        let eval = (sum_of_height as f64 * -0.51)
            + (cleared_line as f64 * 0.76)
            + (hole_count as f64 * -0.3566)
            + (bump as f64 * -0.184);

        /*
        draw::print_debug(field, mino, 0, eval);
        println!(
            "\nでこぼこ:{}\n消去ライン:{}\n穴:{}\n高さ合計:{}\n",
            bump, cleared_line, hole_count, sum_of_height
        );
        let mut temp = String::new();
        getch(true).unwrap();
         */
        /*
        println!("高さ合計:{}\n消去ライン:{}\n穴:{}\nでこぼこ:{}\n穴に高さ合計2乗:{}\nでこぼこに高さ合計2乗:{}",
            (weight[0] * sum_of_height as f64)
                , cleared_value,
                (weight[5] * hole_count as f64)
                ,(weight[6] * bump as f64),
                 (weight[7] * (hole_count * sum_of_height * sum_of_height) as f64)
                , (weight[8]
                    * (bump as isize * sum_of_height as isize * sum_of_height as isize) as f64)
        ); */

        eval
    }
}
