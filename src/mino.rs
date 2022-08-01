use crate::{consttable::*, environment::*};

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

    ///位置を移動
    #[inline(always)]
    pub fn move_pos(&mut self, x: i32, y: i32) {
        for i in 0..4 {
            Self::add_position(&mut self.position, x, y, i);
        }
    }

    ///SRS専用の位置移動
    fn move_for_srs(&mut self, srstest: &[[Vector2; 4]; 4], rotate: i8, rotation: i8) {
        if rotate == Rotate::RIGHT {
            let value = rotation;

            for i in 0..4 {
                Self::add_position(
                    &mut self.position,
                    srstest[value as usize][i].x,
                    srstest[value as usize][i].y,
                    i,
                );
            }
        } else {
            let value = rotate_enum(rotate, rotation) as usize;

            for i in 0..4 {
                Self::add_position(
                    &mut self.position,
                    -srstest[value][i].x,
                    -srstest[value][i].y,
                    i,
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

    ///x,yそれぞれの位置を追加
    #[inline(always)]
    pub fn add_position(position: &mut i64, x: i32, y: i32, mut index: usize) {
        index = 4 - index - 1;

        let mut add_value = y as i64;
        add_value += x as i64 * 100;

        for _i in 0..index {
            add_value *= 10000;
        }

        *position += add_value;
    }

    ///xまたはyの位置を取得
    #[inline(always)]
    pub fn get_position(&self, mut index: i32, x: &mut i32, y: &mut i32) {
        index = 4 - index - 1;

        let mut value = self.position;
        for _i in 0..index {
            value /= 10000;
        }
        value %= 10000;

        *x = value as i32 / 100;
        *y = value as i32 % 100;
    }

    ///引数の値から位置を取得
    #[inline(always)]
    pub fn get_position_from_value(mut value: i64, mut index: i32, x: &mut i32, y: &mut i32) {
        index = 4 - index - 1;

        for _i in 0..index {
            value /= 10000;
        }
        value %= 10000;

        *x = value as i32 / 100;
        *y = value as i32 % 100;
    }

    ///回転を試みる　ミノ情報は変更しないが一時的に変える都合上可変参照
    #[inline(always)]
    pub fn try_rotate(
        &mut self,
        rotate: i8,
        field: &[bool; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT],
        srspos: &mut Vector2,
        test5: &mut Option<bool>,
    ) -> bool {
        if self.mino_kind == MinoKind::O {
            return false;
        }
        *test5 = Some(false);

        let before_rotate = self.rotation;
        Self::rotate_force_without_srs(self, rotate, 5);

        if rotate == Rotate::LEFT as i8 {
            for i in 0..5 {
                if i == 4 && *test5 != Option::None && self.mino_kind == MinoKind::T {
                    *test5 = Some(true);
                }

                if self.mino_kind == MinoKind::I {
                    if Environment::is_valid_pos(
                        &field,
                        &self,
                        (&SRSTable::IKICK_TABLE[self.rotation as usize][i])
                            .revert()
                            .x,
                        (&SRSTable::IKICK_TABLE[self.rotation as usize][i])
                            .revert()
                            .y,
                        -5,
                    ) {
                        *srspos = SRSTable::IKICK_TABLE[self.rotation as usize][i].revert();
                        Self::rotate_force_without_srs(self, Rotate::RIGHT, -5);
                        return true;
                    }
                } else {
                    if Environment::is_valid_pos(
                        &field,
                        &self,
                        (&SRSTable::KICK_TABLE[self.rotation as usize][i])
                            .revert()
                            .x,
                        (&SRSTable::KICK_TABLE[self.rotation as usize][i])
                            .revert()
                            .y,
                        -5,
                    ) {
                        *srspos = SRSTable::KICK_TABLE[self.rotation as usize][i].revert();
                        Self::rotate_force_without_srs(self, Rotate::RIGHT, -5);
                        return true;
                    }
                }
            }

            Self::rotate_force_without_srs(self, Rotate::RIGHT, -5);
            return false;
        } else if rotate == Rotate::RIGHT as i8 {
            for i in 0..5 {
                if i == 4 && *test5 != Option::None && self.mino_kind == MinoKind::T {
                    *test5 = Some(true);
                }

                if self.mino_kind == MinoKind::I {
                    if Environment::is_valid_pos(
                        &field,
                        &self,
                        SRSTable::IKICK_TABLE[before_rotate as usize][i].x,
                        SRSTable::IKICK_TABLE[before_rotate as usize][i].y,
                        -5,
                    ) {
                        Self::rotate_force_without_srs(self, Rotate::LEFT, -5);
                        *srspos = SRSTable::IKICK_TABLE[before_rotate as usize][i].clone();
                        return true;
                    }
                } else {
                    if Environment::is_valid_pos(
                        &field,
                        &self,
                        SRSTable::KICK_TABLE[before_rotate as usize][i].x,
                        SRSTable::KICK_TABLE[before_rotate as usize][i].y,
                        -5,
                    ) {
                        Self::rotate_force_without_srs(self, Rotate::LEFT, -5);
                        *srspos = SRSTable::KICK_TABLE[before_rotate as usize][i].clone();
                        return true;
                    }
                }
            }

            Self::rotate_force_without_srs(self, Rotate::LEFT, -5);
            return false;
        } else {
            panic!("そんな回転は存在しない");
        }
    }

    ///その場で強制回転
    #[inline(always)]
    pub fn rotate_force_without_srs(&mut self, rotate: i8, addtemp: i32) {
        let move_pos;
        self.move_pos(addtemp, addtemp);

        match self.mino_kind as i8 {
            MinoKind::J => move_pos = SRSTable::JROTATE_TABLE,
            MinoKind::L => move_pos = SRSTable::LROTATE_TABLE,
            MinoKind::S => move_pos = SRSTable::SROTATE_TABLE,
            MinoKind::Z => move_pos = SRSTable::ZROTATE_TABLE,
            MinoKind::T => move_pos = SRSTable::TROTATE_TABLE,
            MinoKind::I => move_pos = SRSTable::IROTATE_TABLE,
            _ => panic!("なにそれ"),
        }

        self.move_for_srs(&move_pos, rotate, self.rotation);

        Self::get_next_rotate(rotate, &mut self.rotation);
    }

    #[inline(always)]
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
