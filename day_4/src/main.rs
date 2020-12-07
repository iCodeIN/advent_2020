use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    println!("Day One: {}", part_one());
    println!("Day Two: {}", part_two());
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

fn part_two() -> usize {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let passports: Vec<String> = file_to_passports(&input);
    let passports: Vec<Passport> = passports
        .iter()
        .map(|p| Passport::new(p.to_string()))
        .filter_map(|x| x)
        .filter(|p| p.is_valid())
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

    fn is_valid(&self) -> bool {
        is_valid_birth_year(self.birth_year)
            && is_valid_issue_year(self.issue_year)
            && is_valid_expiration_year(self.expiration_year)
            && is_valid_height(&*self.height)
            && is_valid_hair(&*self.hair_colour)
            && is_valid_eye_colour(&*self.eye_colour)
            && is_valid_passport_number(&*self.passport_id)
    }
}

fn is_valid_birth_year(year: i32) -> bool {
    year >= 1920 && year <= 2002
}

fn is_valid_issue_year(year: i32) -> bool {
    year >= 2010 && year <= 2020
}

fn is_valid_expiration_year(year: i32) -> bool {
    year >= 2020 && year <= 2030
}

fn is_valid_height(height: &str) -> bool {
    if height.ends_with("cm") && height.len() == 5 {
        let cms: i32 = match height[0..3].parse() {
            Ok(val) => val,
            Err(_) => 0,
        };
        return cms >= 150 && cms <= 193;
    }
    if height.ends_with("in") && height.len() == 4 {
        let inches: i32 = match height[0..2].parse() {
            Ok(val) => val,
            Err(_) => 0,
        };
        return inches >= 59 && inches <= 76;
    }
    false
}

fn is_valid_hair(hair: &str) -> bool {
    Regex::new(r"^#[a-f0-9]{6}$").unwrap().is_match(hair)
}

const VALID_EYE_COLOURS: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
fn is_valid_eye_colour(eye: &str) -> bool {
    VALID_EYE_COLOURS.contains(&eye)
}

fn is_valid_passport_number(passport_number: &str) -> bool {
    Regex::new(r"^[0-9]{9}$").unwrap().is_match(passport_number)
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

#[test]
fn can_check_birth_year_is_valid() {
    // four digits; at least 1920 and at most 2002.
    assert!(!is_valid_birth_year(1919));
    assert!(is_valid_birth_year(1920));
    assert!(is_valid_birth_year(2002));
    assert!(!is_valid_birth_year(2003));
}

#[test]
fn can_check_issue_year() {
    // four digits; at least 2010 and at most 2020.
    assert!(!is_valid_issue_year(2009));
    assert!(is_valid_issue_year(2010));
    assert!(is_valid_issue_year(2020));
    assert!(!is_valid_issue_year(2021));
}

#[test]
fn can_check_expiration_year() {
    // four digits; at least 2020 and at most 2030.
    assert!(!is_valid_expiration_year(2019));
    assert!(is_valid_expiration_year(2020));
    assert!(is_valid_expiration_year(2030));
    assert!(!is_valid_expiration_year(2031));
}

#[test]
fn can_check_height() {
    // a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    assert!(!is_valid_height("149cm"));
    assert!(is_valid_height("150cm"));
    assert!(is_valid_height("193cm"));
    assert!(!is_valid_height("194cm"));

    // If in, the number must be at least 59 and at most 76.
    assert!(!is_valid_height("58in"));
    assert!(is_valid_height("59in"));
    assert!(is_valid_height("76in"));
    assert!(!is_valid_height("77in"));
}

#[test]
fn can_check_hair_colour() {
    // a # followed by exactly six characters 0-9 or a-f.
    assert!(is_valid_hair("#123abc"));
    assert!(!is_valid_hair("#123abz"));
    assert!(!is_valid_hair("123abc"));
    assert!(!is_valid_hair("#123abcd"));
}

#[test]
fn can_check_eye_colour() {
    // exactly one of: amb blu brn gry grn hzl oth
    assert!(is_valid_eye_colour("amb"));
    assert!(is_valid_eye_colour("blu"));
    assert!(is_valid_eye_colour("brn"));
    assert!(is_valid_eye_colour("gry"));
    assert!(is_valid_eye_colour("grn"));
    assert!(is_valid_eye_colour("hzl"));
    assert!(is_valid_eye_colour("oth"));
    assert!(!is_valid_eye_colour("zzz"));
}

#[test]
fn cat_check_passport_number() {
    // a nine-digit number, including leading zeroes.
    assert!(is_valid_passport_number("000000001"));
    assert!(!is_valid_passport_number("0123456789"));
}

#[test]
fn can_check_passport_is_valid() {
    let p = Passport {
        birth_year: 1937,
        issue_year: 2017,
        expiration_year: 2020,
        height: "183cm".to_string(),
        hair_colour: "#fffffd".to_string(),
        eye_colour: "gry".to_string(),
        passport_id: "860033327".to_string(),
    };
    assert!(p.is_valid());

    let p2 = Passport {
        birth_year: 10,
        issue_year: 2017,
        expiration_year: 2020,
        height: "183cm".to_string(),
        hair_colour: "#fffffd".to_string(),
        eye_colour: "gry".to_string(),
        passport_id: "860033327".to_string(),
    };
    assert!(!p2.is_valid());
}
