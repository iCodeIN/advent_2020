#[allow(unused_imports)]
use indoc::indoc;
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
    let group_answers: Vec<Vec<&str>> = split_on_blank_lines(&input);
    group_answers
        .into_iter()
        .map(|group| common_answers(group).len())
        .sum()
}

fn unique_letters(input: &str) -> usize {
    let unique: HashSet<char> = input.chars().collect();
    unique.len()
}

fn common_answers(input: Vec<&str>) -> Vec<char> {
    let first = input.first().unwrap_or(&"");
    let rest: &[&str] = &input[1..];
    first
        .chars()
        .filter(|letter| rest.into_iter().all(|a| a.contains(*letter)))
        .collect()
}

fn join_lines_between_blank_lines(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .map(|l| l.chars().filter(|c| !c.is_whitespace()).collect())
        .collect()
}

fn split_on_blank_lines(input: &str) -> Vec<Vec<&str>> {
    input
        .split("\n\n")
        .map(|l| l.split("\n").collect())
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

#[test]
fn it_can_find_common_answers() {
    let empty_array: Vec<char> = vec![];
    assert_eq!(vec!['a', 'b', 'c'], common_answers(vec!["abc"]));
    assert_eq!(empty_array, common_answers(vec!["a", "b", "c"]));
    assert_eq!(vec!['a'], common_answers(vec!["ab", "ac"]));
    assert_eq!(vec!['a'], common_answers(vec!["a", "a", "a", "a"]));
    assert_eq!(vec!['b'], common_answers(vec!["b"]));
}

#[test]
fn it_tidies_up_input_preserving_groups() {
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
    let lines = split_on_blank_lines(&input);

    assert_eq!(
        lines,
        vec![
            vec!["abc"],
            vec!["a", "b", "c"],
            vec!["ab", "ac"],
            vec!["a", "a", "a", "a"],
            vec!["b"]
        ]
    );
}

// Part One: 6549
// Part Two: 3466
