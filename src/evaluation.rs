use winconsole::console::clear;

use crate::{
    environment::{self, Environment, Mino},
    grobaldata::GrobalData,
};

pub struct Evaluation {}

impl Evaluation {
    pub const WEIGHT_COUNT: i32 = 9;

    pub fn Evaluate(
        field: &[bool; Environment::FIELD_HEIGHT * Environment::FIELD_WIDTH],
        mino: &Mino,
        cleared_line: i32,
        grobal_data: &mut GrobalData,
        index: &usize,
    ) -> f64 {
        let mut clearedValue;
        match cleared_line {
            1 => clearedValue = grobal_data.weight[1],
            2 => clearedValue = grobal_data.weight[2],
            3 => clearedValue = grobal_data.weight[3],
            4 => clearedValue = grobal_data.weight[4],
            _ => panic!("1~4ライン消しじゃないよ"),
        }

        let mut smallest_index = -1 as i32;
        let mut smallest = 50 as i32;
        for _x in 0..Environment::FIELD_WIDTH {
            let mut flag = true;

            let mut _y = Environment::FIELD_HEIGHT as isize - 1;
            while _y >= 0 {
                if field[_x + _y as usize * 10] {
                    if smallest > _y as i32 {
                        smallest = _y as i32;
                        smallest_index = _x as i32;
                    }
                    grobal_data.data[*index].row_height[_x] = _y as i32;
                    flag = false;

                    _y -= 1;
                    break;
                }
            }

            if flag {
                smallest_index = -1;
                smallest = 50;
                grobal_data.data[*index].row_height[_x] = -1;
            }
        }
        grobal_data.data[*index].heights_without_ido.clear();
        grobal_data.data[*index]
            .heights_without_ido
            .extend_from_slice(&(grobal_data.data[*index].row_height));
        grobal_data.data[*index]
            .heights_without_ido
            .remove(smallest_index as usize);

        let mut sum_of_height = grobal_data.data[*index].row_height.iter().sum::<i32>();
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
        for i in 0..grobal_data.data[*index].row_height.len() - 1 - 1 {
            bump += (grobal_data.data[*index].heights_without_ido[i]
                - grobal_data.data[*index].heights_without_ido[i + 1])
                .abs();
        }

        ((grobal_data.weight[0] * sum_of_height as f64)
            + clearedValue
            + (grobal_data.weight[5] * hole_count as f64)
            + (grobal_data.weight[6] * bump as f64)
            + (grobal_data.weight[7] * (hole_count * sum_of_height * sum_of_height) as f64)
            + (grobal_data.weight[8]
                * (bump as isize * sum_of_height as isize * sum_of_height as isize) as f64))
    }
}
