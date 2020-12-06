use std::fs;

fn main() {
    println!("Day One: {}", part_one());
}

fn part_one() -> i32 {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let passports = file_to_passports(&input);
    // make into Vec<Hashmap>
    // count valid passports by precense of keys in map
    0
}

fn file_to_passports(input: &str) -> Vec<String> {
    let mut passports: Vec<String> = Vec::new();

    let mut current_passport: Vec<&str> = Vec::new();
    for line in input.lines() {
        if line.len() == 0 {
            passports.push(current_passport.join(" "));
            current_passport = Vec::new();
        } else {
            current_passport.push(line.trim());
        }
    }
    passports.push(current_passport.join(" "));

    return passports;
}

#[test]
fn can_separate_passports_into_single_lines_of_data() {
    let input =
        fs::read_to_string("src/example.txt").expect("Something went wrong reading the file");
    let passports = file_to_passports(&input);

    assert_eq!(4, passports.len());
    assert_eq!(
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm",
        passports[0]
    );
    assert_eq!(
        "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in",
        passports[3]
    );
}
