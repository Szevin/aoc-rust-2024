use std::{collections::{HashMap}, hash::Hash};

use itertools::Itertools;

advent_of_code::solution!(8);

fn is_out_of_bound(grid_end: Coord, pos: Coord) -> bool {
    pos.0 < 0 || pos.1 < 0 || pos.0 > grid_end.0 || pos.1 > grid_end.1
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Coord(i32, i32);

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Coord(x, y)
    }

    fn add(&self, other: &Coord) -> Coord {
        Coord(self.0 + other.0, self.1 + other.1)
    }

    fn sub(&self, other: &Coord) -> Coord {
        Coord(self.0 - other.0, self.1 - other.1)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut antennas: HashMap<char, Vec<Coord>> = HashMap::new();

    let grid = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let grid_end = Coord::new((grid[0].len() as i32)-1, (grid.len() as i32)-1);

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            match char {
                '.' => {},
                _ => {
                    let mut new_values = vec![Coord::new(x as i32, y as i32)];

                    if antennas.contains_key(&char) {
                        let old_values = antennas.get(&char).unwrap();
                        new_values = old_values.iter().cloned().chain(new_values.iter().cloned()).collect();
                    }

                    antennas.insert(char, new_values);
                }
            }
        });
    });

    let antinodes = antennas
    .iter()
    .filter(|(_, coords)| coords.len() > 1)
    .map(|(_, antenna_coords)| {
        antenna_coords.iter().tuple_combinations().map(|(first, second)| {
            let diff_vector = first.sub(second);

            vec![first.add(&diff_vector), second.sub(&diff_vector)]
        }).flatten().collect::<Vec<Coord>>()
    })
    .flatten()
    .filter(|antinode| {
        !is_out_of_bound(grid_end, *antinode)
    })
    .unique()
    .collect::<Vec<Coord>>();

    (antinodes.len() as u32).into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut antennas: HashMap<char, Vec<Coord>> = HashMap::new();

    let grid = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let grid_end = Coord::new((grid[0].len() as i32)-1, (grid.len() as i32)-1);

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            match char {
                '.' => {},
                _ => {
                    let mut new_values = vec![Coord::new(x as i32, y as i32)];

                    if antennas.contains_key(&char) {
                        let old_values = antennas.get(&char).unwrap();
                        new_values = old_values.iter().cloned().chain(new_values.iter().cloned()).collect();
                    }

                    antennas.insert(char, new_values);
                }
            }
        });
    });

    let antinodes = antennas
    .iter()
    .filter(|(_, coords)| coords.len() > 1)
    .map(|(_, antenna_coords)| {
        antenna_coords.iter().tuple_combinations().map(|(first, second)| {
            let diff_vector = first.sub(second);

            let mut first_antennas = vec![];
            let mut current_coord = *first;
            while !is_out_of_bound(grid_end, current_coord) {
                first_antennas.push(current_coord);

                current_coord = current_coord.add(&diff_vector);
            }

            let mut second_antennas = vec![];
            let mut current_coord = *second;
            while !is_out_of_bound(grid_end, current_coord) {
                second_antennas.push(current_coord);

                current_coord = current_coord.sub(&diff_vector);
            }

            first_antennas.iter().cloned().chain(second_antennas.iter().cloned()).collect::<Vec<Coord>>()
        }).flatten().collect::<Vec<Coord>>()
    })
    .flatten()
    .filter(|antinode| {
        !is_out_of_bound(grid_end, *antinode)
    })
    .unique()
    .collect::<Vec<Coord>>();

    (antinodes.len() as u32).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
