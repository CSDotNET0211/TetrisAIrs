use core::panic;
use rand::prelude::*;

pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

trait Revert {
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

pub struct Rotation {}

impl Rotation {
    pub const ZERO: i8 = 0;
    pub const RIGHT: i8 = 1;
    pub const TURN: i8 = 2;
    pub const LEFT: i8 = 3;
}

pub struct Action {}

impl Action {
    pub const MOVE_RIGHT: i8 = 0;
    pub const MOVE_LEFT: i8 = 1;
    pub const ROTATE_RIGHT: i8 = 2;
    pub const ROTATE_LEFT: i8 = 3;
    pub const HARD_DROP: i8 = 4;
    pub const SOFT_DROP: i8 = 5;
    pub const HOLD: i8 = 6;
}

pub struct Rotate {}
impl Rotate {
    pub const RIGHT: i8 = 0;
    pub const LEFT: i8 = 1;
}

#[derive(Clone)]
pub struct Mino {
    pub mino_kind: i8,
    pub rotation: i8,
    pub position: i64,
}

impl Mino {
    pub const fn new() -> Mino {
        Mino {
            mino_kind: -1,
            position: -1,
            rotation: Rotation::ZERO,
        }
    }

    pub fn init(&mut self, position: i64) {
        if position == -1 {
            self.position = -1;
        } else {
            self.position = position;
        }

        self.rotation = Rotation::ZERO;
    }

    pub fn move_pos(&mut self, x: i32, y: i32) {
        if x != i32::MAX {
            for i in 0..4 {
                Self::add_position(&mut self.position, x.into(), i, true);
            }
        }

        if y != i32::MAX {
            for i in 0..4 {
                Self::add_position(&mut self.position, y.into(), i, false);
            }
        }
    }

    pub fn move_for_srs(&mut self, srstest: &[[Vector2; 4]; 4], rotate: i8, rotation: i8) {
        if rotate == Rotate::RIGHT {
            let value = rotation;

            for i in 0..4 {
                Self::add_position(
                    &mut self.position,
                    srstest[value as usize][i].x as i64,
                    i as u32,
                    true,
                );
                Self::add_position(
                    &mut self.position,
                    srstest[value as usize][i].y as i64,
                    i as u32,
                    false,
                );
            }
        } else {
            let value = rotate_enum(rotate, rotation) as usize;

            for i in 0..4 {
                Self::add_position(
                    &mut self.position,
                    -srstest[value][i].x as i64,
                    i as u32,
                    true,
                );
                Self::add_position(
                    &mut self.position,
                    -srstest[value][i].y as i64,
                    i as u32,
                    false,
                );
            }
        }

        fn rotate_enum(rotate1: i8, mut rotation: i8) -> i8 {
            if rotate1 == Rotate::RIGHT {
                rotation += 1;

                if rotation == Rotation::LEFT + 1 {
                    rotation = Rotation::ZERO;
                }
            } else {
                rotation -= 1;

                if rotation == Rotation::ZERO - 1 {
                    rotation = Rotation::LEFT;
                }
            }

            rotation
        }
    }

    fn add_position(array: &mut i64, mut value: i64, mut index: u32, is_x: bool) {
        if index == u32::MAX {
            index = 0;
        } else {
            index = 4 - index - 1;
        }

        for _i in 0..4 * index {
            value *= 10;
        }
        if is_x {
            value *= 100;
        }

        *array += value;
    }

    pub fn add_position_xy(array: &mut i64, x: i32, y: i32) {
        let value = y + (x * 100);
        let mut temp = value;

        for i in 1..5 {
            if i != 1 {
                temp *= 10000;
            }

            *array += temp as i64;
        }
    }

    pub fn get_position(&self, mut index: i32, is_x: bool) -> i32 {
        if index == i32::MAX {
            index = 0;
        } else {
            index = 4 - index - 1;
        }

        let mut value = self.position;
        for _i in 0..index {
            value /= 10000;
        }
        value %= 10000;

        if is_x {
            value as i32 / 100
        } else {
            value as i32 % 100
        }
    }

    pub fn get_position_from_value(mut value: i64, mut index: i32, is_x: bool) -> i32 {
        if index == i32::MAX {
            index = 0;
        } else {
            index = 4 - index - 1;
        }

        for i in 0..index {
            value /= 10000;
        }
        value %= 10000;

        if is_x {
            value as i32 / 100
        } else {
            value as i32 % 100
        }
    }
}

pub struct Environment {
    next_bag: Vec<i8>,
    cleared_line: isize,
    score: isize,
    dead_flag: bool,
    pub now_mino: Mino,
    next: [i8; 5],
    random: ThreadRng,
    field: [bool; Self::FIELD_WIDTH as usize * Self::FIELD_HEIGHT as usize],
    can_hold: bool,
    now_hold: i8,
}

impl Environment {
    pub const FIELD_WIDTH: usize = 10;
    pub const FIELD_HEIGHT: usize = 26;

    const JROTATE_TABLE: [[Vector2; 4]; 4] = [
        [Vector2::X2, Vector2::ONE, Vector2::ZERO, Vector2::MONE],
        [
            Vector2::MY2,
            Vector2::new(1, -1),
            Vector2::ZERO,
            Vector2::new(-1, 1),
        ],
        [Vector2::MX2, Vector2::MONE, Vector2::ZERO, Vector2::ONE],
        [
            Vector2::Y2,
            Vector2::new(-1, 1),
            Vector2::ZERO,
            Vector2::new(1, -1),
        ],
    ];

    const LROTATE_TABLE: [[Vector2; 4]; 4] = [
        [Vector2::MY2, Vector2::ONE, Vector2::ZERO, Vector2::MONE],
        [
            Vector2::MX2,
            Vector2::new(1, -1),
            Vector2::ZERO,
            Vector2::new(-1, 1),
        ],
        [Vector2::Y2, Vector2::MONE, Vector2::ZERO, Vector2::ONE],
        [
            Vector2::X2,
            Vector2::new(-1, 1),
            Vector2::ZERO,
            Vector2::new(1, -1),
        ],
    ];

    const SROTATE_TABLE: [[Vector2; 4]; 4] = [
        [
            Vector2::new(1, -1),
            Vector2::MY2,
            Vector2::ONE,
            Vector2::ZERO,
        ],
        [
            Vector2::MONE,
            Vector2::MX2,
            Vector2::new(1, -1),
            Vector2::ZERO,
        ],
        [
            Vector2::new(-1, 1),
            Vector2::Y2,
            Vector2::MONE,
            Vector2::ZERO,
        ],
        [
            Vector2::ONE,
            Vector2::X2,
            Vector2::new(-1, 1),
            Vector2::ZERO,
        ],
    ];

    const ZROTATE_TABLE: [[Vector2; 4]; 4] = [
        [
            Vector2::X2,
            Vector2::new(1, -1),
            Vector2::ZERO,
            Vector2::MONE,
        ],
        [
            Vector2::MY2,
            Vector2::MONE,
            Vector2::ZERO,
            Vector2::new(-1, 1),
        ],
        [
            Vector2::MX2,
            Vector2::new(-1, 1),
            Vector2::ZERO,
            Vector2::ONE,
        ],
        [
            Vector2::Y2,
            Vector2::ONE,
            Vector2::ZERO,
            Vector2::new(1, -1),
        ],
    ];

    const TROTATE_TABLE: [[Vector2; 4]; 4] = [
        [
            Vector2::new(1, -1),
            Vector2::ONE,
            Vector2::ZERO,
            Vector2::MONE,
        ],
        [
            Vector2::MONE,
            Vector2::new(1, -1),
            Vector2::ZERO,
            Vector2::new(-1, 1),
        ],
        [
            Vector2::new(-1, 1),
            Vector2::MONE,
            Vector2::ZERO,
            Vector2::ONE,
        ],
        [
            Vector2::ONE,
            Vector2::new(-1, 1),
            Vector2::ZERO,
            Vector2::new(1, -1),
        ],
    ];

    const IROTATE_TABLE: [[Vector2; 4]; 4] = [
        [
            Vector2::new(2, 1),
            Vector2::X1,
            Vector2::MY1,
            Vector2::new(-1, -2),
        ],
        [
            Vector2::new(1, -2),
            Vector2::MY1,
            Vector2::MX1,
            Vector2::new(-2, 1),
        ],
        [
            Vector2::new(-2, -1),
            Vector2::MX1,
            Vector2::Y1,
            Vector2::new(1, 2),
        ],
        [
            Vector2::new(-1, 2),
            Vector2::Y1,
            Vector2::X1,
            Vector2::new(2, -1),
        ],
    ];

    const KICK_TABLE: [[Vector2; 5]; 4] = [
        [
            Vector2::ZERO,
            Vector2::MX1,
            Vector2::new(-1, 1),
            Vector2::MY2,
            Vector2::new(-1, -2),
        ],
        [
            Vector2::ZERO,
            Vector2::X1,
            Vector2::new(1, -1),
            Vector2::Y2,
            Vector2::new(1, 2),
        ],
        [
            Vector2::ZERO,
            Vector2::X1,
            Vector2::ONE,
            Vector2::MY2,
            Vector2::new(1, -2),
        ],
        [
            Vector2::ZERO,
            Vector2::MX1,
            Vector2::MONE,
            Vector2::Y2,
            Vector2::new(-1, 2),
        ],
    ];

    const IKICK_TABLE: [[Vector2; 5]; 4] = [
        [
            Vector2::ZERO,
            Vector2::MX2,
            Vector2::X1,
            Vector2::new(-2, -1),
            Vector2::new(1, 2),
        ],
        [
            Vector2::ZERO,
            Vector2::MX1,
            Vector2::X2,
            Vector2::new(-1, 2),
            Vector2::new(2, -1),
        ],
        [
            Vector2::ZERO,
            Vector2::X2,
            Vector2::MX1,
            Vector2::new(2, 1),
            Vector2::new(-1, -2),
        ],
        [
            Vector2::ZERO,
            Vector2::X1,
            Vector2::MX2,
            Vector2::new(1, -2),
            Vector2::new(-2, 1),
        ],
    ];

    pub fn create_mino(&mut self, mino: i8) {
        self.now_mino = Mino::new();

        if mino == -1 {
            self.now_mino.mino_kind = self.next[0];
            self.refresh_next();
        } else {
            self.now_mino.mino_kind = mino;
        }
        self.now_mino
            .init(Self::get_default_mino_pos(&self.now_mino.mino_kind));

        for i in 0..4 {
            let x = self.now_mino.get_position(i, true) as usize;
            let y = self.now_mino.get_position(i, false) as usize;

            if self.field[x + y * 10] {
                self.dead_flag = true;
                break;
            }
        }
    }

    pub fn get_field_ref(&self) -> &[bool; Environment::FIELD_HEIGHT * Environment::FIELD_WIDTH] {
        &self.field
    }

    fn get_default_mino_pos(kind: &i8) -> i64 {
        match *kind as i8 {
            MinoKind::I => 0318041805180618,
            MinoKind::J => 0319031804180518,
            MinoKind::L => 0519031804180518,
            MinoKind::O => 0419051904180518,
            MinoKind::S => 0419051903180418,
            MinoKind::Z => 0319041904180518,
            MinoKind::T => 0419031804180518,
            _ => panic!("存在しない型"),
        }
    }

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

    pub fn search(&self) -> i64 {
        0
    }

    pub fn print_game() {}

    pub fn user_input(&mut self, action: i8) {
        let mut srs: Vector2 = Vector2 { x: 0, y: 0 };

        match action {
            Action::MOVE_RIGHT => {
                if Self::check_valid_pos(&self.field, &self.now_mino, &Vector2::X1, 0) {
                    self.now_mino
                        .move_pos(Vector2::X1.x as i32, Vector2::X1.y as i32);
                }
            }

            Action::MOVE_LEFT => {
                if Self::check_valid_pos(&self.field, &self.now_mino, &Vector2::MX1, 0) {
                    self.now_mino
                        .move_pos(Vector2::MX1.x as i32, Vector2::MX1.y as i32);
                }
            }

            Action::ROTATE_RIGHT => {
                if Self::try_rotate(
                    Rotate::RIGHT as i8,
                    &self.field,
                    &mut self.now_mino,
                    &mut srs,
                ) {
                    self.now_mino.move_pos(srs.x as i32, srs.y as i32);
                    Self::simple_rotate(Rotate::RIGHT, &mut self.now_mino, 0);
                }
            }

            Action::ROTATE_LEFT => {
                if Self::try_rotate(
                    Rotate::LEFT as i8,
                    &self.field,
                    &mut self.now_mino,
                    &mut srs,
                ) {
                    self.now_mino.move_pos(srs.x as i32, srs.y as i32);
                    Self::simple_rotate(Rotate::LEFT, &mut self.now_mino, 0);
                }
            }

            Action::HARD_DROP => {
                self.set_mino();
            }
            Action::SOFT_DROP => loop {
                if Self::check_valid_pos(&self.field, &self.now_mino, &Vector2::MY1, 0) {
                    self.now_mino
                        .move_pos(Vector2::MY1.x as i32, Vector2::MY1.y as i32);
                } else {
                    break;
                }
            },
            Action::HOLD => self.hold(),

            _ => panic!("不明な型"),
        }
    }

    fn hold(&mut self) {
        if self.can_hold {
            self.can_hold = false;

            if self.now_hold == -1 {
                self.now_hold = self.now_mino.mino_kind;
                self.create_mino(-1);
            } else {
                let temp_now = self.now_mino.mino_kind;
                self.create_mino(self.now_hold);
                self.now_hold = temp_now;
            }
        }
    }
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
        }
    }

    pub fn init(&mut self) {
        for _i in 0..self.next.len() {
            self.refresh_next();
        }
        self.create_mino(-1);
    }

    fn set_mino(&mut self) {
        loop {
            if Self::check_valid_pos(&self.field, &self.now_mino, &Vector2::MY1, 0) {
                self.now_mino
                    .move_pos(Vector2::MY1.x as i32, Vector2::MY1.y as i32);
            } else {
                break;
            }
        }

        self.can_hold = true;

        for i in 0..4 {
            let x = self.now_mino.get_position(i, true);
            let y = self.now_mino.get_position(i, false);

            self.field[(x + y * 10) as usize] = true;
        }

        self.score += 1;

        let line = Self::check_and_clear_line(&mut self.field);
        self.cleared_line += line as isize;
        match line {
            0 => {}
            1 => self.score += 100,
            2 => self.score += 300,
            3 => self.score += 500,
            4 => self.score += 800,
            _ => panic!("invalid value"),
        }

        self.create_mino(-1);
    }

    pub fn check_valid_pos(
        field: &[bool; Environment::FIELD_HEIGHT * Environment::FIELD_WIDTH],
        mino: &Mino,
        try_move: &Vector2,
        add: i32,
    ) -> bool {
        for i in 0..4 {
            let x = mino.get_position(i, true) + add;
            let y = mino.get_position(i, false) + add;

            if !(x + try_move.x < Environment::FIELD_WIDTH as i32
                && x + try_move.x >= 0
                && y + try_move.y >= 0
                && y + try_move.y < Environment::FIELD_HEIGHT as i32
                && !field[((x + try_move.x) + (y + try_move.y) * 10) as usize])
            {
                return false;
            }
        }
        true
    }

    pub fn check_and_clear_line(
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
                    temp *= 10;
                }

                value_count += 1;
                values += temp;
            }
        }

        Self::down_line(values as i32, value_count, field);

        value_count
    }

    fn down_line(
        mut value: i32,
        mut value_count: i32,
        field: &mut [bool; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT],
    ) {
        if value_count == 0 {
            return;
        }

        let mut index = 0 as i32;

        let yvalue = value % 10;
        value /= 10;
        index += 1;
        value_count -= 1;

        let mut y = yvalue as i32 - 1;
        while y < Environment::FIELD_HEIGHT as i32 {
            y += 1;

            if y < Environment::FIELD_HEIGHT as i32 {
                if value_count > 0 && y + index == value % 10 {
                    index += 1;
                    value /= 10;
                    value_count -= 1;
                    y -= 1;

                    continue;
                }

                for x in 0..Environment::FIELD_WIDTH as i32 {
                    if y + index >= Environment::FIELD_HEIGHT as i32 {
                        field[(x + y * 10) as usize] = false;
                    } else {
                        field[(x + y * 10) as usize] = field[(x + (y + index) * 10) as usize];
                    }
                }
            }
        }
    }

    pub fn get_eval(_values: &[f32]) -> f32 {
        0.0
    }

    pub fn create_mino_1(mino: i8) -> Mino {
        Mino {
            mino_kind: mino,
            rotation: Rotation::ZERO,
            position: Self::get_default_mino_pos(&mino),
        }
    }

    pub fn try_rotate(
        rotate: i8,
        field: &[bool; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT],
        current: &mut Mino,
        srspos: &mut Vector2,
    ) -> bool {
        if current.mino_kind == MinoKind::O {
            return false;
        }

        let before_rotate = current.rotation;
        Self::simple_rotate(rotate, current, 5);

        if rotate == Rotate::LEFT as i8 {
            for i in 0..5 {
                if current.mino_kind == MinoKind::I {
                    if Self::check_valid_pos(
                        &field,
                        &current,
                        &(&Self::IKICK_TABLE[current.rotation as usize][i]).revert(),
                        -5,
                    ) {
                        *srspos = Environment::IKICK_TABLE[current.rotation as usize][i].revert();
                        Self::simple_rotate(Rotate::RIGHT, current, -5);
                        return true;
                    }
                } else {
                    if Self::check_valid_pos(
                        &field,
                        &current,
                        &(&Self::KICK_TABLE[current.rotation as usize][i]).revert(),
                        -5,
                    ) {
                        *srspos = Self::KICK_TABLE[current.rotation as usize][i].revert();
                        Self::simple_rotate(Rotate::RIGHT, current, -5);
                        return true;
                    }
                }
            }

            Self::simple_rotate(Rotate::RIGHT, current, -5);
            return false;
        } else if rotate == Rotate::RIGHT as i8 {
            for i in 0..5 {
                if current.mino_kind == MinoKind::I {
                    if Self::check_valid_pos(
                        &field,
                        &current,
                        &Self::IKICK_TABLE[before_rotate as usize][i],
                        -5,
                    ) {
                        Self::simple_rotate(Rotate::LEFT, current, -5);
                        *srspos = Self::IKICK_TABLE[before_rotate as usize][i].clone();
                        return true;
                    }
                } else {
                    if Self::check_valid_pos(
                        &field,
                        &current,
                        &Self::KICK_TABLE[before_rotate as usize][i],
                        -5,
                    ) {
                        Self::simple_rotate(Rotate::LEFT, current, -5);
                        *srspos = Self::KICK_TABLE[before_rotate as usize][i].clone();
                        return true;
                    }
                }
            }

            Self::simple_rotate(Rotate::LEFT, current, -5);
            return false;
        } else {
            panic!("そんな回転は存在しない");
        }
    }

    pub fn simple_rotate(rotate: i8, mino: &mut Mino, addtemp: i32) {
        let move_pos;
        mino.move_pos(addtemp, addtemp);

        match mino.mino_kind as i8 {
            MinoKind::J => move_pos = Environment::JROTATE_TABLE,
            MinoKind::L => move_pos = Environment::LROTATE_TABLE,
            MinoKind::S => move_pos = Environment::SROTATE_TABLE,
            MinoKind::Z => move_pos = Environment::ZROTATE_TABLE,
            MinoKind::T => move_pos = Environment::TROTATE_TABLE,
            MinoKind::I => move_pos = Environment::IROTATE_TABLE,
            _ => panic!("なにそれ"),
        }

        mino.move_for_srs(&move_pos, rotate, mino.rotation);

        Self::get_next_rotate(rotate, &mut mino.rotation);
    }

    pub fn get_next_rotate(rotate: i8, rotation: &mut i8) {
        if rotate == Rotate::RIGHT {
            *rotation += 1;
            if *rotation == Rotation::LEFT + 1 {
                *rotation = Rotation::ZERO;
            }
        } else {
            *rotation -= 1;
            if *rotation == Rotation::ZERO - 1 {
                *rotation = Rotation::LEFT;
            }
        }
    }
}
