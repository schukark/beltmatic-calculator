use std::{collections::HashMap, error::Error};

use crate::{
    error::ConfigError,
    operations::{OpList, Operation},
    Info,
};

pub type Dp = (HashMap<i32, i32>, HashMap<i32, (i32, i32, OpList)>); // Min count, mem

pub fn calc_dp(limit_info: Info) -> Result<Dp, Box<dyn Error>> {
    let mut dp: HashMap<i32, i32> = HashMap::new();
    let mut mem: HashMap<i32, (i32, i32, OpList)> = HashMap::new();

    dp.insert(0, 0);

    let goal = *limit_info.get("goal").ok_or(ConfigError::Goal)?;
    let extractor_limit = *limit_info.get("limit").ok_or(ConfigError::Limit)?;

    // We can mine those directly
    for i in 1..=extractor_limit {
        dp.insert(i, 1);
    }

    for i in extractor_limit + 1..=goal {
        dp.insert(i, i32::MAX);
    }

    for i in 1..=goal {
        for j in 1..=i {
            for op in OpList::VALUES {
                if !dp.contains_key(&i) || !dp.contains_key(&j) {
                    continue;
                }

                let new_value = op.execute(i, j);

                if new_value.is_err() {
                    continue;
                }

                let new_value = new_value.unwrap();
                let count = i32::max(*dp.get(&i).unwrap(), *dp.get(&j).unwrap()) + 1;

                if new_value > goal {
                    continue;
                }

                if *dp.get(&new_value).unwrap() > count {
                    // we can optimize this value
                    dp.insert(new_value, count);
                    mem.insert(new_value, (i, j, op));
                }
            }
        }
    }

    Ok((dp, mem))
}

const BELT: [f32; 14] = [
    1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 8.1, 8.2, 8.3, 8.4, 8.5, 9.6,
];
const EXTRACTOR: [f32; 8] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
const ADDER: [f32; 8] = [0.25, 0.333, 0.4, 0.5, 0.667, 1.0, 1.5, 2.0];
const MULTIPLIER: [f32; 8] = [0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 1.0];

pub fn get_best_route(
    dp: &Dp,
    goal: i32,
    level_info: &Info,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let belt_level = *level_info.get("belt").ok_or(ConfigError::BeltLevel)?;
    let extractor_level = *level_info
        .get("extractor")
        .ok_or(ConfigError::ExtractorLevel)?;
    let adder_level = *level_info.get("adder").ok_or(ConfigError::AdderLevel)?;
    let multiplier_level = *level_info
        .get("multiplier")
        .ok_or(ConfigError::MultiplierLevel)?;

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
