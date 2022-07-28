//! ビームサーチ

use crate::environment::*;
use crate::evaluation::*;
use crate::THREAD_POOL;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::sync::Mutex;

///処理データ
struct ProcessData {
    current: i8,
    next: i32,
    next_count: i8,
    hold: i8,
    can_hold: bool,
    field: [bool; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT],
    first_move: i64,
    before_eval: f64,
    combo: i32,
}

impl ProcessData {}

#[derive(PartialOrd, PartialEq, Debug)]
pub enum Tspin {
    No,
    Yes,
    Mini,
}

#[derive(Clone, Copy)]
///検索データ
pub struct SearchedPattern {
    pub move_value: i64,
    pub position: i64,
    pub eval: f64,
    pub move_count: i32,
    pub field_index: i32,
    pub btb: bool,
    pub can_hold: bool,
    pub combo: i32,
}

impl SearchedPattern {
    pub fn new() -> Self {
        SearchedPattern {
            move_value: -1,
            position: -1,
            eval: f64::MIN,
            move_count: -1,
            field_index: -1,
            btb: false,
            can_hold: false,
            combo: -1,
        }
    }
}

///検索：ビームサーチ
pub struct BeemSearch;

thread_local! {
    ///フィールド情報群
    pub static VEC_FIELD:RefCell<Vec<[bool; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT]>>={let  m=Vec::new(); RefCell::new(m)};
    ///検索したデータ
    pub static SEARCHED_DATA:RefCell<HashMap<i64, SearchedPattern>>={let  m=HashMap::new(); RefCell::new(m)};
    //過去に通過した位置を記録
    pub static PASSED_TREE_ROUTE_SET:RefCell<HashMap<i64,u32>>={let  m=HashMap::new();RefCell::new(m)};
    //検索したデータを移行・ソート用
    pub static SEARCHED_DATA_VEC:RefCell<Vec<SearchedPattern>>={let m= Vec::<SearchedPattern>::new();RefCell::new(m)};
}

impl BeemSearch {
    ///最も評価の高い行動を取得
    pub fn get_best_move(
        current: i8,
        nexts: &[i8],
        hold: i8,
        can_hold: bool,
        field: &[bool; Environment::FIELD_HEIGHT * Environment::FIELD_WIDTH],
        next_count: i8,
        combo: i32,
    ) -> i64 {
        let counter = Arc::new(AtomicUsize::new(0));
        let next_count = 4;

        //vec![1,2,3,4] -> 1234
        let mut next_int = 0;
        for i in 0..next_count {
            next_int += {
                let mut temp = nexts[i as usize] as i32;
                for _i in 0..(next_count - i - 1) {
                    temp *= 10;
                }
                temp
            };
        }

        let data = ProcessData {
            current: current,
            next: next_int,
            next_count: next_count,
            hold: hold,
            can_hold: can_hold,
            field: *field,
            before_eval: 0.0,
            first_move: -1,
            combo: combo,
        };

        let queue = Arc::new(Mutex::new(Vec::<ProcessData>::new()));

        {
            queue.lock().unwrap().push(data);
        }

        counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        Self::get_loop(queue, counter)
    }

    #[inline(always)]
    ///スレッド別に処理データを渡して処理する関数
    fn proceed_task(
        processdata: &ProcessData,
        counter: &Arc<AtomicUsize>,
        queue: Arc<Mutex<Vec<ProcessData>>>,
        best: &mut SearchedPattern,
    ) {
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
            false,
            -1,
            &Tspin::No,
            &processdata.combo,
        );

        let mut beem_width = 0;

        SEARCHED_DATA_VEC.with(|searched_data_vec| {
            SEARCHED_DATA.with(|searched_data| {
                searched_data_vec
                    .borrow_mut()
                    .extend(searched_data.borrow().values().into_iter());
            });

            if searched_data_vec.borrow().len() <= 10 {
                beem_width = searched_data_vec.borrow().len();
            } else {
                searched_data_vec
                    .borrow_mut()
                    .sort_by(|a, b| b.eval.partial_cmp(&a.eval).unwrap());
                beem_width = 10;
            }
        });

        if processdata.next_count == 0 {
            //これnewでよくね
            let mut best_in_this_pattern = SearchedPattern {
                position: -1,
                move_value: 0,
                eval: 0.0,
                field_index: 0,
                move_count: 0,
                btb: false,
                can_hold: false,
                combo: -1,
            };

            for beem in 0..beem_width {
                let mut first = -1;

                SEARCHED_DATA_VEC.with(|vec| {
                    if processdata.first_move == -1 {
                        first = vec.borrow()[beem].move_value;
                    } else {
                        first = processdata.first_move;
                    }

                    update_if_better(&mut best_in_this_pattern, &vec.borrow_mut()[beem], first);
                });
            }

            if best.eval < best_in_this_pattern.eval {
                *best = best_in_this_pattern;
            }

        //更新
        } else {
            counter.fetch_add(beem_width, std::sync::atomic::Ordering::SeqCst);

            for beem in 0..beem_width {
                let mut eval = 0.0;
                let mut firstmove = 0;
                SEARCHED_DATA_VEC.with(|vec| {
                    eval = vec.borrow()[beem].eval;
                    firstmove = vec.borrow()[beem].move_value;
                });

                let first: i64;

                if processdata.first_move == -1 {
                    first = firstmove;
                } else {
                    first = processdata.first_move;
                }

                let mut new_current = processdata.next;
                let mut new_next = processdata.next;
                let beforenext = new_next;
                let mut temp_div = 1;

                for _i in 0..processdata.next_count - 1 {
                    new_current /= 10;
                    temp_div *= 10;
                }

                if new_current != 0 {
                    new_next %= temp_div * new_current;
                }

                if cfg!(debug_assertions) {
                    if new_current > 6 {
                        panic!("ミノの値が適切なものではありません");
                    }

                    if new_current != 0 && new_next == processdata.next {
                        panic!("ネクストの値が更新されていません:{}", beforenext);
                    }
                }

                SEARCHED_DATA_VEC.with(|searched_data_vec| {
                    VEC_FIELD.with(|vec_field| {
                        let process_data;

                        process_data = ProcessData {
                            current: new_current as i8,
                            next: new_next,
                            next_count: processdata.next_count - 1,
                            hold: processdata.hold,
                            can_hold: processdata.can_hold,
                            field: vec_field.borrow()
                                [searched_data_vec.borrow()[beem].field_index as usize],
                            before_eval: eval,
                            first_move: first,
                            combo: searched_data_vec.borrow()[beem].combo,
                        };

                        {
                            queue.lock().unwrap().push(process_data);
                        }
                    });
                });
            }
        }

        counter.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);

        #[inline(always)]
        fn init() {
            PASSED_TREE_ROUTE_SET.with(|value| value.borrow_mut().clear());
            VEC_FIELD.with(|value| value.borrow_mut().clear());
            SEARCHED_DATA.with(|value| value.borrow_mut().clear());
            SEARCHED_DATA_VEC.with(|value| value.borrow_mut().clear());
        }

        #[inline(always)]
        fn update_if_better(best: &mut SearchedPattern, test: &SearchedPattern, move_value: i64) {
            if best.position == -1 || test.eval > best.eval {
                *best = *test;
                best.move_value = move_value;
            }
        }
    }

    ///処理データをマルチスレッドに分配して処理する関数
    #[inline(always)]
    fn get_loop(queue: Arc<Mutex<Vec<ProcessData>>>, counter: Arc<AtomicUsize>) -> i64 {
        let thread_pool = THREAD_POOL.get().unwrap().lock().unwrap();
        let best = Arc::new(Mutex::new(SearchedPattern::new()));
        //ここら辺のarc参照にできる？
        loop {
            let data;
            {
                let mut lock = queue.lock().unwrap();
                let len = lock.len();
                //     data = lock.split_off(0);
                if len > 3 {
                    data = lock.split_off(len - 3);
                } else if len != 0 {
                    data = lock.split_off(0);
                } else {
                    //取り出すアイテムが存在しない
                    if counter.load(std::sync::atomic::Ordering::SeqCst) == 0 {
                        return best.lock().unwrap().move_value;
                    } else {
                        continue;
                    }
                }
            }

            let new_counter = Arc::clone(&counter);
            let new_best = Arc::clone(&best);
            let queue_origin = Arc::clone(&queue);

            thread_pool.execute(move || {
                Self::get_loop_multiply(data, new_best, queue_origin, new_counter);
            });
        }
    }
    #[inline(always)]
    fn get_loop_multiply(
        queue: Vec<ProcessData>,
        best: Arc<Mutex<SearchedPattern>>,
        queue_origin: Arc<Mutex<Vec<ProcessData>>>,
        counter: Arc<AtomicUsize>,
    ) {
        counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let mut best_local = SearchedPattern::new();

        for data in &queue {
            Self::proceed_task(data, &counter, Arc::clone(&queue_origin), &mut best_local);
        }
        {
            let mut lock = best.lock().unwrap();
            if best_local.eval > lock.eval {
                lock.move_value = best_local.move_value;
                lock.eval = best_local.eval;
                lock.position = best_local.position;
            }
        }

        counter.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    }

    //再帰で設置パターン列挙
    fn search(
        mino: &mut Mino,
        field: &[bool; Environment::FIELD_HEIGHT * Environment::FIELD_WIDTH],
        move_count: i32,
        move_value: i64,
        before_eval: &f64,
        lock_direction: i8,
        rotate_count: i32,
        move_flag: bool,
        count_after_softdrop: i32,
        tspin: &Tspin,
        combo: &i32,
    ) {
        //ソフドロ落下直後のハードロは禁止してるが、そのあとハードロできる位置に移動して設置も無意味である
        if cfg!(debug_assertions) {
            if move_count > 11 {
                panic!("ループしてない？");
            }
        }

        //もしソフドロカウントがあって出現位置まで上っていけたらハードロだめ

        if count_after_softdrop >= 3 {
            return;
        }

        let mut new_move_diff = 1;

        for _i in 0..move_count {
            new_move_diff *= 10;
        }

        //ハードドロップ・ソフトドロップ
        if count_after_softdrop != 0 {
            let mut newmino = mino.clone();
            let mut temp = 0;

            loop {
                if !Environment::check_valid_pos(&field, &newmino, &Vector2::new(0, -temp), 0) {
                    temp -= 1;
                    break;
                }
                temp += 1;
            }

            newmino.move_pos(0, -temp);

            //ソフトドロップ
            if count_after_softdrop == -1 {
                if temp > 0 {
                    Self::search(
                        &mut newmino,
                        &field,
                        move_count + 1,
                        move_value + new_move_diff * Action::SOFT_DROP as i64,
                        before_eval,
                        lock_direction,
                        rotate_count,
                        move_flag,
                        0,
                        &Tspin::No,
                        &combo,
                    )
                }
            }

            let hash =
                Self::get_hash_for_position(newmino.mino_kind, newmino.rotation, &newmino.position);

            SEARCHED_DATA.with(|value| {
                let mut searched_data = value.borrow_mut();
                if let Some(result) = searched_data.get_mut(&hash) {
                    if result.move_count > move_count {
                        result.move_count = move_count;
                        result.move_value = move_value + new_move_diff * Action::HARD_DROP as i64;
                    }
                } else {
                    let mut pattern = SearchedPattern::new();
                    pattern.position = newmino.position;
                    pattern.move_count = move_count;
                    pattern.move_value = move_value + new_move_diff * Action::HARD_DROP as i64;

                    let mut field_clone = field.clone();

                    for i in 0..4 {
                        let mut x = 0;
                        let mut y = 0;
                        Mino::get_position_from_value(newmino.position, i, &mut x, &mut y);

                        field_clone[(x + y * 10) as usize] = true;
                    }

                    let cleared_line = Environment::check_and_clear_line(&mut field_clone);

                    pattern.eval = Evaluation::evaluate(
                        &field_clone,
                        &newmino,
                        cleared_line,
                        &tspin,
                        false,
                        &(*combo as u32),
                    ) + before_eval;

                    if cleared_line > 0 {
                        pattern.combo = combo + 1;
                    } else {
                        pattern.combo = 0;
                    }

                    if *tspin == Tspin::Mini || *tspin == Tspin::Yes || cleared_line == 4 {
                        pattern.btb = true;
                    }

                    VEC_FIELD.with(|vec_field| {
                        vec_field.borrow_mut().push(field_clone);
                        pattern.field_index = vec_field.borrow().len() as i32 - 1;
                    });

                    searched_data.insert(hash, pattern);
                }
            });
        }

        let softdrop_value;
        if count_after_softdrop == -1 {
            softdrop_value = -1;
        } else {
            softdrop_value = count_after_softdrop + 1;
        }

        //左移動
        if lock_direction != Action::MOVE_RIGHT
            && Environment::check_valid_pos(&field, &mino, &Vector2::MX1, 0)
        {
            let mut newmino = mino.clone();

            if !Self::is_passed_before(
                newmino.mino_kind,
                newmino.position,
                Vector2::MX1.x,
                Vector2::MX1.y,
                mino.rotation,
                move_count as u32,
            ) {
                newmino.move_pos(Vector2::MX1.x, Vector2::MX1.y);

                Self::search(
                    &mut newmino,
                    &field,
                    move_count + 1,
                    move_value + new_move_diff * Action::MOVE_LEFT as i64,
                    &before_eval,
                    Action::MOVE_LEFT,
                    rotate_count,
                    true,
                    softdrop_value,
                    &Tspin::No,
                    &combo,
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
                newmino.position,
                Vector2::X1.x,
                Vector2::X1.y,
                mino.rotation,
                move_count as u32,
            ) {
                newmino.move_pos(Vector2::X1.x, Vector2::X1.y);

                Self::search(
                    &mut newmino,
                    &field,
                    move_count + 1,
                    move_value + new_move_diff * Action::MOVE_RIGHT as i64,
                    &before_eval,
                    Action::MOVE_RIGHT,
                    rotate_count,
                    true,
                    softdrop_value,
                    &Tspin::No,
                    &combo,
                );
            }
        }

        let test5 = false;

        let mut result = Vector2::ZERO;
        //右回転
        if !move_flag
            && rotate_count < 2
            && Environment::try_rotate(Rotate::RIGHT, &field, mino, &mut result, &mut Some(test5))
        {
            let mut newmino = mino.clone();
            let temp_tspin;

            newmino.move_pos(result.x, result.y);
            Environment::simple_rotate(Rotate::RIGHT, &mut newmino, 0);

            if !Self::is_passed_before(
                newmino.mino_kind,
                newmino.position,
                0,
                0,
                newmino.rotation,
                move_count as u32,
            ) {
                if !test5
                    && Environment::is_tspin_corner(&field, mino.position)
                    && newmino.mino_kind == MinoKind::T
                {
                    if Environment::check_behind_hole_for_tspin_mini(
                        &field,
                        mino.position,
                        &mino.rotation,
                    ) {
                        temp_tspin = Tspin::Mini;
                    } else {
                        temp_tspin = Tspin::Yes;
                    }
                } else {
                    temp_tspin = Tspin::No;
                }

                Self::search(
                    &mut newmino,
                    &field,
                    move_count + 1,
                    move_value + new_move_diff * Action::ROTATE_RIGHT as i64,
                    &before_eval,
                    lock_direction,
                    rotate_count + 1,
                    move_flag,
                    softdrop_value,
                    &temp_tspin,
                    &combo,
                );
            }
        }

        //左回転
        if !move_flag
            && rotate_count < 2
            && Environment::try_rotate(Rotate::LEFT, &field, mino, &mut result, &mut Some(test5))
        {
            let mut newmino = mino.clone();
            let temp_tspin;

            newmino.move_pos(result.x, result.y);
            Environment::simple_rotate(Rotate::LEFT, &mut newmino, 0);

            if !Self::is_passed_before(
                newmino.mino_kind,
                newmino.position,
                result.x,
                result.y,
                newmino.rotation,
                move_count as u32,
            ) {
                if !test5
                    && Environment::is_tspin_corner(&field, mino.position)
                    && newmino.mino_kind == MinoKind::T
                {
                    if Environment::check_behind_hole_for_tspin_mini(
                        &field,
                        mino.position,
                        &mino.rotation,
                    ) {
                        temp_tspin = Tspin::Mini;
                    } else {
                        temp_tspin = Tspin::Yes;
                    }
                } else {
                    temp_tspin = Tspin::No;
                }

                Self::search(
                    &mut newmino,
                    &field,
                    move_count + 1,
                    move_value + new_move_diff * Action::ROTATE_LEFT as i64,
                    &before_eval,
                    lock_direction,
                    rotate_count + 1,
                    move_flag,
                    softdrop_value,
                    &temp_tspin,
                    &combo,
                );
            }
        }

        //180回転
    }

    ///過去の位置を記録、参照
    fn is_passed_before(
        kind: i8,
        mut pos: i64,
        x_diff: i32,
        y_diff: i32,
        newrotation: i8,
        move_count: u32,
    ) -> bool {
        Mino::add_position_xy(&mut pos, x_diff, y_diff);

        let hash = Self::get_hash_for_position(kind, newrotation, &pos);
        let mut returnvalue = true;

        PASSED_TREE_ROUTE_SET.with(|value| {
            let mut mut_value = value.borrow_mut();

            if let Some(result) = mut_value.get_mut(&hash) {
                //見つかった場合はmoveカウントで判断
                if *result > move_count {
                    *result = move_count;
                    returnvalue = false;
                }
            } else {
                returnvalue = false;

                mut_value.insert(hash, move_count);
            }
        });

        returnvalue
    }

    #[inline(always)]
    ///位置情報を回転情報関係なく均一にする
    pub fn get_hash_for_position(kind: i8, rotation: i8, position: &i64) -> i64 {
        if rotation == Rotation::ZERO {
            return *position;
        }

        match kind {
            MinoKind::T => match rotation {
                Rotation::RIGHT => return Self::change_hash_order(position, 2013),
                Rotation::TURN => return Self::change_hash_order(position, 3210),
                Rotation::LEFT => return Self::change_hash_order(position, 1320),
                _ => panic!("a"),
            },
            MinoKind::S => match rotation {
                Rotation::RIGHT => Self::change_hash_order(position, 2301),
                Rotation::TURN => Self::change_hash_order(position, 3210),
                Rotation::LEFT => Self::change_hash_order(position, 1032),
                _ => panic!("a"),
            },
            MinoKind::Z => match rotation {
                Rotation::RIGHT => Self::change_hash_order(position, 0213),
                Rotation::TURN => Self::change_hash_order(position, 3210),
                Rotation::LEFT => Self::change_hash_order(position, 3120),
                _ => panic!("a"),
            },
            MinoKind::L => match rotation {
                Rotation::RIGHT => Self::change_hash_order(position, 3012),
                Rotation::TURN => Self::change_hash_order(position, 3210),
                Rotation::LEFT => Self::change_hash_order(position, 0321),
                _ => panic!("a"),
            },
            MinoKind::J => match rotation {
                Rotation::RIGHT => Self::change_hash_order(position, 1023),
                Rotation::TURN => Self::change_hash_order(position, 3210),
                Rotation::LEFT => Self::change_hash_order(position, 2310),
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

    ///位置情報を指定された順番に書き換える
    pub fn change_hash_order(hashcode: &i64, mut order: i32) -> i64 {
        let mut result = 0;

        for i in 0..4 {
            let mut temphash = *hashcode;

            for _i in 0..i {
                temphash /= 10000;
            }
            temphash %= 10000;

            for _i in 0..3 - (order % 10) {
                temphash *= 10000;
            }

            result += temphash;
            order /= 10;
        }

        result
    }
}
