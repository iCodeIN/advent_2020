use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

lazy_static! {
    static ref BAGS_REGEX: Regex = Regex::new(r"(\w* \w*) bags?").unwrap();
}

fn main() {
    println!("Part One: {}", part_one());
}

fn part_one() -> usize {
    let lines = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let bags: Vec<Vec<&str>> = lines.lines().map(|l| find_bags_from_string(l)).collect();
    let bag_parents = discover_bag_parents(bags);
    find_possible_parents_for_given_bag("shiny gold", bag_parents).len()
}

fn find_possible_parents_for_given_bag<'a>(
    bag: &'a str,
    parents: HashMap<&'a str, HashSet<&'a str>>,
) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = HashSet::new();

    let mut bags_to_check: HashSet<&str> = HashSet::new();
    bags_to_check.insert(bag);

    while !bags_to_check.is_empty() {
        let bag_in_question: &str = &bags_to_check.iter().next().unwrap_or(&"");
        bags_to_check.remove(&bag_in_question);

        let default = HashSet::new();
        let bag_parents = parents.get(bag_in_question).unwrap_or(&default);
        for parent in bag_parents.iter() {
            if !result.contains(parent) {
                bags_to_check.insert(&parent);
            }
            result.insert(parent);
        }
    }

    result
}

fn discover_bag_parents(input: Vec<Vec<&str>>) -> HashMap<&str, HashSet<&str>> {
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

fn find_bags_from_string(input: &str) -> Vec<&str> {
    BAGS_REGEX
        .captures_iter(input)
        .into_iter()
        .map(|c| c.get(1).map_or("", |m| m.as_str()))
        .collect()
}

#[test]
fn can_find_bags_from_string() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
    assert_eq!(
        vec!["light red", "bright white", "muted yellow"],
        find_bags_from_string(input)
    );
}

#[test]
fn can_discover_bag_parents() {
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

    assert_eq!(expected, discover_bag_parents(input))
}

#[test]
fn can_find_possible_parents_for_given_bag() {
    let mut parents: HashMap<&str, HashSet<&str>> = HashMap::new();
    parents.insert(
        "bright white",
        HashSet::from_iter(vec!["light red", "dark orange"]),
    );
    parents.insert(
        "muted yellow",
        HashSet::from_iter(vec!["light red", "dark orange"]),
    );
    parents.insert("shiny gold", HashSet::from_iter(vec!["bright white"]));
    let expected = HashSet::from_iter(vec!["bright white", "light red", "dark orange"]);

    assert_eq!(
        expected,
        find_possible_parents_for_given_bag("shiny gold", parents)
    )
}
