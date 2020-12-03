use std::collections::HashSet;
use std::fs;

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    print!("Part one: {}\n", part_one()?);
    print!("Part two: {}\n", part_two()?);

    Ok(())
}

fn part_one() -> Result<i32> {
    let input: Vec<i32> = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    let mut seen: HashSet<i32> = HashSet::new();

    for number in input {
        let desirable = 2020 - number;
        if seen.contains(&desirable) {
            return Ok(number * desirable);
        }
        seen.insert(number);
    }

    panic!("No solution exists")
}

fn part_two() -> Result<i32> {
    let input: Vec<i32> = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    let mut seen: HashSet<i32> = HashSet::new();

    for first_number in &input {
        for second_number in &input {
            let desirable = 2020 - first_number - second_number;
            if seen.contains(&desirable) {
                return Ok(first_number * second_number * desirable);
            }
            seen.insert(*second_number);
        }
        seen.insert(*first_number);
    }

    panic!("No solution exists")
}
