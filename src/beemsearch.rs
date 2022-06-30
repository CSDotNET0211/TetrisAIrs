use crossterm::queue;

use crate::environment::*;
use crate::evaluation::*;
use crate::grobaldata::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::IndexMut;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::sync::Mutex;
struct ProcessData {
    current: i8,
    next: i8,
    next_count: i8,
    hold: i8,
    can_hold: bool,
    field: [bool; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT],
    first_move: i64,
    before_eval: f64,
}

impl ProcessData {
    pub const fn new() {}
}

#[derive(Clone, Copy)]
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
        let mut data = ProcessData {
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

    fn proceed_task(
        data: ProcessData,
        grobal_data: &mut GrobalData,
        index: usize,
        counter: Arc<AtomicUsize>,
        queue: Arc<Mutex<Vec<ProcessData>>>,
        best: Arc<Mutex<SearchedPattern>>,
    ) {
        let counter_clone = Arc::clone(&counter);
        let queue_clone = Arc::clone(&queue);
        let best_clone = Arc::clone(&best);

        init(grobal_data, index);

        let mut mino = Environment::create_mino_1(data.current);
        Self::search(
            &mut mino,
            &data.field,
            0,
            0,
            &data.before_eval,
            Action::NULL,
            0,
            grobal_data,
            &index,
        );
        let mut searched_data_vec = Vec::<SearchedPattern>::new();
        searched_data_vec.extend(grobal_data.data[index].searched_data.values().into_iter());
        //  grobal_data.data[index].searched_data.values().collect();

        let beem_width;

        if searched_data_vec.len() <= 10 {
            beem_width = searched_data_vec.len();
        } else {
            searched_data_vec.sort_by(|a, b| b.eval.partial_cmp(&a.eval).unwrap());
            beem_width = 10;
        }

        if data.next_count == 0 {
            let mut best_in_this_pattern = SearchedPattern {
                position: -1,
                move_value: 0,
                eval: 0.0,
                field_index: 0,
                move_count: 0,
            };

            for beem in 0..beem_width {
                let first: i64;

                if data.first_move == -1 {
                    first = searched_data_vec[beem].move_value;
                } else {
                    first = data.first_move;
                }

                update_ifbetter(&mut best_in_this_pattern, &searched_data_vec[beem], first);
            }

            {
                update_ifbetter(
                    best_clone.lock().unwrap(),
                    &best_in_this_pattern,
                    best_in_this_pattern.move_value,
                );
            }
        //更新
        } else {
            for beem in 0..beem_width {
                searched_data_vec[beem].eval += data.before_eval;

                let first: i64;

                if data.first_move == -1 {
                    first = searched_data_vec[beem].move_value;
                } else {
                    first = data.first_move;
                }

                let mut newcurrent = data.next;
                let mut newnext = data.next;
                let mut tempDiv = 10;

                for _i in 0..data.next_count - 1 {
                    newcurrent /= 10;
                    tempDiv *= 10;
                }

                newnext %= tempDiv;
            }
        }

        fn init(grobal_data: &mut GrobalData, index: usize) {
            let mut data = grobal_data.data.index_mut(index as usize);
            data.passed_tree_route_set.clear();
            data.vec_field.clear();
            data.searched_data.clear();
        }

        fn update_ifbetter(best: &mut SearchedPattern, test: &SearchedPattern, move_value: i64) {
            if best.position == -1 || test.eval > best.eval {
                *best = *test;
                best.move_value = move_value;
            }
        }
    }

    fn get_loop() {
        let counter = Arc::new(AtomicUsize::new(0));
        let queue = Arc::new(Mutex::new(Vec::<ProcessData>::new()));
        //      counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let mut hashmap = HashMap::<i32, i64>::new();
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

            match data.searched_data.get_mut(&hash) {
                Some(result) => {
                    if result.move_count > move_count {
                        result.move_count = move_count;
                        result.move_value = move_value + new_move_diff as i64;
                    }
                }
                None => {
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
