use std::collections::HashSet;
use std::fs;

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;
const HEIGHT_OF_ROUTE: i32 = 323;
const WIDTH_OF_ROUTE: i32 = 31;

fn main() -> Result<()> {
    println!("Part one: {}", part_one()?);
    Ok(())
}

fn part_one() -> Result<i32> {
    let trees = parse_map()?;

    let mut position = [0, 0];
    let mut trees_encountered = 0;

    while position[1] < HEIGHT_OF_ROUTE {
        position = next_position(position);
        let wrapped_position = adjusted_position(position);
        if trees.contains(&wrapped_position) {
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

fn next_position(current: [i32; 2]) -> [i32; 2] {
    [current[0] + 3, current[1] + 1]
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
    assert_eq!([3, 1], next_position([0, 0]));
    assert_eq!([6, 2], next_position([3, 1]))
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
