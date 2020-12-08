use std::collections::HashSet;
use std::fs;

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> i32 {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");

    let mut seats: Vec<(i32, i32, i32)> = input.lines().map(|s| place_for_ticket(s)).collect();
    seats.sort_by(|a, b| b.2.cmp(&a.2));

    seats[0].2
}

fn part_two() -> i32 {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");

    let mut seats: Vec<(i32, i32, i32)> = input.lines().map(|s| place_for_ticket(s)).collect();
    seats.sort_by(|a, b| b.2.cmp(&a.2));

    let highest_seat_id = seats[0].2;
    let lowest_seat_id: i32 = match seats.last() {
        Some(v) => v.2,
        None => 0,
    };
    let seat_number_range = lowest_seat_id..=highest_seat_id;

    let mut seen: HashSet<i32> = HashSet::new();
    for (_, _, seat_id) in seats {
        seen.insert(seat_id);
    }

    for seat_id in seat_number_range {
        if !seen.contains(&seat_id) {
            return seat_id;
        }
    }

    0
}

fn place_for_ticket(input: &str) -> (i32, i32, i32) {
    let (row_min, row_max): (i32, i32) = find_upper_or_lower_range(&input[0..1], 0, 127);
    let (row_min, row_max): (i32, i32) = find_upper_or_lower_range(&input[1..2], row_min, row_max);
    let (row_min, row_max): (i32, i32) = find_upper_or_lower_range(&input[2..3], row_min, row_max);
    let (row_min, row_max): (i32, i32) = find_upper_or_lower_range(&input[3..4], row_min, row_max);
    let (row_min, row_max): (i32, i32) = find_upper_or_lower_range(&input[4..5], row_min, row_max);
    let (row_min, row_max): (i32, i32) = find_upper_or_lower_range(&input[5..6], row_min, row_max);
    let (row, _): (i32, i32) = find_upper_or_lower_range(&input[6..7], row_min, row_max);

    let (col_min, col_max): (i32, i32) = find_upper_or_lower_range(&input[7..8], 0, 7);
    let (col_min, col_max): (i32, i32) = find_upper_or_lower_range(&input[8..9], col_min, col_max);
    let (col, _): (i32, i32) = find_upper_or_lower_range(&input[9..10], col_min, col_max);

    let seat_id = (row * 8) + col;

    (row, col, seat_id)
}

fn find_upper_or_lower_range(input: &str, min: i32, max: i32) -> (i32, i32) {
    if input == "F" || input == "L" {
        (min, (((max - min) / 2) + min))
    } else {
        ((((max - min) / 2) + min) + 1, max)
    }
}

#[test]
fn can_get_seat_details() {
    assert_eq!((44, 5, 357), place_for_ticket("FBFBBFFRLR"));
    assert_eq!((70, 7, 567), place_for_ticket("BFFFBBFRRR"));
    assert_eq!((14, 7, 119), place_for_ticket("FFFBBBFRRR"));
    assert_eq!((102, 4, 820), place_for_ticket("BBFFBBFRLL"));
}

#[test]
fn can_process_front_back() {
    assert_eq!((0, 63), find_upper_or_lower_range("F", 0, 127));
    assert_eq!((32, 63), find_upper_or_lower_range("B", 0, 63));
    assert_eq!((32, 47), find_upper_or_lower_range("F", 32, 63));
    assert_eq!((40, 47), find_upper_or_lower_range("B", 32, 47));
    assert_eq!((44, 47), find_upper_or_lower_range("B", 40, 47));
    assert_eq!((44, 45), find_upper_or_lower_range("F", 44, 47));
    assert_eq!((44, 44), find_upper_or_lower_range("F", 44, 45));

    assert_eq!((4, 7), find_upper_or_lower_range("R", 0, 7));
    assert_eq!((4, 5), find_upper_or_lower_range("L", 4, 7));
    assert_eq!((5, 5), find_upper_or_lower_range("R", 4, 5));
}
