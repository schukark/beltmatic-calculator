use std::{collections::HashMap, error::Error, fs::read_to_string};

use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "beltmatic.pest"]
struct BeltMaticParser;

type Info = (HashMap<String, i32>, HashMap<String, i32>);

pub fn get_info(input_file: &str) -> Result<Info, Box<dyn Error>> {
    let input = read_to_string(input_file)?;
    let parse = BeltMaticParser::parse(Rule::file, &input)?.next().unwrap();

    let mut parse = parse.into_inner();
    let levels = parse.next().unwrap().clone().into_inner();
    let limits = parse.next().unwrap().clone().into_inner();

    let level_info = parse_levels(levels)?;
    let limit_info = parse_limits(limits)?;

    Ok((level_info, limit_info))
}

fn parse_levels(levels: Pairs<'_, Rule>) -> Result<HashMap<String, i32>, Box<dyn Error>> {
    let mut result = HashMap::new();

    for rule in levels {
        match rule.as_rule() {
            Rule::level_entry => {
                let mut inner = rule.into_inner();

                let key = inner.next().unwrap().as_str();
                let value = inner.next().unwrap().as_str().parse::<i32>()?;

                result.insert(key.to_string(), value);
            }
            _ => unreachable!(),
        }
    }

    Ok(result)
}

fn parse_limits(limits: Pairs<'_, Rule>) -> Result<HashMap<String, i32>, Box<dyn Error>> {
    let mut result = HashMap::new();

    for rule in limits {
        match rule.as_rule() {
            Rule::limit_entry => {
                let mut inner = rule.into_inner();

                let key = inner.next().unwrap().as_str();
                let value = inner.next().unwrap().as_str().parse::<i32>()?;

                result.insert(key.to_string(), value);
            }
            _ => unreachable!(),
        }
    }

    Ok(result)
}
