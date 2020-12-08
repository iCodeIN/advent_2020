#[allow(unused_imports)]
use indoc::indoc;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> usize {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let group_answers: Vec<String> = join_lines_between_blank_lines(&input);
    group_answers
        .into_iter()
        .map(|group| unique_letters(&group))
        .sum()
}

fn part_two() -> usize {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let group_answers: Vec<String> = join_lines_between_blank_lines(&input);
    group_answers
        .into_iter()
        .map(|group| repeated_letters(&group).len())
        .sum()
}

fn unique_letters(input: &str) -> usize {
    let unique: HashSet<char> = input.chars().collect();
    unique.len()
}

fn repeated_letters(input: &str) -> Vec<char> {
    let mut frequency: HashMap<char, u32> = HashMap::new();
    for letter in input.chars() {
        *frequency.entry(letter).or_insert(0) += 1;
    }

    let repeated_only: HashMap<char, u32> = frequency.into_iter().filter(|&(_, v)| v > 1).collect();
    repeated_only.keys().cloned().collect()
}

fn join_lines_between_blank_lines(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .map(|l| l.chars().filter(|c| !c.is_whitespace()).collect())
        .collect()
}

#[test]
fn can_work_out_unique() {
    assert_eq!(3, unique_letters("abc"));
    assert_eq!(3, unique_letters("abac"));
    assert_eq!(1, unique_letters("aaaa"));
    assert_eq!(1, unique_letters("b"));
}

#[test]
fn can_work_out_repeated() {
    assert_eq!(vec!['a'], repeated_letters("aabc"));
}

#[test]
fn it_tidies_up_input() {
    let input = indoc! {"abc

        a
        b
        c
        
        ab
        ac
        
        a
        a
        a
        a
        
        b"};
    let lines = join_lines_between_blank_lines(&input);

    assert_eq!(lines, vec!["abc", "abc", "abac", "aaaa", "b"]);
}
