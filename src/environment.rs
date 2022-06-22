use num;
use num::traits::FromPrimitive;
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Vector2 {
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

pub enum Rotation {
    Zero,
    Right,
    Turn,
    Left,
}
impl FromPrimitive for Rotation {
    fn from_i64(n: i64) -> Option<Rotation> {
        match n {
            0 => Some(Rotation::Zero),
            1 => Some(Rotation::Right),
            2 => Some(Rotation::Turn),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Rotation> {
        match n {
            0 => Some(Rotation::Zero),
            1 => Some(Rotation::Right),
            2 => Some(Rotation::Turn),
            _ => None,
        }
    }
}

pub enum Rotate {
    Right,
    Left,
}
pub struct Mino {
    pub MinoKind: isize,
    pub Rotation: isize,
    pub Position: i64,
}

impl Mino {
    pub const    fn new(MinoKind: isize, Rotation: isize, Position: i64) -> Mino {
        Mino {
            MinoKind: MinoKind,
            Rotation: Rotation,
            Position: Position,
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

_nextBag:Vec;
_clearedLine:i32;
_score:i32;
_deadFlag:bool;
_nowMino:Mino;
_next:[i32:5];
_random:Rng;
_field:[bool;FIELD_WIDTH*FIELD_HEIGHT];
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

    const FIELD_WIDTH:u32=10;
	const FIELD_HEIGHT:u32=26;


}
