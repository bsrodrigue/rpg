use std::fmt;

pub struct Attack {
    pub name: String,
    pub damage: u64,
}

impl fmt::Display for Attack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
