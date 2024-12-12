use std::collections::HashSet;

use crate::util::{grid::Grid, positions::Vec2};

crate::day!("Garden Groups" => {
    part_1
});

fn part_1(data: &str) -> usize {
    let mut grid = parse_input(data);
    let mut result = 0;
    loop {
        if let Some((area, perimeter)) = isolate_region(&mut grid) {
            result += area * perimeter;
        } else {
            break;
        }
    }
    return result;
}

fn isolate_region(grid: &mut Grid<char>) -> Option<(usize, usize)> {
    let possible_start = grid.iter_coords()
        .filter_map(|(coord, plant): (Vec2<usize>, _)| match plant.to_owned() {
            Some(p) => Some((coord.components_try_into::<isize>().unwrap(), *p)),
            None => None
        })
        .next();
    if possible_start.is_none() {
        return None;
    }
    let (start, plant) = possible_start.unwrap();
    let directions: [Vec2<isize>;4] = [Vec2::up(), Vec2::down(), Vec2::left(), Vec2::right()];
    let mut area = 0usize;
    let mut perimeter = 0usize;
    let mut to_check: Vec<Vec2<isize>> = Vec::new();
    let mut visited: HashSet<Vec2<isize>> = HashSet::new();
    to_check.push(start);
    loop {
        if let Some(pos) = to_check.pop() {
            if visited.contains(&pos) {
                continue;
            }
            area += 1;
            grid.remove_coord(pos);
            visited.insert(pos);
            for dir in directions {
                match grid.get_coord(pos + dir) {
                    None => if !visited.contains(&(pos + dir)) {
                        perimeter += 1
                    }
                    Some(p) => if *p == plant {
                        to_check.push(pos + dir);
                    } else if !visited.contains(&(pos + dir)){
                        perimeter += 1
                    }
                }
            }
        } else {
            break;
        }
    }
    return Some((area, perimeter));
}

fn parse_input(input: &str) -> Grid<char> {
    input.lines().map(|line| {
        line.chars().collect::<Vec<char>>()
    }).collect()
}

crate::test_day!(
"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
{
    part_1 => 1930
}
);