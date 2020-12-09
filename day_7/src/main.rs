use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

lazy_static! {
    static ref BAGS_REGEX: Regex = Regex::new(r"(\w* \w*) bags?").unwrap();
}

fn main() {
    // println!("{}", find_bags());
}

fn find_bags(input: &str) -> Vec<&str> {
    BAGS_REGEX
        .captures_iter(input)
        .into_iter()
        .map(|c| c.get(1).map_or("", |m| m.as_str()))
        .collect()
}

fn find_bag_parents(input: Vec<Vec<&str>>) -> HashMap<&str, HashSet<&str>> {
    let mut result: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input {
        let parent = line.first().unwrap_or(&"");
        let children: &[&str] = &line[1..];

        for child in children {
            let known_kids = result.entry(child).or_insert(HashSet::new());
            known_kids.insert(parent);
        }
    }

    result
}

#[test]
fn can_find_bags() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
    assert_eq!(
        vec!["light red", "bright white", "muted yellow"],
        find_bags(input)
    );
}

#[test]
fn can_find_bag_parents() {
    let input = vec![
        vec!["light red", "bright white", "muted yellow"],
        vec!["dark orange", "bright white", "muted yellow"],
        vec!["bright white", "shiny gold"],
    ];
    let mut expected: HashMap<&str, HashSet<&str>> = HashMap::new();
    expected.insert(
        "bright white",
        HashSet::from_iter(vec!["light red", "dark orange"]),
    );
    expected.insert(
        "muted yellow",
        HashSet::from_iter(vec!["light red", "dark orange"]),
    );
    expected.insert("shiny gold", HashSet::from_iter(vec!["bright white"]));

    assert_eq!(expected, find_bag_parents(input))
}
