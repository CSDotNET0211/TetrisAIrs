use std::{cell::RefCell, ops::IndexMut};

use crate::{
    environment::{Environment, Mino},
    grobaldata::{Data, GrobalData},
    WEIGHT,
};

pub struct Evaluation {}

thread_local! {
    static row_height:RefCell<[i32;Environment::FIELD_WIDTH]>={let mut m=[0;Environment::FIELD_WIDTH]; RefCell::new(m)};
    static heights_without_ido:RefCell<Vec<i32>>={let mut m=Vec::new();RefCell::new(m)};
}

impl Evaluation {
    pub const WEIGHT_COUNT: i32 = 9;

    pub fn evaluate(field: &[bool], mino: &Mino, cleared_line: i32) -> f64 {
        let cleared_value;
        let weight = WEIGHT.get().unwrap();

        match cleared_line {
            1 => cleared_value = weight[1],
            2 => cleared_value = weight[2],
            3 => cleared_value = weight[3],
            4 => cleared_value = weight[4],
            _ => panic!("1~4ライン消しじゃないよ"),
        }
        let mut rowheight_len = 0;

        let mut smallest_index = -1 as i32;
        let mut smallest = 50 as i32;
        for _x in 0..Environment::FIELD_WIDTH {
            let mut flag = true;

            let mut _y = Environment::FIELD_HEIGHT as isize - 1;

            row_height.with(|value| {
                let mut mutvalue = value.borrow_mut();
                rowheight_len = mutvalue.len();

                while _y >= 0 {
                    if field[_x + _y as usize * 10] {
                        if smallest > _y as i32 {
                            smallest = _y as i32;
                            smallest_index = _x as i32;
                        }
                        mutvalue[_x] = _y as i32;
                        flag = false;

                        _y -= 1;
                        break;
                    }
                }

                if flag {
                    smallest_index = -1;
                    smallest = 50;
                    mutvalue[_x] = -1;
                }
            });
        }
        {
            heights_without_ido.with(|value| {
                let mut mutvalue = value.borrow_mut();
                mutvalue.clear();

                row_height.with(|rowheight| mutvalue.extend(rowheight.borrow().iter().clone()));

                mutvalue.remove(smallest_index as usize);
            });
        }

        let mut sum_of_height = 0;
        row_height.with(|value| sum_of_height = value.borrow().iter().sum::<i32>());
        let mut hole_count = 0;

        let mut y = Environment::FIELD_HEIGHT - 1;
        while y >= 1 {
            for x in 0..Environment::FIELD_WIDTH {
                if field[x + y * 10] && field[x + (y - 1) * 10] {
                    hole_count += 1;
                }
            }

            y -= 1;
        }

        let mut bump = 0;
        heights_without_ido.with(|value| {
            let mutvalue = value.borrow_mut();

            for i in 0..rowheight_len - 1 - 1 {
                bump += (mutvalue[i] - mutvalue[i + 1]).abs();
            }
        });

        (weight[0] * sum_of_height as f64)
            + cleared_value
            + (weight[5] * hole_count as f64)
            + (weight[6] * bump as f64)
            + (weight[7] * (hole_count * sum_of_height * sum_of_height) as f64)
            + (weight[8] * (bump as isize * sum_of_height as isize * sum_of_height as isize) as f64)
    }
}
