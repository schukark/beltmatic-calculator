use dp::{calc_dp, get_best_route};
use error::ConfigError;
use std::{collections::HashMap, error::Error};
use tabled::{
    settings::{themes::ColumnNames, Style},
    Table,
};

mod dp;
mod error;
mod grammar;
mod operations;

type Info = HashMap<String, i32>;

fn main() -> Result<(), Box<dyn Error>> {
    let (level_info, limit_info) = grammar::get_info("input.txt")?;
    let goal = *limit_info.get("goal").ok_or(ConfigError::Goal)?;

    let dp = calc_dp(limit_info)?;
    let route = get_best_route(&dp, goal, &level_info)?;

    let mut table = Table::new(route);
    table.with(Style::modern_rounded());
    table.with(ColumnNames::new(["expression", "factories"]));

    println!("{}", table);

    Ok(())
}
