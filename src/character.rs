#[derive(Debug)]
pub enum Character {
    /// Correctly typed character
    Hit(char),

    /// Incorrectly typed character
    Miss(char),

    /// Untyped character
    Empty(char)
}

pub fn classify_character(target: Option<char>, input: Option<char>) -> Character {
    match (target, input) {
        (Some(t), Some(i)) if t == i => Character::Hit(t),
        (Some(_), Some(i)) => Character::Miss(i),
        (None, Some(i)) => Character::Miss(i),
        (Some(t), None) => Character::Empty(t),
        _ => panic!("Unexpected character classification"),
    }
}
