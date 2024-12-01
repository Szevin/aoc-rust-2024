advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let lists = input.lines().map(|line| {
        line.split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });

    let mut list1 = lists.clone().map(|nums| nums[0]).collect::<Vec<i32>>();
    list1.sort();

    let mut list2 = lists.clone().map(|nums| nums[1]).collect::<Vec<i32>>();
    list2.sort();

    let mut result = 0;
    let mut i = 0;
    while i < list1.len() {
        let diff = list1[i] - list2[i];
        result += diff.abs();
        i += 1;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let lists = input.lines().map(|line| {
        line.split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });

    let list1 = lists.clone().map(|nums| nums[0]).collect::<Vec<i32>>();
    let list2 = lists.clone().map(|nums| nums[1]).collect::<Vec<i32>>();

    list1
        .iter()
        .map(|l| l * list2.iter().filter(|r| r == &l).collect::<Vec<_>>().len() as i32)
        .sum::<i32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
