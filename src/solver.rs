use std::{collections::HashMap, error::Error};

use crate::{error::ConfigError, operations::*, Info};

pub type Dp = HashMap<i32, (i32, i32, Option<OpList>)>; // Min count, mem

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

    let mem = dp;

    if !mem.contains_key(&goal) {
        return Err(Box::new(ConfigError::UnreachableNumber));
    }

    if mem.get(&goal).unwrap().2.is_none() {
        let belt_count = EXTRACTOR[extractor_level as usize] * BELT[belt_level as usize];
        return Ok(vec![(
            format!("(extractor) -> {}", goal),
            format!("{}/s out", belt_count),
        )]);
    }

    let (i, j, op) = *mem.get(&goal).ok_or(ConfigError::Goal)?;
    let op = op.unwrap();
    let left_route = get_best_route(dp, i, level_info)?;
    let mut right_route = get_best_route(dp, j, level_info)?;

    let mut result = left_route.clone();
    result.append(&mut right_route);
    let mut this_string = (format!("{i} {op} {j} -> {goal}"), "".to_owned());

    let factory_name = op.get_factory_name();
    let building_level = *level_info
        .get(factory_name)
        .ok_or(ConfigError::BuildingMissing(factory_name))?
        - 1;

    let influx = BELT[belt_level as usize] * EXTRACTOR[extractor_level as usize];
    this_string.1 += &format!(
        "{} {}s",
        f32::ceil(influx / op.get_factory_throughput(building_level as usize)),
        factory_name
    );

    result.push(this_string);

    Ok(result)
}

const LIMIT: i32 = 100;
#[allow(clippy::map_entry)]
pub fn solve(limit_info: Info, available_ops: &[OpList]) -> Result<Dp, Box<dyn Error>> {
    let goal = *limit_info.get("goal").ok_or(ConfigError::Goal)?;
    let extractor_limit = *limit_info.get("limit").ok_or(ConfigError::Limit)?;

    let mut dist: HashMap<i32, i32> = HashMap::new();
    let mut mem: HashMap<i32, (i32, i32, Option<OpList>)> = HashMap::new();

    // We can mine those directly
    for i in 1..=extractor_limit {
        dist.insert(i, 0);
        mem.insert(i, (i, 0, None));
    }

    for _iter in 1..LIMIT {
        for i in (-2 * goal)..=(2 * goal) {
            for j in (-2 * goal)..=(2 * goal) {
                if !dist.contains_key(&i) || !dist.contains_key(&j) {
                    continue;
                }

                let d_i = *dist.get(&i).unwrap();
                let d_j = *dist.get(&j).unwrap();

                for &op in available_ops {
                    let new_value = op.execute(i, j);

                    if new_value.is_err() {
                        continue;
                    }

                    let new_value = new_value.unwrap();

                    let count = i32::max(d_i, d_j) + 1;

                    if !dist.contains_key(&new_value) || *dist.get(&new_value).unwrap() > count {
                        dist.insert(new_value, count);
                        mem.insert(new_value, (i, j, Some(op)));
                    }
                }
            }
        }

        if mem.contains_key(&goal) {
            break;
        }
    }

    Ok(mem)
}
