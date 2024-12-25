use error::ConfigError;
use solver::{get_best_route, solve};
use std::{collections::HashMap, error::Error};
use tabled::{
    settings::{themes::ColumnNames, Style},
    Table,
};

mod error;
mod grammar;
mod operations;
mod solver;

type Info = HashMap<String, i32>;

fn main() -> Result<(), Box<dyn Error>> {
    let (level_info, limit_info) = grammar::get_info("input.txt")?;
    let goal = *limit_info.get("goal").ok_or(ConfigError::Goal)?;

    let solution = solve(limit_info)?;
    let route = get_best_route(&solution, goal, &level_info)?;

    let mut table = Table::new(route);
    table.with(Style::modern_rounded());
    table.with(ColumnNames::new(["expression", "factories"]));

    println!("{}", table);

    Ok(())
}
