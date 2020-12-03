use std::fs;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    println!("Part one: {}", part_one()?);
    println!("Part two: {}", part_two()?);

    Ok(())
}

fn part_one() -> Result<i32> {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");

    let mut count = 0;

    for line in input.lines() {
        // string is of format: 4-5 l: rllllj
        // <range> <letter>: <password>
        let line_parts: Vec<&str> = line.split(" ").collect();

        let range: Vec<&str> = line_parts[0].split("-").collect();
        let range_start: usize = range[0].parse()?;
        let range_end: usize = range[1].parse()?;
        let letter = line_parts[1].replace(":", "");
        let password = line_parts[2];

        let occurances = password.matches(&letter).count();

        if occurances >= range_start && occurances <= range_end {
            count = count + 1;
        }
    }
    Ok(count)
}

fn part_two() -> Result<i32> {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");

    let mut count = 0;

    for line in input.lines() {
        let line_parts: Vec<&str> = line.split(" ").collect();

        let range: Vec<&str> = line_parts[0].split("-").collect();
        let first_index: usize = range[0].parse()?;
        let second_index: usize = range[1].parse()?;
        let letter = line_parts[1].replace(":", "");
        let password = line_parts[2];

        let first_letter_to_check: &str = &password[(first_index - 1)..first_index];
        let second_letter_to_check: &str = &password[(second_index - 1)..second_index];
        let first_ok = first_letter_to_check.matches(&letter).count() == 1;
        let second_ok = second_letter_to_check.matches(&letter).count() == 1;

        if (first_ok && !second_ok) || (!first_ok && second_ok) {
            count = count + 1;
        }
    }
    return Ok(count);
}
