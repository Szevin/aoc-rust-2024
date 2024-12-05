use std::{collections::HashMap, ops::Index};

advent_of_code::solution!(5);

fn parse_rules(input: &str) -> HashMap<u32, Vec<u32>> {
    let rules: HashMap<u32, Vec<u32>> = input.lines().fold(HashMap::new(), |mut acc, rule| {
        let (lower, higher) = rule.split_once("|").unwrap();

        let lower = lower.parse::<u32>().unwrap();
        let value = higher.parse::<u32>().unwrap();

        if acc.contains_key(&lower) {
            let mut highers = acc.get(&lower).unwrap().clone();
            highers.push(value);

            acc.insert(lower, highers);
        } else {
            acc.insert(lower, vec![value]);
        }

        acc
    });

    rules
}

fn parse_updates(input: &str) -> Vec<Vec<u32>> {
    input
    .lines()
    .map(|update|
        update
        .split(",")
        .map(|page| page.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
    ).collect()
}

fn divide_updates_by_validity<'a>(rules: &'a HashMap<u32, Vec<u32>>) -> impl FnMut((Vec<Vec<u32>>, Vec<Vec<u32>>), &'a Vec<u32>) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) + 'a {
    |mut acc, update| {
        let mut is_valid = true;

        for (lower, highers) in rules.iter() {
            if !update.contains(lower) {
                continue;
            }

            for higher in highers {
                if !update.contains(higher) {
                    continue;
                }

                let lower_index = update.iter().position(|&x| x == *lower).unwrap();
                let higer_index = update.iter().position(|&x| x == *higher).unwrap();
                if lower_index > higer_index {
                    is_valid = false;
                    break;
                }
            }

            if !is_valid {
                break;
            }
        }

        if is_valid {
            acc.0.push(update.clone());
        } else {
            acc.1.push(update.clone());
        }

        acc
    }
}

fn sum_middle_values(acc: u32, update: &Vec<u32>) -> u32 {
    acc + update[update.len() / 2]
}

pub fn part_one(input: &str) -> Option<u32> {
    let rows = input.lines().collect::<Vec<&str>>();
    let mut rows_split = rows.split(|row| row == &"");

    let rules = parse_rules(rows_split.next().unwrap().join("\n").as_str());
    let updates = parse_updates(rows_split.next().unwrap().join("\n").as_str());

    updates.iter().fold((vec![], vec![]), divide_updates_by_validity(&rules)).0.iter().fold(0, sum_middle_values).into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows = input.lines().collect::<Vec<&str>>();
    let mut rows_split = rows.split(|row| row == &"");

    let rules = parse_rules(rows_split.next().unwrap().join("\n").as_str());
    let updates = parse_updates(rows_split.next().unwrap().join("\n").as_str());

    let invalid_updates = updates.iter().fold((vec![], vec![]), divide_updates_by_validity(&rules)).1;

    let fixed_updates = invalid_updates.iter().map(|update| {
        let mut is_fixed = false;
        let mut fixed_update = update.clone();

        while !is_fixed {
            is_fixed = true;

            for (lower, highers) in rules.iter() {
                if !fixed_update.contains(lower) || !highers.iter().any(|higher| fixed_update.contains(higher)) {
                    continue;
                }

                let lower_index = fixed_update.iter().position(|&x| x == *lower).unwrap();
                let first_higer_index = fixed_update.iter().position(|&x| highers.contains(&x)).unwrap();

                if lower_index > first_higer_index {
                    fixed_update.swap(lower_index, first_higer_index);
                    is_fixed = false;
                    break;
                }
            }
        }

        fixed_update
    }).collect::<Vec<Vec<u32>>>();

    fixed_updates.iter().fold(0, sum_middle_values).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
