use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref BAGS_REGEX: Regex = Regex::new(r"(\w* \w*) bags?").unwrap();
}

fn main() {
    // println!("{}", find_bags());
}

fn find_bags(input: &str) -> Vec<&str> {
    let a: Vec<_> = BAGS_REGEX
        .captures_iter(input)
        .into_iter()
        .map(|c| c.get(1).map_or("", |m| m.as_str()))
        .collect();

    a
}

#[test]
fn can_find_bags() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
    assert_eq!(
        vec!["light red", "bright white", "muted yellow"],
        find_bags(input)
    );
}
