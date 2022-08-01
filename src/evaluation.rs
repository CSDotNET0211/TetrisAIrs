//! 盤面評価

use core::panic;
use std::cell::RefCell;

use crate::{beemsearch::Tspin, consttable::*, environment::Environment, mino::Mino, WEIGHT};

pub struct Evaluation;

thread_local! {
    static ROW_HEIGHT:RefCell<[i32;Environment::FIELD_WIDTH]>={let m=[0;Environment::FIELD_WIDTH]; RefCell::new(m)};
    static HEIGHTS_WITHOUT_IDO:RefCell<Vec<i32>>={let m=Vec::new();RefCell::new(m)};
}

impl Evaluation {
    pub const WEIGHT_COUNT: i32 = 7 + 4;

    pub fn evaluate(
        field: &[bool],
        mino: &Mino,
        cleared_line: i32,
        tspin: &Tspin,
        btb: bool,
        combo: &u32,
    ) -> f64 {
        //b2b実装
        let b2b_level = 0;

        let mut bump = 0;
        let mut sum_of_height = 0;

        let mut attack_eval = 0.0;
        let mut defence_eval = 0.0;

        let weight;

        unsafe {
            weight = *WEIGHT;
        }

        let mut power = 0;
        //火力計算

        if cleared_line != 0 {
            if *tspin == Tspin::Yes {
                match cleared_line {
                    1 => power = TSPIN_SINGLE_TABLE.get().unwrap()[b2b_level][*combo as usize],
                    2 => power = TSPIN_DOUBLE_TABLE.get().unwrap()[b2b_level][*combo as usize],
                    3 => power = TSPIN_TRIPLE_TABLE.get().unwrap()[b2b_level][*combo as usize],
                    _ => panic!("?"),
                }
            } else if *tspin == Tspin::Mini {
                match cleared_line {
                    1 => power = TSPIN_MINI_SINGLE_TABLE.get().unwrap()[b2b_level][*combo as usize],
                    2 => power = TSPIN_MINI_DOUBLE_TABLE.get().unwrap()[b2b_level][*combo as usize],
                    _ => panic!("?"),
                }
            } else {
                match cleared_line {
                    1 => power = AttackTable::SINGLE[*combo as usize],
                    2 => power = AttackTable::DOUBLE[*combo as usize],
                    3 => power = AttackTable::TRIPLE[*combo as usize],
                    4 => power = AttackTable::QUAD[*combo as usize],
                    _ => panic!("{}", cleared_line),
                }
            }

            match cleared_line {
                1 => defence_eval += weight[7],
                2 => defence_eval += weight[8],
                3 => defence_eval += weight[9],
                4 => defence_eval += weight[10],
                _ => panic!("aaaa"),
            }
        }

        if btb {
            attack_eval += weight[0];
        }

        attack_eval += power as f64 * weight[1];

        let mut smallest_index = -1 as i32;
        let mut smallest = 50 as i32;

        ROW_HEIGHT.with(|value| {
            let mut row_height = value.borrow_mut();

            for x in 0..Environment::FIELD_WIDTH {
                let mut smallest_found_flag = true;

                let mut y = Environment::FIELD_HEIGHT as isize - 1;

                while y >= 0 {
                    if field[x + y as usize * 10] {
                        if smallest > y as i32 {
                            smallest = y as i32;
                            smallest_index = x as i32;
                        }
                        row_height[x] = y as i32 + 1;
                        smallest_found_flag = false;

                        break;
                    }
                    y -= 1;
                }

                if smallest_found_flag {
                    smallest_index = -1;
                    smallest = 50;
                    row_height[x] = 0;
                }
            }

            // □□■
            // □□□■
            //■■□■■

            HEIGHTS_WITHOUT_IDO.with(|value| {
                let mut height_without_ido = value.borrow_mut();
                height_without_ido.clear();

                height_without_ido.extend(row_height.iter().clone());

                if smallest_index != -1 {
                    height_without_ido.remove(smallest_index as usize);
                }

                for i in 0..Environment::FIELD_WIDTH - 1 - 1 {
                    bump += (height_without_ido[i] - height_without_ido[i + 1]).abs();
                }
            });

            sum_of_height = row_height.iter().sum::<i32>();
        });

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

        defence_eval += hole_count as f64 * weight[2];
        defence_eval += hole_count as f64 * hole_count as f64 * weight[3];
        defence_eval += bump as f64 * weight[4];
        defence_eval += bump as f64 * bump as f64 * weight[5];
        defence_eval += sum_of_height as f64 * weight[6];

        attack_eval + defence_eval
    }
}
