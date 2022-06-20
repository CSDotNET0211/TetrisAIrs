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

pub enum MinoKind {
    S,
    Z,
    L,
    J,
    O,
    I,
    T,
    Null,
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
    pub MinoKind: MinoKind,
    pub Rotation: Rotation,
    pub Position: i64,
}

impl Mino {
    pub fn new(MinoKind: MinoKind, Rotation: Rotation, Position: i64) -> Mino {
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

    pub fn MoveForSRS(&mut self, srstest: [[Vector2; 1]; 1], rotate: Rotate, rotation: Rotation) {
        if let rotate = Rotate::Right {
            let value = rotation as usize;

            for i in 0..4 {
                Self::AddPosition(&mut self.Position, srstest[value][i].x.into(), i, true);
                Self::AddPosition(&mut self.Position, srstest[value][i].y.into(), i, false);
            }
        } else {
            for i in 0..4 {}
        }

        fn RoteteEnum(rotate: Rotate, rotation: Rotation, invert: bool) -> Rotation {
            if invert {
                if let mut rotate = Rotate::Left {
                    rotate = Rotate::Right;
                } else {
                    rotate = Rotate::Left;
                }
            }

            let mut rotation = rotation as usize;

            if let rotate = Rotate::Right {
                rotation += 1;

                if rotation == Rotation::Left as usize + 1 {
                    rotation = Rotation::Zero as usize;
                }
            } else {
                rotation -= 1;

                if rotation == Rotation::Zero as usize - 1 {
                    rotation = Rotation::Left as usize;
                }
            }
            Rotation::from_usize(rotation as usize).unwrap()
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
}

struct Environment {}

impl Environment {}
