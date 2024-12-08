use crate::util::positions::Vec2;
use itertools::Itertools;

crate::day!("Resonant Collinearity" => {
    part_1,
    part_2
});

type Position = Vec2<isize>;

fn part_1(data: &str) -> usize {
    let grid = parse_input(data);
    let (x_max, y_max) = grid.extents;
    grid.antennae.iter()
        .map(|ants| {
            ants.iter().combinations(2)
                .map(|pair| get_antinodes((*pair[0], *pair[1])))
        })
        .flatten()
        .flatten()
        .unique()
        .filter(|&p| {
            if p.x < 0 || p.x >= x_max as isize {
                return false;
            }
            if p.y < 0 || p.y >= y_max as isize {
                return false;
            }
            return true;
        })
        .count()
}

fn part_2(data: &str) -> usize {
    let grid = parse_input(data);
    grid.antennae.iter()
        .map(|ants| {
            ants.iter().combinations(2)
                .map(|pair| get_all_antinodes((*pair[0], *pair[1]), grid.extents))
        })
        .flatten()
        .flatten()
        .unique()
        .count()
}

fn get_antinodes(positions: (Position, Position)) -> Vec<Position> {
    let (a, b) = positions;
    let d = b - a;
    vec![a - d, b + d]
}

fn get_all_antinodes(positions: (Position, Position), extents: (usize, usize)) -> Vec<Position> {
    let (a, b) = positions;
    let (max_x, max_y) = extents;
    let mut positions = Vec::new();
    let d = b - a;
    let mut i_pos = a;
    loop {
        let check = i_pos - d;
        if check.x < 0 || check.x >= max_x as isize || check.y < 0 || check.y >= max_y as isize {
            break;
        }
        i_pos = check;
    }
    loop {
        positions.push(i_pos);
        let check = i_pos + d;
        if check.x < 0 || check.x >= max_x as isize || check.y < 0 || check.y >= max_y as isize {
            break;
        }
        i_pos = check;
    }
    return positions;
}

#[derive(Debug)]
struct Grid {
    pub antennae: Vec<Vec<Position>>,
    pub extents: (usize, usize) // x,y extents
}

fn parse_input(input: &str) -> Grid {
    let extents: (usize, usize) = (input.lines().next().unwrap().len(), input.lines().count());
    let antennae: Vec<Vec<Position>> = ('0'..='9').chain('a'..='z').chain('A'..='Z')
        .map(|antenna| {
            input.lines().enumerate().map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == antenna {
                        Some(Position::new(x as isize, y as isize))
                    } else {
                        None
                    }
                })
            }).flatten().collect()
        })
        .filter(|l: &Vec<Position>| l.len() != 0)
        .collect();
    Grid {
        antennae,
        extents
    }
}

crate::test_day!(
"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
",
{
    part_1 => 14,
    part_2 => 34
}
);