use std::collections::HashSet;
use std::fs;

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;
const HEIGHT_OF_ROUTE: i32 = 323;
const WIDTH_OF_ROUTE: i32 = 31;

fn main() -> Result<()> {
    println!("Part one: {}", part_one()?);
    println!("Part two: {}", part_two()?);
    Ok(())
}

fn part_one() -> Result<i64> {
    let trees: HashSet<[i32; 2]> = parse_map()?;
    return count_trees_encountered(&trees, 3, 1);
}

fn part_two() -> Result<i64> {
    let trees: HashSet<[i32; 2]> = parse_map()?;
    let mut result: i64 = count_trees_encountered(&trees, 1, 1)?;
    result = result * count_trees_encountered(&trees, 3, 1)?;
    result = result * count_trees_encountered(&trees, 5, 1)?;
    result = result * count_trees_encountered(&trees, 7, 1)?;
    result = result * count_trees_encountered(&trees, 1, 2)?;

    Ok(result)
}

fn count_trees_encountered(trees: &HashSet<[i32; 2]>, x_move: i32, y_move: i32) -> Result<i64> {
    let mut trees_encountered = 0;
    let mut position = [0, 0];
    while position[1] < HEIGHT_OF_ROUTE {
        position = next_position(position, x_move, y_move);
        let adjusted = adjusted_position(position);
        if trees.contains(&adjusted) {
            trees_encountered = trees_encountered + 1
        }
    }

    Ok(trees_encountered)
}

fn parse_map() -> Result<HashSet<[i32; 2]>> {
    let mut trees: HashSet<[i32; 2]> = HashSet::new();

    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for terrain in line.chars() {
            if terrain == '#' {
                trees.insert([x, y]);
            }
            x = x + 1;
        }
        y = y + 1
    }

    Ok(trees)
}

fn next_position(current: [i32; 2], x_move: i32, y_move: i32) -> [i32; 2] {
    [current[0] + x_move, current[1] + y_move]
}

fn adjusted_position(current: [i32; 2]) -> [i32; 2] {
    [current[0] % (WIDTH_OF_ROUTE), current[1]]
}

#[test]
fn can_parse_map() {
    let trees = parse_map().expect("");
    assert_eq!(false, trees.contains(&[0, 0]));
    assert_eq!(true, trees.contains(&[0, 7]));
}

#[test]
fn can_plot_route() {
    assert_eq!([3, 1], next_position([0, 0], 3, 1));
    assert_eq!([6, 2], next_position([3, 1], 3, 1))
}

#[test]
fn can_calculate_adjusted_position_leaves_y() {
    assert_eq!([0, 0], adjusted_position([0, 0]));
    assert_eq!([0, 10], adjusted_position([0, 10]));
}

#[test]
fn can_calculate_adjusted_position_fixes_x() {
    assert_eq!([0, 0], adjusted_position([0, 0]));
    assert_eq!([30, 0], adjusted_position([30, 0]));
    assert_eq!([0, 0], adjusted_position([31, 0]));
    assert_eq!([1, 0], adjusted_position([32, 0]));
    assert_eq!([29, 0], adjusted_position([60, 0]));
}

#[test]
fn can_count_trees() {
    let mut trees: HashSet<[i32; 2]> = HashSet::new();
    trees.insert([0, 1]);
    trees.insert([0, HEIGHT_OF_ROUTE]);
    assert_eq!(2, count_trees_encountered(&trees, 0, 1).expect("bang"))
}
