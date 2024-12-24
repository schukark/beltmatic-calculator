use std::{error::Error, fmt::Display};

pub trait Operation {
    fn execute(&self, a: i32, b: i32) -> Result<i32, Box<dyn Error>>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OpList {
    Add,
    Mul,
}

impl Operation for OpList {
    fn execute(&self, a: i32, b: i32) -> Result<i32, Box<dyn Error>> {
        match self {
            OpList::Add => Ok(a + b),
            OpList::Mul => Ok(a * b),
        }
    }
}

impl Display for OpList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_string = match self {
            OpList::Add => "+",
            OpList::Mul => "*",
        };

        write!(f, "{op_string}")
    }
}

impl OpList {
    pub const VALUES: [OpList; 2] = [OpList::Add, OpList::Mul];
}
