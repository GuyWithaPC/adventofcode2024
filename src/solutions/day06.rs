use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

crate::day!("Guard Gallivant" => {
    part_1,
    part_2
});

type Grid = HashMap<Position, Token>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position(isize, isize); // col, row format
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Direction(isize, isize); // col, row format

impl Add<Direction> for Position {
    type Output = Position;
    fn add(self, rhs: Direction) -> Position {
        return Position(self.0 + rhs.0, self.1 + rhs.1);
    }
}

fn part_1(data: &str) -> usize {
    let map = parse_input(data);
    let mut pos = get_initial_guard_position(&map);
    let mut dir = Direction(0, -1);
    let mut positions: HashSet<Position> = HashSet::new();
    positions.insert(pos);
    'outer: loop {
        match map.get(&(pos + dir)) {
            Some(Token::Obstacle) => dir = dir.rotate(),
            None => break 'outer,
            _ => {
                pos = pos + dir;
                positions.insert(pos);
            }
        }
    }
    positions.len()
}

fn part_2(data: &str) -> usize {
    let map = parse_input(data);
    let mut pos = get_initial_guard_position(&map);
    let mut dir = Direction(0, -1);
    let mut positions: HashSet<Position> = HashSet::new();
    positions.insert(pos);
    'outer: loop {
        match map.get(&(pos + dir)) {
            Some(Token::Obstacle) => {
                dir = dir.rotate();
            }
            None => break 'outer,
            _ => {
                pos = pos + dir;
            }
        }
        positions.insert(pos);
    }
    let mut new_obstacles: HashSet<Position> = HashSet::new();
    let guard_position = get_initial_guard_position(&map);
    for pos in positions {
        if pos == guard_position {
            continue;
        }
        if can_loop(pos, &map) {
            new_obstacles.insert(pos + dir);
        }
    }
    new_obstacles.len()
}

// check whether there's a loopable spot to the right
fn can_loop(pos: Position, map: &Grid) -> bool {
    let mut test_map = map.clone();
    test_map.insert(pos, Token::Obstacle);
    let mut pos = get_initial_guard_position(map);
    let mut dir = Direction(0, -1);
    let mut visited: HashSet<Position> = HashSet::new();
    let mut consecutive_repeats = 0;
    visited.insert(pos);
    loop {
        match test_map.get(&(pos + dir)) {
            Some(Token::Obstacle) => {
                dir = dir.rotate();
            }
            None => {
                return false;
            }
            _ => {
                pos = pos + dir;
                if !visited.insert(pos) {
                    consecutive_repeats += 1;
                    if consecutive_repeats == visited.len() {
                        return true;
                    }
                } else {
                    consecutive_repeats = 0;
                }
            }
        }
    }
}

fn get_initial_guard_position(map: &Grid) -> Position {
    map.iter()
        .filter_map(|(pos, token)| match token {
            Token::Guard => Some(*pos),
            _ => None,
        })
        .next()
        .unwrap()
}

#[derive(Clone, Copy, Debug)]
enum Token {
    Empty,
    Obstacle,
    Guard,
}

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars().enumerate().map(move |(col, c)| match c {
                '#' => (Position(col as isize, row as isize), Token::Obstacle),
                '^' => (Position(col as isize, row as isize), Token::Guard), // guard facing up
                _ => (Position(col as isize, row as isize), Token::Empty),
            })
        })
        .flatten()
        .collect()
}

impl Direction {
    fn rotate(&self) -> Direction {
        match self {
            Direction(0, -1) => Direction(1, 0),
            Direction(1, 0) => Direction(0, 1),
            Direction(0, 1) => Direction(-1, 0),
            _ => Direction(0, -1),
        }
    }
}
