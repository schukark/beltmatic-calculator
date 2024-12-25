use std::{error::Error, fmt::Display};

use crate::Info;

pub trait Operation {
    fn execute(&self, a: i32, b: i32) -> Result<i32, Box<dyn Error>>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OpList {
    Add,
    Mul,
    Sub,
    Div,
    Exp,
}

impl Operation for OpList {
    fn execute(&self, a: i32, b: i32) -> Result<i32, Box<dyn Error>> {
        match self {
            OpList::Add => Ok(a + b),
            OpList::Mul => Ok(a * b),
            OpList::Sub => Ok(a - b),
            OpList::Div => {
                if b == 0 {
                    Err("Division by zero".into())
                } else {
                    Ok(a / b)
                }
            }
            OpList::Exp => Ok(a.pow(b as u32)),
        }
    }
}

impl Display for OpList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_string = match self {
            OpList::Add => "+",
            OpList::Mul => "*",
            OpList::Sub => "-",
            OpList::Div => "/",
            OpList::Exp => "^",
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
pub const SUBTRACTOR: [f32; 8] = [0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 1.0];
pub const DIVIDER: [f32; 8] = [0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 1.0];
pub const EXPONENTIATOR: [f32; 8] = [0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 1.0];

impl OpList {
    const VALUES: [OpList; 5] = [
        OpList::Add,
        OpList::Mul,
        OpList::Sub,
        OpList::Div,
        OpList::Exp,
    ];

    pub fn get_factory_name(&self) -> &'static str {
        match self {
            OpList::Add => "adder",
            OpList::Mul => "multiplier",
            OpList::Sub => "subtractor",
            OpList::Div => "divider",
            OpList::Exp => "exponentiator",
        }
    }

    pub fn get_factory_throughput(&self, level: usize) -> f32 {
        match self {
            OpList::Add => ADDER[level],
            OpList::Mul => MULTIPLIER[level],
            OpList::Sub => SUBTRACTOR[level],
            OpList::Div => DIVIDER[level],
            OpList::Exp => EXPONENTIATOR[level],
        }
    }

    pub fn from_level_info(level_info: &Info) -> Vec<OpList> {
        OpList::VALUES
            .iter()
            .cloned()
            .filter(|op| level_info.contains_key(op.get_factory_name()))
            .collect()
    }
}
