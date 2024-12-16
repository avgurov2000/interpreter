pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Precedence {
    pub fn from_value(value: i32) -> Option<Self> {
        match value {
            1 => Some(Precedence::Lowest),
            2 => Some(Precedence::Equals),
            3 => Some(Precedence::LessGreater),
            4 => Some(Precedence::Sum),
            5 => Some(Precedence::Product),
            6 => Some(Precedence::Prefix),
            7 => Some(Precedence::Call),
            _ => None,
        }
    }
    pub fn to_value(&self) -> u8 {
        match &self {
            Precedence::Lowest => 1,
            Precedence::Equals => 2,
            Precedence::LessGreater => 3,
            Precedence::Sum => 4,
            Precedence::Product => 5,
            Precedence::Prefix => 6,
            Precedence::Call => 7,
        }
    }
}
