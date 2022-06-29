use crate::environment::*;
use crate::evaluation::*;
use crate::grobaldata::*;
use std::collections::HashSet;
use std::ops::IndexMut;

struct Data {
    current: i8,
    next: i8,
    next_count: i8,
    hold: i8,
    can_hold: bool,
    field: [bool; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT],
    first_move: i64,
    before_eval: f64,
}

impl Data {
    pub const fn new() {}
}

pub struct SearchedPattern {
    pub move_value: i64,
    pub position: i64,
    pub eval: f64,
    pub move_count: i32,
    pub field_index: i32,
}

impl SearchedPattern {
    pub fn new() -> Self {
        SearchedPattern {
            move_value: -1,
            position: -1,
            eval: -1.0,
            move_count: -1,
            field_index: -1,
        }
    }
}

pub struct BeemSearch {}

impl BeemSearch {
    fn get_best_move(
        current: i8,
        nexts: &[i8],
        hold: i8,
        can_hold: bool,
        field: &[bool; Environment::FIELD_HEIGHT * Environment::FIELD_WIDTH],
        next_count: i8,
    ) -> i64 {
        let mut next_int = 0;

        for i in 0..next_count {
            next_int = nexts[i as usize] * (10 * next_count - i - 1);
        }
        eprintln!("next_int={}", next_int);
        let mut data = Data {
            current: current,
            next: next_int,
            next_count: next_count,
            hold: hold,
            can_hold: can_hold,
            field: *field,
            before_eval: 0.0,
            first_move: 0,
        };

        1
    }

    fn search(
        mino: &mut Mino,
        field: &[bool; Environment::FIELD_HEIGHT * Environment::FIELD_WIDTH],
        move_count: i32,
        move_value: i64,
        before_eval: &f64,
        lock_direction: i8,
        rotate_count: i32,
        grobal_data: &mut GrobalData,
        task_index: &usize,
    ) {
        //ハードドロップ
        {
            let mut new_move_diff = Action::HARD_DROP as i32;
            for _i in 0..move_count {
                new_move_diff *= 10;
            }

            let mut newmino = mino.clone();
            let mut temp = 0;

            loop {
                temp += 1;
                if Environment::check_valid_pos(&field, &newmino, &Vector2::new(0, -temp), 0) {
                    temp -= 1;
                    break;
                }
            }

            newmino.move_pos(0, -temp);

            let hash =
                Self::get_hash_for_position(newmino.mino_kind, newmino.rotation, &newmino.position);

            let weight = grobal_data.weight;
            let data = grobal_data.data.index_mut(*task_index);
            //    let searched_data = &mut data.searched_data;
            let result = data.searched_data.get_mut(&hash);

            if result.is_some() {
                let mut result = result.unwrap();

                if result.move_count > move_count {
                    result.move_count = move_count;
                    result.move_value = move_value + new_move_diff as i64;
                }
            } else {
                let mut pattern = SearchedPattern::new();
                pattern.position = newmino.position;
                pattern.move_count = move_count;
                pattern.move_value = move_value + new_move_diff as i64;

                let mut field_clone = field.clone();

                for i in 0..4 {
                    let x = Mino::get_position_from_value(pattern.position, i, true);
                    let y = Mino::get_position_from_value(pattern.position, i, false);

                    field_clone[(x + y * 10) as usize] = true;
                }

                let cleared_line = Environment::check_and_clear_line(&mut field_clone);
                pattern.eval = Evaluation::evaluate(
                    &field_clone,
                    &newmino,
                    cleared_line,
                    data,
                    &weight,
                    task_index,
                );

                data.vec_field.push(field_clone);
                pattern.field_index = data.vec_field.len() as i32 - 1;
                data.searched_data.insert(hash, pattern);
            }
        }

        //左移動
        if lock_direction != Action::MOVE_RIGHT
            && Environment::check_valid_pos(&field, &mino, &Vector2::MX1, 0)
        {
            let mut newmino = mino.clone();

            if !Self::is_passed_before(
                newmino.mino_kind,
                mino.position,
                Vector2::MX1.x,
                Vector2::MX1.y,
                mino.rotation,
                true,
                &mut grobal_data.data[*task_index].passed_tree_route_set,
            ) {
                newmino.move_pos(Vector2::MX1.x, Vector2::MX1.y);
                let mut temp = Action::MOVE_LEFT as i64;
                for _i in 0..move_count {
                    temp *= 10;
                }

                Self::search(
                    &mut newmino,
                    &field,
                    move_count + 1,
                    move_value + temp,
                    &before_eval,
                    Action::MOVE_LEFT,
                    rotate_count,
                    grobal_data,
                    task_index,
                );
            }
        }
        //右移動
        if lock_direction != Action::MOVE_LEFT
            && Environment::check_valid_pos(&field, &mino, &Vector2::X1, 0)
        {
            let mut newmino = mino.clone();

            if !Self::is_passed_before(
                newmino.mino_kind,
                mino.position,
                Vector2::X1.x,
                Vector2::X1.y,
                mino.rotation,
                true,
                &mut grobal_data.data[*task_index].passed_tree_route_set,
            ) {
                newmino.move_pos(Vector2::X1.x, Vector2::X1.y);

                let mut temp = Action::MOVE_RIGHT as i64;
                for _i in 0..move_count {
                    temp *= 10;
                }

                Self::search(
                    &mut newmino,
                    &field,
                    move_count + 1,
                    move_value + temp,
                    &before_eval,
                    Action::MOVE_LEFT,
                    rotate_count,
                    grobal_data,
                    task_index,
                );
            }
        }

        let mut result = Vector2::ZERO;
        //右回転
        if rotate_count < 3 && Environment::try_rotate(Rotate::RIGHT, &field, mino, &mut result) {
            let mut newmino = mino.clone();
            let mut newrotation = newmino.rotation;
            Environment::get_next_rotate(Rotate::RIGHT, &mut newrotation);

            if !Self::is_passed_before(
                newmino.mino_kind,
                newmino.position,
                result.x,
                result.y,
                newrotation,
                true,
                &mut grobal_data.data[*task_index].passed_tree_route_set,
            ) {
                newmino.move_pos(result.x, result.y);
                Environment::simple_rotate(Rotate::RIGHT, &mut newmino, 0);

                let mut temp = Action::ROTATE_RIGHT as i64;
                for _i in 0..move_count {
                    temp *= 10;
                }

                Self::search(
                    &mut newmino,
                    &field,
                    move_count + 1,
                    move_value + temp,
                    &before_eval,
                    lock_direction,
                    rotate_count + 1,
                    grobal_data,
                    task_index,
                );
            }
        }

        //左回転
        if rotate_count < 3 && Environment::try_rotate(Rotate::LEFT, &field, mino, &mut result) {
            let mut newmino = mino.clone();
            let mut newrotation = newmino.rotation;
            Environment::get_next_rotate(Rotate::LEFT, &mut newrotation);

            if !Self::is_passed_before(
                newmino.mino_kind,
                newmino.position,
                result.x,
                result.y,
                newrotation,
                true,
                &mut grobal_data.data[*task_index].passed_tree_route_set,
            ) {
                newmino.move_pos(result.x, result.y);
                Environment::simple_rotate(Rotate::LEFT, &mut newmino, 0);

                let mut temp = Action::ROTATE_LEFT as i64;
                for _i in 0..move_count {
                    temp *= 10;
                }

                Self::search(
                    &mut newmino,
                    &field,
                    move_count + 1,
                    move_value + temp,
                    &before_eval,
                    lock_direction,
                    rotate_count + 1,
                    grobal_data,
                    task_index,
                );
            }
        }
    }

    fn is_passed_before(
        kind: i8,
        mut pos: i64,
        x_diff: i32,
        y_diff: i32,
        newrotation: i8,
        apply_history: bool,
        passed_tree_route_set: &mut HashSet<i64>,
    ) -> bool {
        Mino::add_position_xy(&mut pos, x_diff, y_diff);

        let hash = Self::get_hash_for_position(kind, newrotation, &pos);
        let result = passed_tree_route_set.contains(&hash);
        if result {
            return true;
        }

        if apply_history {
            passed_tree_route_set.insert(hash);
        }

        return false;
    }

    fn get_hash_for_position(kind: i8, rotation: i8, position: &i64) -> i64 {
        if rotation == Rotation::ZERO {
            return *position;
        }

        match kind {
            MinoKind::T => match rotation {
                Rotation::RIGHT => return Self::change_hash_order(position, 1203),
                Rotation::TURN => return Self::change_hash_order(position, 3210),
                Rotation::LEFT => return Self::change_hash_order(position, 3021),
                _ => panic!("a"),
            },
            MinoKind::S => match rotation {
                Rotation::RIGHT | Rotation::LEFT => Self::change_hash_order(position, 2301),
                Rotation::TURN => Self::change_hash_order(position, 3210),
                _ => panic!("a"),
            },
            MinoKind::Z => match rotation {
                Rotation::RIGHT => Self::change_hash_order(position, 0213),
                Rotation::TURN => Self::change_hash_order(position, 3210),
                Rotation::LEFT => Self::change_hash_order(position, 3120),
                _ => panic!("a"),
            },
            MinoKind::L => match rotation {
                Rotation::RIGHT => Self::change_hash_order(position, 1230),
                Rotation::TURN => Self::change_hash_order(position, 3210),
                Rotation::LEFT => Self::change_hash_order(position, 0321),
                _ => panic!("a"),
            },
            MinoKind::J => match rotation {
                Rotation::RIGHT => Self::change_hash_order(position, 1023),
                Rotation::TURN => Self::change_hash_order(position, 3210),
                Rotation::LEFT => Self::change_hash_order(position, 3201),
                _ => panic!("a"),
            },
            MinoKind::I => match rotation {
                Rotation::RIGHT => Self::change_hash_order(position, 0123),
                Rotation::TURN | Rotation::LEFT => Self::change_hash_order(position, 3210),
                _ => panic!("a"),
            },
            _ => panic!("a"),
        }
    }

    fn change_hash_order(hashcode: &i64, order: i32) -> i64 {
        let mut result = 0;
        for i in 0..4 {
            let mut temphash = *hashcode;
            let mut temporder = order;

            for _j in 0..i {
                temphash /= 10000;
                temporder /= 10;
            }

            temphash %= 10000;
            temporder %= 10;

            temporder = 3 - temporder;

            for _j in 0..temporder {
                temphash *= 10000;
            }

            result += temphash;
        }

        result
    }
}
