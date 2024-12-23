use std::{collections::HashSet, error::Error, fmt, hash::Hash};

use crate::file_utils::read_lines_from_file_v2;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Coord {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Coord {
    fn from(point: (i64, i64)) -> Self {
        Coord {
            x: point.0,
            y: point.1,
        }
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

struct Map {
    obstacles: HashSet<Coord>,

    max_x: i64,
    max_y: i64,
}

impl Map {
    fn new(obstacles: HashSet<Coord>) -> Result<Self, Box<dyn std::error::Error>> {
        let max_x = obstacles
            .iter()
            .map(|o| o.x)
            .max()
            .ok_or("No obstacles to calculate max x")?;

        let max_y = obstacles
            .iter()
            .map(|o| o.y)
            .max()
            .ok_or("No obstacles to calculate max y")?;

        return Ok(Map {
            obstacles,
            max_x,
            max_y,
        });
    }

    fn is_on_map(&self, coord: &Coord) -> bool {
        coord.x >= 0 && coord.x <= self.max_x && coord.y >= 0 && coord.y <= self.max_y
    }

    fn has_obstacle(&self, coord: &Coord) -> bool {
        self.obstacles.contains(coord)
    }

    fn add_obstacle(&mut self, coord: Coord) -> () {
        self.obstacles.insert(coord);
    }

    fn remove_obstacle(&mut self, coord: Coord) -> () {
        self.obstacles.remove(&coord);
    }
}

struct Guard {
    position: Coord,
    direction: Direction,
}

impl Guard {
    fn new(start_position: Coord) -> Self {
        Guard {
            position: start_position,
            direction: Direction::UP,
        }
    }

    fn next_position(&self) -> Coord {
        match self.direction {
            Direction::UP => (self.position.x, self.position.y - 1),
            Direction::DOWN => (self.position.x, self.position.y + 1),
            Direction::RIGHT => (self.position.x + 1, self.position.y),
            Direction::LEFT => (self.position.x - 1, self.position.y),
        }
        .into()
    }

    /// Returns new direction.
    fn rotate(&mut self) -> () {
        self.direction = match self.direction {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }

    /// Makes a step, returns next position.
    fn step(&mut self, map: &Map) -> () {
        let mut next_position = self.next_position();

        while map.has_obstacle(&next_position) {
            self.rotate();
            next_position = self.next_position();
        }

        self.position = next_position
    }
}

fn lines_into_guard_and_map(
    lines: impl Iterator<Item = String>,
) -> Result<(Guard, Map), Box<dyn Error>> {
    let mut obstacles: HashSet<Coord> = HashSet::new();
    let mut guard_pos: Coord = (0, 0).into();

    for (y_pos, a_line) in lines.enumerate() {
        for (x_pos, a_char) in a_line.chars().enumerate() {
            let x_pos: i64 = x_pos.try_into().unwrap();
            let y_pos: i64 = y_pos.try_into().unwrap();

            match a_char {
                '#' => {
                    obstacles.insert((x_pos, y_pos).into());
                }
                '^' => guard_pos = (x_pos, y_pos).into(),
                _ => (),
            }
        }
    }

    let a_map = Map::new(obstacles)?;

    Ok((Guard::new(guard_pos), a_map))
}

fn task1_run(path: &str) -> Result<i64, Box<dyn Error>> {
    let lines = read_lines_from_file_v2(path);
    let (mut guard, map) = lines_into_guard_and_map(lines)?;

    let mut visited: HashSet<Coord> = HashSet::new();

    while map.is_on_map(&guard.position) {
        visited.insert(guard.position);
        guard.step(&map);
    }

    Ok(visited.len().try_into()?)
}

fn task2_run(path: &str) -> Result<i64, Box<dyn Error>> {
    let lines = read_lines_from_file_v2(path);
    todo!()
}

pub fn task1() -> Result<i64, Box<dyn Error>> {
    task1_run("data/day5_test.txt")
}

pub fn task2() -> Result<i64, Box<dyn Error>> {
    task2_run("data/day5_test.txt")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn task1_test_data_test() {
        assert_eq!(41, task1_run("data/day6_test.txt").unwrap())
    }

    #[test]
    fn task1_test() {
        assert_eq!(5067, task1_run("data/day6.txt").unwrap())
    }

    #[test]
    fn task2_test_data_test() {
        assert_eq!(123, task2_run("data/day6_test.txt").unwrap())
    }

    #[test]
    fn task2_test() {
        assert_eq!(4260, task2_run("data/day6.txt").unwrap())
    }
}
