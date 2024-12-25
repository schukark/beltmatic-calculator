use std::{collections::HashMap, error::Error};

use crate::{error::ConfigError, operations::*, Info};

pub type Dp = (HashMap<i32, i32>, HashMap<i32, (i32, i32, OpList)>); // Min count, mem

pub fn get_best_route(
    dp: &Dp,
    goal: i32,
    level_info: &Info,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let belt_level = *level_info.get("belt").ok_or(ConfigError::BeltLevel)? - 1;
    let extractor_level = *level_info
        .get("extractor")
        .ok_or(ConfigError::ExtractorLevel)?
        - 1;
    let adder_level = *level_info.get("adder").ok_or(ConfigError::AdderLevel)? - 1;
    let multiplier_level = *level_info
        .get("multiplier")
        .ok_or(ConfigError::MultiplierLevel)?
        - 1;

    let (_, mem) = dp;

    if !mem.contains_key(&goal) {
        let belt_count = EXTRACTOR[extractor_level as usize] * BELT[belt_level as usize];
        return Ok(vec![(
            format!("(extractor) -> {}", goal),
            format!("({} numbers/s out)", belt_count),
        )]);
    }

    let (i, j, op) = *mem.get(&goal).ok_or(ConfigError::Goal)?;
    let left_route = get_best_route(dp, i, level_info)?;
    let mut right_route = get_best_route(dp, j, level_info)?;

    let mut result = left_route.clone();

    result.append(&mut right_route);

    let mut this_string = (format!("{i} {op} {j} -> {goal}"), "".to_owned());

    if op == OpList::Add {
        let influx = BELT[belt_level as usize] * EXTRACTOR[extractor_level as usize];
        this_string.1 += &format!(
            "{} adders required",
            f32::ceil(influx / ADDER[adder_level as usize]) as i32
        );
    } else if op == OpList::Mul {
        let influx = BELT[belt_level as usize] * EXTRACTOR[extractor_level as usize];
        this_string.1 += &format!(
            "{} multipliers required",
            f32::ceil(influx / MULTIPLIER[multiplier_level as usize]) as i32
        );
    }

    result.push(this_string);

    Ok(result)
}

pub fn solve(limit_info: Info) -> Result<Dp, Box<dyn Error>> {
    let goal = *limit_info.get("goal").ok_or(ConfigError::Goal)?;
    let extractor_limit = *limit_info.get("limit").ok_or(ConfigError::Limit)?;

    let mut dist: HashMap<i32, i32> = HashMap::new();
    let mut mem = HashMap::new();

    // We can mine those directly
    for i in 1..=extractor_limit {
        dist.insert(i, 1);
    }

    let mut change = true;

    while change {
        change = false;

        if dist.contains_key(&goal) {
            break;
        }

        let mut new_dist = dist.clone();

        for (&i, &d_i) in &dist {
            for (&j, &d_j) in &dist {
                for op in OpList::VALUES {
                    if !dist.contains_key(&i) || !dist.contains_key(&j) {
                        continue;
                    }

                    let new_value = op.execute(i, j);

                    if new_value.is_err() {
                        continue;
                    }

                    let new_value = new_value.unwrap();
                    let count = i32::max(d_i, d_j) + 1;

                    if *dist.get(&new_value).unwrap_or(&i32::MAX) > count {
                        new_dist.insert(new_value, count);
                        mem.insert(new_value, (i, j, op));
                        change = true;
                    }
                }
            }
        }

        dist = new_dist;
    }

    Ok((dist, mem))
}