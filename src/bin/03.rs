use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\([0-9]{1,3}\,[0-9]{1,3}\)").unwrap();
    re.find_iter(input).map(|m| m.as_str()).map(|m| {
        let (a, b) = m.split_once(",").unwrap();
        a[4..].parse::<u32>().unwrap() * b[..b.len() - 1].parse::<u32>().unwrap()

    }).sum::<u32>().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul\([0-9]{1,3}\,[0-9]{1,3}\))|(do\(\))|(don't\(\))").unwrap();

    let mut mul_enabled = true;

    re.find_iter(input).map(|m| m.as_str()).map(|m| {
        match m {
            "do()" => {
                mul_enabled = true;
                0
            },
            "don't()" => {
                mul_enabled = false;
                0
            },
            _ => {
                if mul_enabled {
                    let (a, b) = m.split_once(",").unwrap();
                    a[4..].parse::<u32>().unwrap() * b[..b.len() - 1].parse::<u32>().unwrap()
                } else {
                    0
                }
            }
        }
    }).sum::<u32>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
