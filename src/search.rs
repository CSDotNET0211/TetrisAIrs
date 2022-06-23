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
    field: [bool; _],
    first_move: i64,
    before_eval: f64,
}

impl Data {
    pub const fn new() {}
}

struct BeemSearch {}
