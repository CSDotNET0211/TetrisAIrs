use core::panic;
use std::vec;

use num;
use num::traits::FromPrimitive;
use rand::prelude::*;
pub struct Vector2 {
    pub x: isize,
    pub y: isize,
}

trait Revert {
    fn Revert(&self) -> Self;
}

impl Revert for Vector2 {
    fn Revert(&self) -> Self {
        Vector2 {
            x: -self.x,
            y: -self.y,
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
    pub const MX2: Vector2 = Self::new(-1, 0);
    pub const Y2: Vector2 = Self::new(0, 2);
    pub const MY2: Vector2 = Self::new(0, -2);

    pub const fn new(x: isize, y: isize) -> Vector2 {
        Vector2 { x: x, y: y }
    }
}

struct MinoKind {}
impl MinoKind {
    pub const S: i8 = 0;
    pub const Z: i8 = 1;
    pub const L: i8 = 2;
    pub const J: i8 = 3;
    pub const O: i8 = 4;
    pub const I: i8 = 5;
    pub const T: i8 = 6;
}

struct Rotation {}

impl Rotation {
    pub const Zero: i8 = 0;
    pub const Right: i8 = 1;
    pub const Turn: i8 = 2;
    pub const Left: i8 = 3;
}

struct Action {}

impl Action {
    pub const MoveRight: u32 = 0;
}

pub struct Rotate {}
impl Rotate {
    const Right: i32 = 0;
    const Left: i32 = 1;
}
pub struct Mino {
    pub MinoKind: i32,
    pub Rotation: i32,
    pub Position: i64,
}

impl Mino {
    pub const fn new() -> Mino {
        Mino {
            MinoKind: -1,
            Position: -1,
            Rotation: Rotation::Zero as i32,
        }
    }

    pub fn Init(&mut self, position: i64) {
        if position == -1 {
            self.Position = -1;
        } else {
            self.Position = position;
        }
    }

    pub fn Move(&mut self, x: i32, y: i32) {
        if x != i32::MAX {
            for i in 0..4 {
                Self::AddPosition(&mut self.Position, x.into(), i, true);
            }
        }

        if y != i32::MAX {
            for i in 0..4 {
                Self::AddPosition(&mut self.Position, y.into(), i, false);
            }
        }
    }

    pub fn MoveForSRS(&mut self, srstest: [[Vector2; 4]; 4], rotate: i32, rotation: i32) {
        if let rotate = Rotate::Right {
            let value = rotation as usize;

            for i in 0..4 {
                Self::AddPosition(&mut self.Position, srstest[value][i].x as i64, i, true);
                Self::AddPosition(&mut self.Position, srstest[value][i].y as i64, i, false);
            }
        } else {
            let value = RotateEnum(rotate, rotation, false) as usize;

            for i in 0..4 {
                Self::AddPosition(&mut self.Position, (-srstest[value][i].x as i64), i, true);
                Self::AddPosition(&mut self.Position, (-srstest[value][i].y as i64), i, false);
            }
        }

        fn RotateEnum(mut rotate: i32, mut rotation: i32, invert: bool) -> i32 {
            if invert {
                if rotate == Rotate::Left as i32 {
                    rotate = Rotate::Right as i32;
                } else {
                    rotate = Rotate::Left as i32;
                }
            }

            if let rotate = Rotate::Right {
                rotation += 1;

                if rotation == Rotation::Left as i32 + 1 {
                    rotation = Rotation::Zero as i32;
                }
            } else {
                rotation -= 1;

                if rotation == Rotation::Zero as i32 - 1 {
                    rotation = Rotation::Left as i32;
                }
            }

            rotation
        }
    }

    pub fn AddPosition(array: &mut i64, mut value: i64, mut index: usize, isX: bool) {
        if index == usize::MAX {
            index = 0;
        } else {
            index = 4 - index - 1;
        }

        for i in 0..4 * index {
            value *= 10;
        }
        if isX {
            value *= 100;
        }

        *array += value;
    }

    pub fn AddPositionXY(array: &mut i64, x: i32, y: i32) {
        let value = y + (x * 100);
        let mut temp = value;

        for i in 1..5 {
            if i != 1 {
                temp *= 10000;
            }

            *array += temp as i64;
        }
        //   value += x * 100;
    }

    pub fn GetPosition(&self, mut index: i32, isX: bool) -> i32 {
        if index == i32::MAX {
            index = 0;
        } else {
            index = 4 - index - 1;
        }

        let mut value = self.Position;
        for i in 0..index {
            value /= 10000;
        }
        value %= 10000;

        if isX {
            value as i32 / 100
        } else {
            value as i32 % 100
        }
    }

    pub fn GetPositionFromValue(mut value: i64, mut index: i32, isX: bool) -> i32 {
        if index == i32::MAX {
            index = 0;
        } else {
            index = 4 - index - 1;
        }

        for i in 0..index {
            value /= 10000;
        }
        value %= 10000;

        if isX {
            value as i32 / 100
        } else {
            value as i32 % 100
        }
    }
}

struct Environment {
    _nextBag: Vec<u32>,
    _clearedLine: isize,
    _score: isize,
    _deadFlag: bool,
    _nowMino: Mino,
    _next: [i32; 5],
    _random: ThreadRng,
    _field: [bool; Self::FIELD_WIDTH as usize * Self::FIELD_HEIGHT as usize],
    _canHold: bool,
    _nowHold: i32,
}

impl Environment {
    const BAG_ARRAY: [i8; 7] = [
        MinoKind::S,
        MinoKind::Z,
        MinoKind::L,
        MinoKind::J,
        MinoKind::O,
        MinoKind::I,
        MinoKind::T,
    ];

    pub const FIELD_WIDTH: usize = 10;
    pub const FIELD_HEIGHT: usize = 26;

    const JRotateTable: [[Vector2; 4]; 4] = [
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

    const LRotateTable: [[Vector2; 4]; 4] = [
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

    const SRotateTable: [[Vector2; 4]; 4] = [
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

    const ZRotateTable: [[Vector2; 4]; 4] = [
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

    const TRotateTable: [[Vector2; 4]; 4] = [
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

    const IRotateTable: [[Vector2; 4]; 4] = [
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
            Vector2::new(-2, 2),
            Vector2::Y1,
            Vector2::X1,
            Vector2::new(2, -1),
        ],
    ];

    const KickTable: [[Vector2; 5]; 4] = [
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

    const IKickTable: [[Vector2; 5]; 4] = [
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

    pub fn CreateMino(&mut self, mino: i32) {
        self._nowMino = Mino::new();

        if mino == -1 {
            self._nowMino.MinoKind = self._next[0];
            self.RefreshNext();
        } else {
            self._nowMino.MinoKind = mino;
        }
        self._nowMino
            .Init(Self::GetDefaultMinoPos(&self._nowMino.MinoKind));

        for i in 0..4 {
            let x = self._nowMino.GetPosition(i, true) as usize;
            let y = self._nowMino.GetPosition(i, false) as usize;

            if self._field[x + y * 10] {
                self._deadFlag = true;
                break;
            }
        }
    }

    fn GetDefaultMinoPos(kind: &i32) -> i64 {
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

    fn RefreshNext(&mut self) {
        for i in 0..self._next.len() - 1 {
            self._next[i] = self._next[i + 1];
        }

        if self._nextBag.len() == 0 {
            self._nextBag = (0..7).collect();
        }

        let randomIndex = self._random.gen_range(0..self._nextBag.len());
        let mino = self._nextBag[randomIndex];
        self._nextBag.remove(randomIndex);

        self._next[self._next.len() - 1] = mino as i32;
    }

    pub fn Search() -> i64 {
        0
    }

    pub fn PrintGame() {}

    pub fn UserInput(action: u32) {
        let mut srs: Vector2;

        match action {
            Action::MoveRight => 
{
if Self::CheckValidPos(&self._field,&self._nowMino,Environment::Vector2::X1)
{
self.Move(Environment::Vector2::X1.x,
}
},
            _ => panic!("不明な型"),
        }
    }

    fn Hold(&mut self) {
        if self._canHold {
            self._canHold = false;

            if self._nowHold == -1 {
                self._nowHold = self._nowMino.MinoKind;
                self.CreateMino(-1);
            } else {
                let tempNow = self._nowMino.MinoKind;
                self.CreateMino(self._nowHold);
                self._nowHold = tempNow;
            }
        }
    }
    fn Init(&self) {
        self._nextBag = (0..7).collect();

        for i in 0..self._next.len() {
            self.RefreshNext();
        }

        self._field = [false; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT];
        self.CreateMino(-1);
    }

    fn SetMino(&self) {
        while true {
            if Self::CheckValidPos(&self._field, &self._nowMino, &Vector2::MY1, 0) {
                self._nowMino
                    .Move(Vector2::MY1.x as i32, Vector2::MY1.y as i32);
            } else {
                break;
            }

            self._canHold = true;

            for i in 0..4 {
                let x = self._nowMino.GetPosition(i, true);
                let y = self._nowMino.GetPosition(i, false);

                self._field[(x + y * 10) as usize] = true;
            }

            self._score += 2;

            let line = Self::CheckAndClearLine(&self._field);
self.clearedLine+=line;
match line
{
	1=>self._score+=100,
	2=>self._score+=300,
	3=>self._score+=500,
	4=>self._score+=800,
	_=>panic!("invalid value");
}

self.CreateMino(-1);

        }
    }

    pub fn CheckValidPos(
        field: &[bool; Environment::FIELD_HEIGHT * Environment::FIELD_WIDTH],
        mino: &Mino,
        tryMove: &Vector2,
        add: i32,
    ) -> bool {
        for i in 0..4 {
            let x = (mino.GetPosition(i, true) + add) as isize;
            let y = (mino.GetPosition(i, false) + add) as isize;

            if !(x + tryMove.x < Environment::FIELD_WIDTH as isize
                && x + tryMove.x >= 0
                && y + tryMove.y >= 0
                && y + tryMove.y < Environment::FIELD_HEIGHT as isize
                && !field[((x + tryMove.x) + (y + tryMove.y) * 10) as usize])
            {
                return false;
            }
        }
        true
    }

    pub fn CheckAndClearLine(
        field: &[bool; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT],
    ) -> i32 {
        let mut values = 0;
        let mut valueCount = 0;
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

                for i in 0..valueCount {
                    temp *= 10;
                }

                valueCount += 1;
                values += temp;
            }
        }

        Self::DownLine(values, valueCount, &field);

        valueCount
    }

    fn DownLine(
        value: usize,
        valueCount: i32,
        field: &[bool; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT],
    ) {
        if valueCount == 0 {
            return;
        }

        let mut index = 0;

        let yvalue = value % 10;
        value /= 10;
        index += 1;
        valueCount -= 1;

        for y in yvalue..Environment::FIELD_HEIGHT {
            if valueCount > 0 && y + index == value % 10 {
                index += 1;
                value /= 10;
                valueCount -= 1;
                y -= 1;
                continue;
            }

            for x in 0..Environment::FIELD_WIDTH {
                if y + index >= Environment::FIELD_HEIGHT {
                    field[(x + y * 10) as usize] = false;
                } else {
                    field[(x + y * 10) as usize] = field[(x + (y + index) * 10) as usize];
                }
            }
        }
    }

    pub fn GetEval(values: &[f32]) -> f32 {
        0.0
    }

    pub fn CreateMino1(mino: i32) -> Mino {
        Mino {
            MinoKind: mino,
            Rotation: Rotation::Zero as i32,
            Position: Self::GetDefaultMinoPos(&mino),
        }
    }

    pub fn TryRotate(
        rotate: i8,
        field: &[bool; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT],
        current: &Mino,
        srspos: &Vector2,
    ) -> bool {
        if current.MinoKind == MinoKind::O as i32 {
            return false;
        }

        Self::SimpleRotate(rotate as i32, &current, 5);

        if rotate == Rotate::Left as i8 {
            for i in 0..5 {
                if current.MinoKind == MinoKind::I as i32 {
                    if Self::CheckValidPos(
                        &field,
                        &current,
                        &Self::IKickTable[current.Rotation as usize][i].Revert(),
                        -5,
                    ) {
                        *srspos = Environment::IKickTable[current.Rotation as usize][i].Revert();
                        Self::SimpleRotate(Rotate::Right, &current, -5);
                        return true;
                    }
                } else {
                    if Self::CheckValidPos(
                        &field,
                        &current,
                        &Self::KickTable[current.Rotation as usize][i].Revert(),
                        -5,
                    ) {
                        *srspos = Self::KickTable[current.Rotation as usize][i].Revert();
                        Self::SimpleRotate(Rotate::Right, &current, -5);
                        return true;
                    }
                }
            }

            Self::SimpleRotate(Rotate::Right, &current, -5);
            return false;
        } else if rotate == Rotate::Right as i8 {
            let beforeRotate = current.Rotation;

            for i in 0..5 {
                if current.MinoKind == MinoKind::I as i32 {
                    if Self::CheckValidPos(
                        &field,
                        &current,
                        &Self::IKickTable[beforeRotate as usize][i],
                        -5,
                    ) {
                        Self::SimpleRotate(Rotate::Left, &current, -5);
                        *srspos = Self::IKickTable[beforeRotate as usize][i];
                        return true;
                    }
                } else {
                    if Self::CheckValidPos(
                        &field,
                        &current,
                        &Self::KickTable[beforeRotate as usize][i],
                        -5,
                    ) {
                        Self::SimpleRotate(Rotate::Left, &current, -5);
                        *srspos = Self::KickTable[beforeRotate as usize][i];
                        return true;
                    }
                }
            }

            Self::SimpleRotate(Rotate::Left, &current, -5);
            return false;
        } else {
            panic!("そんな回転は存在しない");
        }
    }

    fn SimpleRotate(rotate: i32, mino: &Mino, addtemp: i32) {
        let mut movePos;
        mino.Move(addtemp, addtemp);

        match mino.MinoKind as i8 {
            MinoKind::J => movePos = Environment::JRotateTable,
            MinoKind::L => movePos = Environment::LRotateTable,
            MinoKind::S => movePos = Environment::SRotateTable,
            MinoKind::Z => movePos = Environment::ZRotateTable,
            MinoKind::T => movePos = Environment::TRotateTable,
            MinoKind::I => movePos = Environment::IRotateTable,
            _ => panic!("なにそれ"),
        }

        mino.MoveForSRS(movePos, rotate, mino.Rotation);

        GetNextRotate(rotate, &mino.Rotation);

        fn GetNextRotate(rotate: i32, rotation: &i32) {
            if rotate == Rotate::Right {
                *rotation += 1;
                if *rotation == Rotation::Left as i32 + 1 {
                    *rotation = Rotation::Zero as i32;
                }
            } else {
                *rotation -= 1;
                if *rotation == Rotation::Zero as i32 - 1 {
                    *rotation = Rotation::Left as i32;
                }
            }
        }
    }
}
