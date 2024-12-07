use std::{fmt::Debug, thread::sleep, time::Duration};
use colored::Colorize;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(6);

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Clone, Copy)]
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

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Entity::Wall, Entity::Wall) => true,
            (Entity::Empty, Entity::Empty) => true,
            _ => false,
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

#[derive(Clone)]
struct Tile {
    coord: Coord,
    entity: Entity,
    visited: bool,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.entity)
    }
}

#[derive(Debug, Clone)]
struct Tiles(Vec<Vec<Tile>>);

impl Tiles {
    fn get(&self, coord: &Coord) -> &Tile {
        &self.0[coord.1 as usize][coord.0 as usize]
    }

    fn get_mut(&mut self, coord: &Coord) -> &mut Tile {
        &mut self.0[coord.1 as usize][coord.0 as usize]
    }

    fn get_empty_coords(&self) -> Vec<Coord> {
        self.0.iter().flatten().filter(|tile| tile.entity == Entity::Empty).map(|tile| tile.coord.clone()).collect()
    }

    fn set_tile(&mut self, coord: &Coord, entity: Entity) {
        self.get_mut(coord).entity = entity;
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

    fn solve(&mut self, guard_starting_coord: Coord, with_print: bool) -> bool {
        let mut guard_coord = guard_starting_coord;
        let mut guard_direction = Direction::Up;

        let mut visited: Vec<(Coord, Direction)> = vec![(guard_coord, guard_direction.clone())];

        while self.is_in_bounds(&guard_coord.add(&guard_direction.to_coord())) {
            let next_coord = guard_coord.add(&guard_direction.to_coord());
            let next_tile = self.get(&next_coord);

            if visited.iter().any(|(coord, dir)| coord == &next_coord && dir == &guard_direction) {
                return false;
            }

            match next_tile.entity {
                Entity::Wall => {
                    guard_direction.rotate_by_90();
                },
                Entity::Empty => {
                    guard_coord = next_coord;
                    visited.push((guard_coord, guard_direction.clone()));
                    self.set_visited(&guard_coord);
                },
            }

            if with_print {
                clear_screen();
                self.print();
                sleep(Duration::from_millis(100));
            }
        }

        true
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

    let mut tiles: Tiles = Tiles(input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| Tile {
                    coord: Coord::new(x as i32, y as i32),
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

    tiles.solve(guard_coord, false);

    tiles.count_visited().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut guard_coord = Coord::new(0, 0);

    let tiles: Tiles = Tiles(input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| Tile {
                    coord: Coord::new(x as i32, y as i32),
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

    let solution = tiles.get_empty_coords().par_iter().filter(|coord| {
        let mut simulated_tiles = tiles.clone();
        simulated_tiles.set_tile(coord, Entity::Wall);
        !simulated_tiles.solve(guard_coord, false)
    }).count();

    Some(solution as u32)
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
        assert_eq!(result, Some(6));
    }
}
