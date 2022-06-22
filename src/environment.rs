use std::io::IoSlice;

use num;
use num::traits::FromPrimitive;
use rand::prelude::*;
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
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

    pub const fn new(x: i32, y: i32) -> Vector2 {
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

pub enum Rotate {
    Right,
    Left,
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

    pub fn MoveForSRS(&mut self, srstest: [[Vector2; 1]; 1], rotate: i32, rotation: i32) {
        if let rotate = Rotate::Right {
            let value = rotation as usize;

            for i in 0..4 {
                Self::AddPosition(&mut self.Position, srstest[value][i].x.into(), i, true);
                Self::AddPosition(&mut self.Position, srstest[value][i].y.into(), i, false);
            }
        } else {
            let value = RotateEnum(rotate, rotation, false) as usize;

            for i in 0..4 {
                Self::AddPosition(&mut self.Position, (-srstest[value][i].x).into(), i, true);
                Self::AddPosition(&mut self.Position, (-srstest[value][i].y).into(), i, false);
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

    pub const FIELD_WIDTH: u32 = 10;
    pub const FIELD_HEIGHT: u32 = 26;

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
            refresh
        } else {
            self._nowMino.MinoKind = mino;
        }
        self._nowMino
            .Init(Self::GetDefaultMinoPos(self._nowMino.MinoKind as i8));
    }

    fn GetDefaultMinoPos(kind: i8) -> i64 {
        match kind {
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

    fn RefreshNext(nexts: [i32; 5]) {
        for i in 0..nexts.len() - 1 {}
    }
}
