# Learnings from Advent of Code 2020 in Rust

# Exercise 1

- The `Result` object is pretty cool. The language forces you to handle error cases
- It appears you can ignore them by adding a `?` to the end of your statement like `let first_number: i32 = line.parse()?;`
- Semicolons have some magical meaning. Don't add it to the end of your method
- Rust likes snake_case
- if statements do not have braces
- `Ok(())` is what you use to say everything is fine. You can do early returns in loops with this

# Exercise 2

Part 1

- String and &str are different. The first is mutable and has more methods
- Numbers can be i32 and usize. You have to type convert to perform calculations on them (no idea what's going but `as usize` seems to convert)
- If you reference a local variable you need to put an ampersand before it (&my_var)

Part 2

- Getting substrings is hard to get started with. You are exposed to unicode weirdness
- You can return using `Result<i32>` the `Result<()>` I was using before was for undefined
- You cannot chain methods together so easily when you need to use an ampersand to use them first

Refactoring after looking at others solutions

- You can split on whitespace using `split_whitespace()`
- Rust has destructuing like `let (range_min, range_max) = (ranges[0], ranges[1]);`
- You can index into string by making a char vector first `"abc".chars().collect()`
- Raise errors with `panic!("impossible")`

## Others

- https://github.com/XorJoep/AoC_2020
- https://github.com/vildapavlicek/AoC2020/
- https://github.com/pedantic79/advent-of-code-2020
- https://github.com/kvrhdn/Advent-of-Code
- https://github.com/CapnRat/adventofcode2020
