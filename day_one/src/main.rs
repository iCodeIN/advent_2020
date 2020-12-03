use std::fs;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    part_one()?;
    part_two()?;

    Ok(())
}

fn part_one() -> Result<()> {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");

    for line in input.lines() {
        for line2 in input.lines() {
            let first_number: i32 = line.parse()?;
            let second_number: i32 = line2.parse()?;
            if first_number + second_number == 2020 {
                print!("{}\n", first_number * second_number);
                return Ok(());
            }
        }
    }
    Ok(())
}

fn part_two() -> Result<()> {
    let input =
        fs::read_to_string("src/input.full.txt").expect("Something went wrong reading the file");

    for line in input.lines() {
        for line2 in input.lines() {
            for line3 in input.lines() {
                let first_number: i32 = line.parse()?;
                let second_number: i32 = line2.parse()?;
                let third_number: i32 = line3.parse()?;
                if first_number + second_number + third_number == 2020 {
                    print!("{}\n", first_number * second_number * third_number);
                    return Ok(());
                }
            }
        }
    }
    Ok(())
}
