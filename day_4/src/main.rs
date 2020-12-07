use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    println!("Day One: {}", part_one());
}

fn part_one() -> usize {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let passports: Vec<String> = file_to_passports(&input);
    let passports: Vec<Passport> = passports
        .iter()
        .map(|p| Passport::new(p.to_string()))
        .filter_map(|x| x)
        .collect();

    passports.len()
}

fn file_to_passports(input: &str) -> Vec<String> {
    let mut passports: Vec<String> = Vec::new();

    let mut current_passport: Vec<&str> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            passports.push(current_passport.join(" "));
            current_passport = Vec::new();
        } else {
            current_passport.push(line.trim());
        }
    }
    passports.push(current_passport.join(" "));

    passports
}

#[derive(Debug, PartialEq)]
struct Passport {
    birth_year: i32,
    issue_year: i32,
    expiration_year: i32,
    height: String,
    hair_colour: String,
    eye_colour: String,
    passport_id: String,
}

impl Passport {
    fn new(line: String) -> Option<Passport> {
        let mut passport_bits: HashMap<&str, &str> = HashMap::new();
        let tokens = line.split(' ');
        for token in tokens {
            let token: Vec<&str> = token.split(':').collect();
            passport_bits.insert(token[0], token[1]);
        }

        let required_fields: HashSet<&'static str> =
            vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .into_iter()
                .collect();
        let available_fields: HashSet<&str> = passport_bits.keys().copied().collect();
        if required_fields.intersection(&available_fields).count() != 7 {
            return None;
        }

        Some(Passport {
            birth_year: match passport_bits.get("byr") {
                Some(val) => val.parse().expect("not a number"),
                None => panic!("field missing birth_year"),
            },
            issue_year: match passport_bits.get("iyr") {
                Some(val) => val.parse().expect("not a number"),
                None => panic!("field missing issue_year"),
            },
            expiration_year: match passport_bits.get("eyr") {
                Some(val) => val.parse().expect("not a number"),
                None => panic!("field missing expiration_year"),
            },
            height: match passport_bits.get("hgt") {
                Some(val) => val.to_string(),
                None => panic!("field missing height"),
            },
            hair_colour: match passport_bits.get("hcl") {
                Some(val) => val.to_string(),
                None => panic!("field missing hair_colour"),
            },
            eye_colour: match passport_bits.get("ecl") {
                Some(val) => val.to_string(),
                None => panic!("field missing eye_colour"),
            },
            passport_id: match passport_bits.get("pid") {
                Some(val) => val.to_string(),
                None => panic!("field missing passport_id"),
            },
        })
    }
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

#[test]
fn can_make_passport_line_into_passport() {
    let p = Passport {
        birth_year: 1937,
        issue_year: 2017,
        expiration_year: 2020,
        height: "183cm".to_string(),
        hair_colour: "#fffffd".to_string(),
        eye_colour: "gry".to_string(),
        passport_id: "860033327".to_string(),
    };

    let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";

    assert_eq!(p, Passport::new(input.to_string()).expect(""));
}

#[test]
fn can_reject_invalid_passports() {
    let input = "a:1";
    assert!(Passport::new(input.to_string()).is_none());
}
