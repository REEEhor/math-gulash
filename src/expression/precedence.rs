
#[derive(Clone, Copy)]
pub struct Precedence(pub(super) u32);

impl Precedence {
    pub const fn eval_first() -> Self {
        Self(0)
    }

    pub const fn is_before(self, other: Self) -> bool {
        self.0 < other.0
    }

    pub const fn is_before_or_same(self, other: Self) -> bool {
        self.0 <= other.0
    }
}

pub const ADDITION: Precedence = Precedence(5);

pub const MULTIPLICATION: Precedence = Precedence(4);
pub const DIVISION: Precedence = Precedence(3);

pub const UNARY_MINUS: Precedence = Precedence(2);
pub const EXPONENTIATION: Precedence = Precedence(1);

pub const NUMBER: Precedence = Precedence::eval_first();
pub const VARIABLE: Precedence = Precedence::eval_first();
