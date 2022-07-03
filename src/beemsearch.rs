use crate::environment::*;
use crate::evaluation::*;
use crate::grobaldata::*;
use crate::threadpool::ThreadPool;
use std::cell::RefCell;
use std::collections::hash_set;
use std::collections::HashMap;
use std::collections::HashSet;
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
    pub const fn new() -> Self {
        ProcessData {
            current: -1,
            next: -1,
            next_count: -1,
            hold: -1,
            can_hold: false,
            field: [false; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT],
            first_move: -1,
            before_eval: -1.0,
        }
    }

    pub fn clone(&self) -> Self {
        ProcessData {
            current: self.current,
            next: self.next,
            next_count: self.next_count,
            hold: self.hold,
            can_hold: self.can_hold,
            field: self.field,
            first_move: self.first_move,
            before_eval: self.before_eval,
        }
    }
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
            eval: f64::MIN,
            move_count: -1,
            field_index: -1,
        }
    }
}

pub struct BeemSearch;

thread_local! {
 pub   static vec_field:RefCell<Vec<[bool; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT]>>={let mut m=Vec::new(); RefCell::new(m)};
 pub static searched_data:RefCell<HashMap<i64, SearchedPattern>>={let mut m=HashMap::new(); RefCell::new(m)};
pub static passed_tree_route_set:RefCell<HashSet<i64>>={let mut m=HashSet::new();RefCell::new(m)};
}

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
        processdata: &ProcessData,
        counter: Arc<AtomicUsize>,
        queue: Arc<Mutex<Vec<ProcessData>>>,
        best: Arc<Mutex<SearchedPattern>>,
    ) {
        //     searched_data::with(|value| {});
        init();

        let mut mino = Environment::create_mino_1(processdata.current);

        Self::search(
            &mut mino,
            &processdata.field,
            0,
            0,
            &processdata.before_eval,
            Action::NULL,
            0,
        );
        let mut searched_data_vec = Vec::<SearchedPattern>::new();

        searched_data.with(|value| searched_data_vec.extend(value.borrow().values().into_iter()));

        let beem_width;

        if searched_data_vec.len() <= 10 {
            beem_width = searched_data_vec.len();
        } else {
            searched_data_vec.sort_by(|a, b| b.eval.partial_cmp(&a.eval).unwrap());
            beem_width = 10;
        }

        if processdata.next_count == 0 {
            let mut best_in_this_pattern = SearchedPattern {
                position: -1,
                move_value: 0,
                eval: 0.0,
                field_index: 0,
                move_count: 0,
            };

            for beem in 0..beem_width {
                let first: i64;

                if processdata.first_move == -1 {
                    first = searched_data_vec[beem].move_value;
                } else {
                    first = processdata.first_move;
                }

                update_ifbetter(&mut best_in_this_pattern, &searched_data_vec[beem], first);
            }

            {
                let mut value = best.lock().unwrap();
                if value.eval < best_in_this_pattern.eval {
                    *value = best_in_this_pattern;
                }
            }
        //更新
        } else {
            for beem in 0..beem_width {
                searched_data_vec[beem].eval += processdata.before_eval;

                let first: i64;

                if processdata.first_move == -1 {
                    first = searched_data_vec[beem].move_value;
                } else {
                    first = processdata.first_move;
                }

                let mut new_current = processdata.next;
                let mut new_next = processdata.next;
                let mut temp_div = 10;

                for _i in 0..processdata.next_count - 1 {
                    new_current /= 10;
                    temp_div *= 10;
                }

                new_next %= temp_div;

                counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

                let mut process_data = ProcessData::new();
                vec_field.with(|value| {
                    process_data = ProcessData {
                        current: new_current,
                        next: new_next,
                        next_count: processdata.next_count - 1,
                        hold: processdata.hold,
                        can_hold: processdata.can_hold,
                        field: value.borrow()[beem],
                        before_eval: searched_data_vec[beem].eval,
                        first_move: first,
                    }
                });

                {
                    queue.lock().unwrap().push(process_data);
                }
            }

            {
                counter.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
            }
        }

        fn init() {
            passed_tree_route_set.with(|value| value.borrow_mut().clear());
            vec_field.with(|value| value.borrow_mut().clear());
            searched_data.with(|value| value.borrow_mut().clear());
        }

        fn update_ifbetter(best: &mut SearchedPattern, test: &SearchedPattern, move_value: i64) {
            if best.position == -1 || test.eval > best.eval {
                *best = *test;
                best.move_value = move_value;
            }
        }
    }

    fn get_loop(grobal_data: &'static mut GrobalData) {
        let counter = Arc::new(AtomicUsize::new(0));
        let queue = Arc::new(Mutex::new(Vec::<ProcessData>::new()));
        let hashmap = HashMap::<i32, i64>::new();
        let thread_pool = ThreadPool::new(num_cpus::get());
        let best = Arc::new(Mutex::new(SearchedPattern::new()));

        loop {
            let data = queue.lock().unwrap().pop();
            let counter = Arc::clone(&counter);
            let best = Arc::clone(&best);
            let queue = Arc::clone(&queue);

            match data {
                Some(result) => {
                    thread_pool.execute(move || {
                        Self::proceed_task(&result, counter, queue, Arc::clone(&best));
                    });
                }
                None => {}
            }
        }
    }

    fn search(
        mino: &mut Mino,
        field: &[bool; Environment::FIELD_HEIGHT * Environment::FIELD_WIDTH],
        move_count: i32,
        move_value: i64,
        before_eval: &f64,
        lock_direction: i8,
        rotate_count: i32,
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

            searched_data.with(|value| {
                let mut mutvalue = value.borrow_mut();
                if let Some(result) = mutvalue.get_mut(&hash) {
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
                    pattern.eval = Evaluation::evaluate(&field_clone, &newmino, cleared_line);

                    vec_field.with(|value| {
                        value.borrow_mut().push(field_clone);
                        pattern.field_index = mutvalue.len() as i32 - 1;
                    });

                    mutvalue.insert(hash, pattern);
                }
            });
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
    ) -> bool {
        Mino::add_position_xy(&mut pos, x_diff, y_diff);

        let hash = Self::get_hash_for_position(kind, newrotation, &pos);
        let mut result = false;
        passed_tree_route_set.with(|value| {
            result = value.borrow().contains(&hash);

            if !result && apply_history {
                value.borrow_mut().insert(hash);
            }
        });

        if result {
            return true;
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
