use std::str::FromStr;

advent_of_code::solution!(2);

#[derive(Debug)]
struct Report {
    levels: Vec<u32>,
}

impl Report {
    fn is_safe(&self) -> bool {
        let is_desc = self.levels.windows(2).all(|w| {
            let (a, b) = (w[0], w[1]);
            a < b && a.abs_diff(b) <= 3
        });

        let is_asc = self.levels.windows(2).all(|w| {
            let (a, b) = (w[0], w[1]);
            a > b && a.abs_diff(b) <= 3
        });

        is_desc || is_asc
    }

    fn is_almost_safe(&self) -> bool {
        for (index_to_skip, _) in self.levels.iter().enumerate() {
            let mut levels = self.levels.clone();
            levels.remove(index_to_skip);

            let is_desc = levels.windows(2).all(|w| {
                let (a, b) = (w[0], w[1]);
                a < b && a.abs_diff(b) <= 3
            });

            let is_asc = levels.windows(2).all(|w| {
                let (a, b) = (w[0], w[1]);
                a > b && a.abs_diff(b) <= 3
            });

            if is_desc || is_asc {
                return true;
            }
        }

        false
    }
}

impl FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        Ok(Report { levels })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|report| report.parse::<Report>().unwrap())
        .filter(|r| r.is_safe())
        .count()
        .try_into()
        .ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|report| report.parse::<Report>().unwrap())
        .filter(|r| r.is_safe() || r.is_almost_safe())
        .count()
        .try_into()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
