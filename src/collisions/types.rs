use std::fmt;


#[derive(Debug,PartialEq,Clone)]
pub enum UpDown {
    Up,
    Down
}
#[derive(Debug,PartialEq,Clone)]
pub enum LeftRight {
    Left,
    Right,
    None
}
impl fmt::Display for LeftRight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LeftRight::Left => write!(f, "Left"),
            LeftRight::Right => write!(f, "Right"),
            LeftRight::None => write!(f, "None"),
        }
    }
}
