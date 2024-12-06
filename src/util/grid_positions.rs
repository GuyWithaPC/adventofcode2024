/**
 * Structs to make it easier to define positions in a grid (no more evil messing around with tuples)
 * They are implemented as if positive X is right, negative X is left, positive Y is down, negative Y is up.
 */
use std::ops::{Add, AddAssign, Sub, SubAssign};

/**
 * An integer position
 */
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Position {
        Position { x, y }
    }
}

impl Add<Direction> for Position {
    type Output = Position;
    fn add(self, rhs: Direction) -> Position {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Direction> for Position {
    type Output = Position;
    fn sub(self, rhs: Direction) -> Position {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Direction> for Position {
    fn sub_assign(&mut self, rhs: Direction) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl From<(isize, isize)> for Position {
    fn from(tup: (isize, isize)) -> Position {
        Position::new(tup.0, tup.1)
    }
}

impl From<Position> for (isize, isize) {
    fn from(pos: Position) -> (isize, isize) {
        (pos.x, pos.y)
    }
}

/**
 * an integer direction
 */
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Direction {
    pub x: isize,
    pub y: isize,
}

impl Direction {
    pub fn new(x: isize, y: isize) -> Direction {
        Direction { x, y }
    }

    /**
     * Return a new direction rotated left 90 degrees.
     */
    pub fn rotate_left(&self) -> Direction {
        Direction {
            x: self.y,
            y: -self.x,
        }
    }

    /**
     * Return a new direction rotated right 90 degrees.
     */
    pub fn rotate_right(&self) -> Direction {
        Direction {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn up() -> Direction {
        Direction::new(0, -1)
    }

    pub fn right() -> Direction {
        Direction::new(1, 0)
    }

    pub fn down() -> Direction {
        Direction::new(0, 1)
    }

    pub fn left() -> Direction {
        Direction::new(-1, 0)
    }
}

impl From<(isize, isize)> for Direction {
    fn from(tup: (isize, isize)) -> Direction {
        Direction::new(tup.0, tup.1)
    }
}

impl From<Direction> for (isize, isize) {
    fn from(dir: Direction) -> (isize, isize) {
        (dir.x, dir.y)
    }
}
