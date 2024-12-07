use itertools::Itertools;

advent_of_code::solution!(7);

enum Operation {
    Add,
    Multiply,
    Concat,
}

impl Operation {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
            Operation::Concat => ([a.to_string(), b.to_string()].join("")).parse::<u64>().ok().unwrap(),
        }
    }
}

fn solve(operations: &Vec<Operation>, acc: u64, expected: u64, numbers: Vec<u64>) -> bool {
    if numbers.len() == 0 {
        return acc == expected;
    }

    if acc > expected {
        return false;
    }

    let next_number = numbers.first().unwrap();

    operations.iter().any(|operation| {
        solve(operations, operation.apply(acc, *next_number), expected, numbers[1..].to_vec())
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let operations: &Vec<Operation> = &vec![Operation::Add, Operation::Multiply];

    input.lines().filter_map(|line| {
        if let Some((expected, rest)) = line.split_once(":") {
            Some((expected.parse::<u64>().ok().unwrap(), rest.trim().split(" ").map(|n| n.parse::<u64>().ok().unwrap()).collect::<Vec<u64>>()))
        } else {
            None
        }
    })
    .filter_map(|(expected, numbers)| {
        match solve(operations, 0, expected, numbers) {
            true => Some(expected),
            false => None
        }
    }).sum::<u64>().into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let operations: &Vec<Operation> = &vec![Operation::Add, Operation::Multiply, Operation::Concat];

    input.lines().filter_map(|line| {
        if let Some((expected, rest)) = line.split_once(":") {
            Some((expected.parse::<u64>().ok().unwrap(), rest.trim().split(" ").map(|n| n.parse::<u64>().ok().unwrap()).collect::<Vec<u64>>()))
        } else {
            None
        }
    })
    .filter_map(|(expected, numbers)| {
        match solve(operations, 0, expected, numbers) {
            true => Some(expected),
            false => None
        }
    }).sum::<u64>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
