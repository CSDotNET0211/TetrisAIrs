//! テトリスシミュレート環境

use core::panic;
use rand::prelude::*;

use crate::consttable::{
    AttackTable, QUAD_TABLE, TSPIN_DOUBLE_TABLE, TSPIN_MINI_DOUBLE_TABLE, TSPIN_MINI_SINGLE_TABLE,
    TSPIN_SINGLE_TABLE, TSPIN_TRIPLE_TABLE,
};
use crate::mino::Mino;
use crate::{beemsearch::BeemSearch, degit, evaluation::Evaluation, WEIGHT};

pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

pub trait Revert {
    fn revert(&self) -> Self;
    fn clone(&self) -> Self;
}

impl Revert for Vector2 {
    fn revert(&self) -> Self {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }

    fn clone(&self) -> Self {
        Vector2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Vector2 {
    pub const ZERO: Vector2 = Self::new(0, 0);
    pub const X1: Vector2 = Self::new(1, 0);
    pub const MX1: Vector2 = Self::new(-1, 0);
    pub const Y1: Vector2 = Self::new(0, 1);
    pub const MY1: Vector2 = Self::new(0, -1);
    pub const ONE: Vector2 = Self::new(1, 1);
    pub const MONE: Vector2 = Self::new(-1, -1);
    pub const X2: Vector2 = Self::new(2, 0);
    pub const MX2: Vector2 = Self::new(-2, 0);
    pub const Y2: Vector2 = Self::new(0, 2);
    pub const MY2: Vector2 = Self::new(0, -2);

    pub const fn new(x: i32, y: i32) -> Vector2 {
        Vector2 { x: x, y: y }
    }
}

pub struct MinoKind {}
impl MinoKind {
    pub const S: i8 = 0;
    pub const Z: i8 = 1;
    pub const L: i8 = 2;
    pub const J: i8 = 3;
    pub const O: i8 = 4;
    pub const I: i8 = 5;
    pub const T: i8 = 6;
}

pub struct Rotation;

impl Rotation {
    pub const ZERO: i8 = 0;
    pub const RIGHT: i8 = 1;
    pub const TURN: i8 = 2;
    pub const LEFT: i8 = 3;
}

pub struct Action;

impl Action {
    pub const MOVE_RIGHT: i8 = 0;
    pub const MOVE_LEFT: i8 = 1;
    pub const ROTATE_RIGHT: i8 = 2;
    pub const ROTATE_LEFT: i8 = 3;
    pub const HARD_DROP: i8 = 4;
    pub const SOFT_DROP: i8 = 5;
    pub const HOLD: i8 = 6;
    pub const NULL: i8 = 7;
}

pub struct Rotate;
impl Rotate {
    pub const RIGHT: i8 = 0;
    pub const LEFT: i8 = 1;
}

pub struct Environment {
    next_bag: Vec<i8>,
    cleared_line: isize,
    pub score: isize,
    pub dead_flag: bool,
    pub now_mino: Mino,
    next: [i8; 5],
    random: ThreadRng,
    field: [bool; Self::FIELD_WIDTH as usize * Self::FIELD_HEIGHT as usize],
    can_hold: bool,
    now_hold: i8,
    btb_level: u32,
    combo: u32,
    last_move: i8,
    srs_test5: bool,
}

impl Environment {
    pub const FIELD_WIDTH: usize = 10;
    pub const FIELD_HEIGHT: usize = 26;

    ///ミノ情報を作成して環境を更新する
    pub fn create_mino(&mut self, mino: Option<i8>) {
        self.now_mino = Mino::new();

        if mino == Option::None {
            self.now_mino.mino_kind = self.next[0];
            self.refresh_next();
        } else {
            self.now_mino.mino_kind = mino.unwrap();
        }
        self.now_mino
            .init(Self::get_default_mino_pos(&self.now_mino.mino_kind));

        for i in 0..4 {
            let mut x = 0;
            let mut y = 0;
            self.now_mino.get_position(i, &mut x, &mut y);

            if self.field[x as usize + y as usize * 10] {
                self.dead_flag = true;
                self.score -= 1000;
                break;
            }
        }
    }

    ///フィールドの参照取得（フィールドをprivateにしといて
    pub fn get_field_ref(&self) -> &[bool; Environment::FIELD_HEIGHT * Environment::FIELD_WIDTH] {
        &self.field
    }

    ///ミノの出現位置を取得
    fn get_default_mino_pos(kind: &i8) -> i64 {
        match *kind as i8 {
            MinoKind::I => 0318041805180618,
            MinoKind::J => 0319031804180518,
            MinoKind::L => 0519031804180518,
            MinoKind::O => 0419051904180518,
            MinoKind::S => 0419051903180418,
            MinoKind::Z => 0319041904180518,
            MinoKind::T => 0419031804180518,
            _ => panic!("存在しない型:{}", *kind),
        }
    }

    ///ネクストを更新する
    fn refresh_next(&mut self) {
        for i in 0..self.next.len() - 1 {
            self.next[i] = self.next[i + 1];
        }

        if self.next_bag.len() == 0 {
            self.next_bag = (0..7).collect();
        }

        let random_index = self.random.gen_range(0..self.next_bag.len());
        let mino = self.next_bag[random_index];
        self.next_bag.remove(random_index);

        self.next[self.next.len() - 1] = mino;
    }

    ///現在の手で最善なものを選択
    pub fn search(&self) -> i64 {
        BeemSearch::get_best_move(
            self.now_mino.mino_kind,
            &self.next,
            self.now_hold,
            self.can_hold,
            &self.field,
            self.next.len() as i8,
            self.combo.try_into().unwrap(),
        )
    }

    ///操作を入力
    pub fn user_input(&mut self, action: i8) {
        let mut srs: Vector2 = Vector2 { x: 0, y: 0 };

        match action {
            Action::MOVE_RIGHT => {
                if Self::is_valid_pos(&self.field, &self.now_mino, Vector2::X1.x, Vector2::X1.y, 0)
                {
                    self.now_mino
                        .move_pos(Vector2::X1.x as i32, Vector2::X1.y as i32);
                    self.last_move = Action::MOVE_RIGHT;
                }
            }

            Action::MOVE_LEFT => {
                if Self::is_valid_pos(
                    &self.field,
                    &self.now_mino,
                    Vector2::MX1.x,
                    Vector2::MX1.y,
                    0,
                ) {
                    self.now_mino
                        .move_pos(Vector2::MX1.x as i32, Vector2::MX1.y as i32);
                    self.last_move = Action::MOVE_LEFT;
                }
            }

            Action::ROTATE_RIGHT => {
                if self.now_mino.try_rotate(
                    Rotate::RIGHT as i8,
                    &self.field,
                    &mut srs,
                    &mut Some(self.srs_test5),
                ) {
                    self.now_mino.move_pos(srs.x as i32, srs.y as i32);
                    self.now_mino.rotate_force_without_srs(Rotate::RIGHT, 0);
                    self.last_move = Action::ROTATE_RIGHT;
                }
            }

            Action::ROTATE_LEFT => {
                if self.now_mino.try_rotate(
                    Rotate::LEFT as i8,
                    &self.field,
                    &mut srs,
                    &mut Some(self.srs_test5),
                ) {
                    self.now_mino.move_pos(srs.x as i32, srs.y as i32);
                    self.now_mino.rotate_force_without_srs(Rotate::LEFT, 0);
                    self.last_move = Action::ROTATE_LEFT;
                }
            }

            Action::HARD_DROP => {
                self.set_mino();
            }

            Action::SOFT_DROP => loop {
                if Self::is_valid_pos(
                    &self.field,
                    &self.now_mino,
                    Vector2::MY1.x,
                    Vector2::MY1.y,
                    0,
                ) {
                    self.now_mino
                        .move_pos(Vector2::MY1.x as i32, Vector2::MY1.y as i32);
                    self.last_move = Action::SOFT_DROP;
                } else {
                    break;
                }
            },

            Action::HOLD => self.hold(),

            _ => panic!("不明な型"),
        }
    }

    ///ホールド
    fn hold(&mut self) {
        if self.can_hold {
            self.can_hold = false;

            if self.now_hold == -1 {
                self.now_hold = self.now_mino.mino_kind;
                self.create_mino(None);
            } else {
                let temp_now = self.now_mino.mino_kind;
                self.create_mino(Some(self.now_hold));
                self.now_hold = temp_now;
            }
        }
    }

    ///環境を生成
    pub fn new() -> Self {
        Environment {
            next_bag: (0..7).collect(),
            cleared_line: 0,
            score: 0,
            dead_flag: false,
            now_mino: Mino::new(),
            next: [-1; 5],
            random: rand::thread_rng(),
            field: [false; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT],
            can_hold: true,
            now_hold: -1,
            combo: 0,
            btb_level: 0,
            last_move: Action::NULL,
            srs_test5: false,
        }
    }

    ///環境を初期化、new()を併せて使う
    pub fn init(&mut self) {
        for _i in 0..self.next.len() {
            self.refresh_next();
        }
        self.create_mino(None);
    }

    ///ハードドロップ
    fn set_mino(&mut self) {
        loop {
            if Self::is_valid_pos(
                &self.field,
                &self.now_mino,
                Vector2::MY1.x,
                Vector2::MY1.y,
                0,
            ) {
                self.now_mino
                    .move_pos(Vector2::MY1.x as i32, Vector2::MY1.y as i32);
                self.last_move = Action::SOFT_DROP;
            } else {
                break;
            }
        }

        self.can_hold = true;

        for i in 0..4 {
            let mut x = 0;
            let mut y = 0;
            self.now_mino.get_position(i, &mut x, &mut y);

            self.field[(x + y * 10) as usize] = true;
        }

        let line = Self::check_and_clear_line(self);
        self.cleared_line += line as isize;

        self.score += 2;

        self.create_mino(None);
    }

    fn update_score(&mut self, cleared_line: u32) {
        if self.now_mino.mino_kind == MinoKind::T
            && (self.last_move == Action::ROTATE_RIGHT || self.last_move == Action::ROTATE_LEFT)
            && Self::is_tspin_corner(&self.field, self.now_mino.position)
            && !self.srs_test5
        {
            if Self::check_behind_hole_for_tspin_mini(
                &self.field,
                self.now_mino.position,
                &self.now_mino.rotation,
            ) {
                match cleared_line {
                    0 => self.score += 100,
                    1 => self.score += 200,
                    2 => self.score += 400,
                    _ => panic!("3ライン以上のTスピンミニ消しは存在しません。"),
                }
            } else {
                match cleared_line {
                    0 => self.score += 400,
                    1 => self.score += 800,
                    2 => self.score += 1200,
                    3 => self.score += 1600,
                    _ => panic!("4ライン以上のTスピン消しは存在しません。"),
                }
            }
        } else {
            match cleared_line {
                0 => {}
                1 => self.score += 100,
                2 => self.score += 300,
                3 => self.score += 500,
                4 => self.score += 800,
                _ => panic!("5ライン以上の同時消しは存在しません。"),
            }
        }
    }
    ///移動後の位置が適切かどうか
    #[inline(always)]
    pub fn is_valid_pos(
        field: &[bool; Environment::FIELD_HEIGHT * Environment::FIELD_WIDTH],
        mino: &Mino,
        move_x: i32,
        move_y: i32,
        add: i32,
    ) -> bool {
        for i in 0..4 {
            let mut x = 0;
            let mut y = 0;
            mino.get_position(i, &mut x, &mut y);
            x += add;
            y += add;

            if !(x + move_x < Environment::FIELD_WIDTH as i32
                && x + move_x >= 0
                && y + move_y >= 0
                && y + move_y < Environment::FIELD_HEIGHT as i32
                && !field[((x + move_x) + (y + move_y) * 10) as usize])
            {
                return false;
            }
        }
        true
    }

    ///位置が適切かどうか
    #[inline(always)]
    pub fn check_filled_pos(
        field: &[bool; Environment::FIELD_HEIGHT * Environment::FIELD_WIDTH],
        x: i32,
        y: i32,
    ) -> bool {
        if !(x < Environment::FIELD_WIDTH as i32
            && x >= 0
            && y >= 0
            && y < Environment::FIELD_HEIGHT as i32
            && !field[(x + y * 10) as usize])
        {
            return false;
        }

        true
    }

    ///ライン消去できるか判断、出来ればフィールド更新してライン消去数を返す
    #[inline(always)]
    pub fn check_and_clear_line_value(
        field: &mut [bool; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT],
    ) -> i32 {
        let mut values = 0;
        let mut value_count = 0;
        let mut flag;

        for y in 0..Environment::FIELD_HEIGHT {
            flag = true;

            for x in 0..Environment::FIELD_WIDTH {
                if !field[x + y * 10] {
                    flag = false;
                    break;
                }
            }

            if flag {
                let mut temp = y;

                for _i in 0..value_count {
                    temp *= 100;
                }

                value_count += 1;
                values += temp;
            }
        }

        Self::down_line(values as i32, value_count, field);

        value_count
    }

    #[inline(always)]
    fn check_and_clear_line(&mut self) -> i32 {
        let mut values = 0;
        let mut value_count = 0;
        let mut flag;

        for y in 0..Environment::FIELD_HEIGHT {
            flag = true;

            for x in 0..Environment::FIELD_WIDTH {
                if !self.field[x + y * 10] {
                    flag = false;
                    break;
                }
            }

            if flag {
                let mut temp = y;

                for _i in 0..value_count {
                    temp *= 100;
                }

                value_count += 1;
                values += temp;
            }
        }

        Self::update_score(self, value_count as u32);

        Self::down_line(values as i32, value_count, &mut self.field);

        value_count
    }

    #[inline(always)]
    ///フィールドの消去した部分を下げる
    fn down_line(
        mut value: i32,
        mut value_count: i32,
        field: &mut [bool; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT],
    ) {
        if value_count == 0 {
            return;
        }

        let mut skip_count = 0 as i32;

        let mut y_value = value % 100;
        value /= 100;
        skip_count += 1;
        value_count -= 1;

        while y_value < Environment::FIELD_HEIGHT as i32 {
            //y_valueのインデックスが上の消去部だったらスキップして飛ばす数を増やす
            if value_count > 0 && y_value + skip_count == value % 100 {
                skip_count += 1;
                value /= 100;
                value_count -= 1;
            } else {
                //それ以外の場合は下げる
                for x in 0..Environment::FIELD_WIDTH as i32 {
                    if y_value + skip_count >= Environment::FIELD_HEIGHT as i32 {
                        field[(x + y_value * 10) as usize] = false;
                    } else {
                        field[(x + y_value * 10) as usize] =
                            field[(x + (y_value + skip_count) * 10) as usize];
                    }
                }

                y_value += 1;
            }
        }
    }

    #[inline(always)]
    ///ミノを生成する・環境とは独立
    pub fn create_mino_data(mino: i8) -> Mino {
        Mino {
            mino_kind: mino,
            rotation: Rotation::ZERO,
            position: Self::get_default_mino_pos(&mino),
        }
    }

    pub fn get_eval(weight: &[f64; Evaluation::WEIGHT_COUNT as usize]) -> i32 {
        let mut environment = Environment::new();
        environment.init();

        unsafe {
            *WEIGHT = *weight;
        }

        loop {
            let mut result = environment.search();
            for _i in 0..degit(result) {
                environment.user_input((result % 10) as i8);
                result /= 10;
            }

            if environment.cleared_line >= 150 || environment.dead_flag {
                return environment.score as i32;
            }
        }
    }

    #[inline(always)]
    pub fn get_attack_garbage(
        cleared_line: u32,
        is_tspin: bool,
        is_tspin_mini: bool,
        b2b_combo: u32,
        combo: u32,
        is_pc: bool,
    ) -> u32 {
        let b2b_level;
        match b2b_combo {
            0 => b2b_level = 0,
            1..=2 => b2b_level = 1,
            _ => b2b_level = 2,
        }

        match cleared_line {
            0 => 0,
            1 => {
                if is_tspin {
                    TSPIN_SINGLE_TABLE.get().unwrap()[b2b_level][combo as usize]
                } else if is_tspin_mini {
                    TSPIN_MINI_SINGLE_TABLE.get().unwrap()[b2b_level][combo as usize]
                } else {
                    AttackTable::SINGLE[combo as usize]
                }
            }
            2 => {
                if is_tspin {
                    TSPIN_DOUBLE_TABLE.get().unwrap()[b2b_level][combo as usize]
                } else if is_tspin_mini {
                    TSPIN_MINI_DOUBLE_TABLE.get().unwrap()[b2b_level][combo as usize]
                } else {
                    AttackTable::DOUBLE[combo as usize]
                }
            }
            3 => {
                if is_tspin {
                    TSPIN_TRIPLE_TABLE.get().unwrap()[b2b_level][combo as usize]
                } else {
                    AttackTable::TRIPLE[combo as usize]
                }
            }
            4 => QUAD_TABLE.get().unwrap()[b2b_level][combo as usize],
            _ => panic!("なにこれ"),
        }
    }

    ///後ろが埋まってたらtrue
    #[inline(always)]
    pub fn check_behind_hole_for_tspin_mini(
        field: &[bool; Environment::FIELD_HEIGHT * Environment::FIELD_WIDTH],
        t_pos: i64,
        rotation: &i8,
    ) -> bool {
        let mut x = 0;
        let mut y = 0;

        Mino::get_position_from_value(t_pos, 2, &mut x, &mut y);

        match *rotation {
            Rotation::ZERO => Self::check_filled_pos(&field, x, y - 1),
            Rotation::RIGHT => Self::check_filled_pos(&field, x - 1, y),
            Rotation::TURN => Self::check_filled_pos(&field, x, y + 1),
            Rotation::LEFT => Self::check_filled_pos(&field, x + 1, y),
            _ => panic!("わっと"),
        }
    }
    #[inline(always)]
    pub fn is_tspin_corner(
        field: &[bool; Environment::FIELD_HEIGHT * Environment::FIELD_WIDTH],
        t_pos: i64,
    ) -> bool {
        let mut x = 0;
        let mut y = 0;

        Mino::get_position_from_value(t_pos, 2, &mut x, &mut y);

        let mut corner_count = 0;

        if Self::check_filled_pos(&field, x + 1, y + 1) {
            corner_count += 1;
        }

        if Self::check_filled_pos(&field, x + -1, y + 1) {
            corner_count += 1;
        }

        if Self::check_filled_pos(&field, x + 1, y + -1) {
            corner_count += 1;
        }

        if Self::check_filled_pos(&field, x + -1, y + -1) {
            corner_count += 1;
        }

        if corner_count >= 3 {
            return true;
        }

        return false;
    }

    #[inline(always)]
    pub fn is_tspin() -> bool {
        false
    }
}
