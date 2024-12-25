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

pub const BELT: [f32; 14] = [
    1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 8.1, 8.2, 8.3, 8.4, 8.5, 9.6,
];
pub const EXTRACTOR: [f32; 8] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
pub const ADDER: [f32; 8] = [0.25, 0.333, 0.4, 0.5, 0.667, 1.0, 1.5, 2.0];
pub const MULTIPLIER: [f32; 8] = [0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 1.0];

impl OpList {
    pub const VALUES: [OpList; 2] = [OpList::Add, OpList::Mul];

    pub fn get_factory_name(&self) -> &'static str {
        match self {
            OpList::Add => "adder",
            OpList::Mul => "multiplier",
        }
    }
}
