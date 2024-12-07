use std::{fmt::Debug, thread::sleep, time::Duration};
use colored::Colorize;

advent_of_code::solution!(6);

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

#[derive(Debug)]
struct Coord(i32, i32);

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Coord(x, y)
    }

    fn add(&self, other: &Coord) -> Coord {
        Coord(self.0 + other.0, self.1 + other.1)
    }
}

enum Entity {
    Wall,
    Empty,
}

impl Debug for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entity::Wall => write!(f, "#"),
            Entity::Empty => write!(f, "."),
        }
    }
}

impl Entity {
    fn to_string(&self) -> String {
        match self {
            Entity::Wall => "#".to_string(),
            Entity::Empty => ".".to_string(),
        }
    }
}

struct Tile {
    entity: Entity,
    visited: bool,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.entity)
    }
}

struct Tiles(Vec<Vec<Tile>>);

impl Tiles {
    fn get(&self, coord: &Coord) -> &Tile {
        &self.0[coord.1 as usize][coord.0 as usize]
    }

    fn get_mut(&mut self, coord: &Coord) -> &mut Tile {
        &mut self.0[coord.1 as usize][coord.0 as usize]
    }

    fn set_visited(&mut self, coord: &Coord) {
        self.get_mut(coord).visited = true;
    }

    fn is_in_bounds(&self, coord: &Coord) -> bool {
        coord.0 >= 0 && coord.0 < self.0[0].len() as i32 && coord.1 >= 0 && coord.1 < self.0.len() as i32
    }

    fn count_visited(&self) -> u32 {
        self.0.iter().flatten().filter(|tile| tile.visited).count() as u32
    }

    fn print(&self) {
        for row in &self.0 {
            for tile in row {
                if tile.visited {
                    print!("{}", tile.entity.to_string().red());
                } else {
                    print!("{}", tile.entity.to_string());
                }
            }
            println!();
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_coord(&self) -> Coord {
        match self {
            Direction::Up => Coord(0, -1),
            Direction::Down => Coord(0, 1),
            Direction::Left => Coord(-1, 0),
            Direction::Right => Coord(1, 0),
        }
    }

    fn rotate_by_90(&mut self) -> &Direction {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
        self
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut guard_coord = Coord::new(0, 0);
    let mut guard_direction = Direction::Up;

    let mut tiles: Tiles = Tiles(input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| Tile {
                    entity: match c {
                        '#' => Entity::Wall,
                        '.' => Entity::Empty,
                        '^' => {
                            guard_coord = Coord::new(x as i32, y as i32);
                            Entity::Empty
                        },
                        _ => panic!("Invalid character"),
                    },
                    visited: match c {
                        '^' => true,
                        _ => false,
                    },
                })
                .collect()
        })
        .collect::<Vec<Vec<Tile>>>());

    while tiles.is_in_bounds(&guard_coord.add(&guard_direction.to_coord())) {
        // clear_screen();

        let next_coord = guard_coord.add(&guard_direction.to_coord());
        let next_tile = tiles.get(&next_coord);

        match next_tile.entity {
            Entity::Wall => {
                guard_direction.rotate_by_90();
            },
            Entity::Empty => {
                guard_coord = next_coord;
                tiles.set_visited(&guard_coord);
            },
        }

        // tiles.print();
        // sleep(Duration::from_millis(100));
    }

    tiles.count_visited().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
