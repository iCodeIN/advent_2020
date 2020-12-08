#[allow(unused_imports)]
use indoc::indoc;
use std::collections::HashSet;
use std::fs;

fn main() {
    println!("Part One: {}", part_one());
}

fn part_one() -> usize {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let group_answers: Vec<String> = join_lines_between_blank_lines(&input);
    let scores = calculate_scores_per_group(group_answers);
    scores.into_iter().sum()
}

fn calculate_scores_per_group(group_answers: Vec<String>) -> Vec<usize> {
    group_answers
        .into_iter()
        .map(|group| {
            let unique: HashSet<char> = group.chars().collect();
            unique.len()
        })
        .collect()
}

fn join_lines_between_blank_lines(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .map(|l| l.chars().filter(|c| !c.is_whitespace()).collect())
        .collect()
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
fn it_calculates() {
    let answers = calculate_scores_per_group(vec![
        "abc".to_string(),
        "abc".to_string(),
        "abac".to_string(),
        "aaaa".to_string(),
        "b".to_string(),
    ]);

    assert_eq!(answers, vec![3, 3, 3, 1, 1]);
}
