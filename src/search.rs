use std::collections::{hash_set, HashSet};
use Environment;

use crate::environment::*;

struct Pattern {
    Move: i64,
    Position: i64,
    Eval: f64,
    MoveCount: i32,
    FieldIndex: i32,
}

struct Data {
    current: i32,
    next: i32,
    next_count: i32,
    hold: i32,
    can_hold: bool,
    field: [bool; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT],
    first_move: i64,
    before_eval: f64,
}

impl Data {
    pub const fn new() {}
}

pub struct Search {}

impl Search {
    fn Search(
        mino: &Mino,
        field: &[bool; Environment::FIELD_HEIGHT * Environment::FIELD_WIDTH],
        move_count: i32,
        move_value: i64,
        before_eval: &f64,
        lock_direction: i8,
        rotate_count: i32,
        passedTreeRouteSet: &mut HashSet<i64>,
    ) {
        //ハードドロップ
        {
            let mut newMoveDiff = Action::HARD_DROP as i32;
            for _i in 0..move_count {
                newMoveDiff *= 10;
            }

            let mut newmino = mino.clone();
            let mut temp = 0;

            loop {
                temp += 1;
                if Environment::check_valid_pos(&field, &newmino, &Vector2::new(0, -temp), 0) {
                    temp -= 1;
                    break;
                }
                newmino.move_pos(0, -temp);

                //  let hash=
            }

            if lock_direction != Action::MOVE_RIGHT
                && Environment::check_valid_pos(&field, &mino, &Vector2::MX1, 0)
            {
                let mut newmino = mino.clone();

                if !Self::IsPassedBefore(
                    newmino.mino_kind,
                    mino.position,
                    Vector2::MX1.x,
                    Vector2::MX1.y,
                    mino.rotation,
                    true,
                    passedTreeRouteSet,
                ) {
                    newmino.move_pos(Vector2::MX1.x, Vector2::MX1.y);
                    let mut temp = Action::MOVE_LEFT as i64;
                    for i in 0..move_count {
                        temp *= 10;
                    }

                    Self::Search(
                        &newmino,
                        &field,
                        move_count,
                        move_value + 1,
                        &before_eval,
                        Action::MOVE_LEFT,
                        rotate_count,
                        passedTreeRouteSet,
                    )
                }

                if lock_direction != Action::MOVE_LEFT
                    && Environment::check_valid_pos(&field, &mino, &Vector2::X1, 0)
                {
                }
            }
        }
    }

    fn IsPassedBefore(
        kind: i8,
        mut pos: i64,
        x: i32,
        y: i32,
        newrotation: i8,
        apply_history: bool,
        passedTreeRouteSet: &mut HashSet<i64>,
    ) -> bool {
        Mino::add_position_xy(&mut pos, x, y);

        let hash = Self::GetHashForPosition(kind, newrotation, &pos);
        let result = passedTreeRouteSet.contains(&hash);
        if result {
            return true;
        }

        if apply_history {
            passedTreeRouteSet.insert(hash);
        }

        return false;
    }

    fn GetHashForPosition(kind: i8, rotation: i8, hash: &i64) -> i64 {
        if rotation == Rotation::ZERO {
            hash;
        }

        match kind {
            MinoKind::T => match rotation {
                Rotation::RIGHT => return Self::ChangeHashOrder(hash, 1203),
                Rotation::TURN => return Self::ChangeHashOrder(hash, 3210),
                Rotation::LEFT => return Self::ChangeHashOrder(hash, 3021),
                _ => panic!("a"),
            },
            MinoKind::S => match rotation {
                Rotation::RIGHT | Rotation::LEFT => Self::ChangeHashOrder(hash, 2301),
                Rotation::TURN => Self::ChangeHashOrder(hash, 3210),
                _ => panic!("a"),
            },
            MinoKind::Z => match rotation {
                Rotation::RIGHT => Self::ChangeHashOrder(hash, 0213),
                Rotation::TURN => Self::ChangeHashOrder(hash, 3210),
                Rotation::LEFT => Self::ChangeHashOrder(hash, 3120),
                _ => panic!("a"),
            },
            MinoKind::L => match rotation {
                Rotation::RIGHT => Self::ChangeHashOrder(hash, 1230),
                Rotation::TURN => Self::ChangeHashOrder(hash, 3210),
                Rotation::LEFT => Self::ChangeHashOrder(hash, 0321),
                _ => panic!("a"),
            },
            MinoKind::J => match rotation {
                Rotation::RIGHT => Self::ChangeHashOrder(hash, 1023),
                Rotation::TURN => Self::ChangeHashOrder(hash, 3210),
                Rotation::LEFT => Self::ChangeHashOrder(hash, 3201),
                _ => panic!("a"),
            },
            MinoKind::I => match rotation {
                Rotation::RIGHT => Self::ChangeHashOrder(hash, 0123),
                Rotation::TURN | Rotation::LEFT => Self::ChangeHashOrder(hash, 3210),
                _ => panic!("a"),
            },
            _ => panic!("a"),
        }
    }

    fn ChangeHashOrder(hashcode: &i64, order: i32) -> i64 {
        let mut result = 0;
        for i in 0..4 {
            let mut temphash = *hashcode;
            let mut temporder = order;

            for j in 0..i {
                temphash /= 10000;
                temporder /= 10;
            }

            temphash %= 10000;
            temporder %= 10;

            temporder = 3 - temporder;

            for j in 0..temporder {
                temphash *= 10000;
            }

            result += temphash;
        }

        result
    }
}
