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

# Reading the Rust book Chapter 1-2

- Functions with an exclaimation point! are calling Macros
- `rustc` is the compiler. You don't need Rust installed to run Rust apps
- Rustaceans!
- `cargo build --release` to make the proper version
- There is an idea of a 'Prelude' which is auto-imported parts of standard libraries: https://doc.rust-lang.org/std/prelude/index.html
- `::` indicates an 'associated function' like `String::new`. Like static methods
- `&` is a reference, It is used for sharing a variable with other bits of code for it to modify. References and variables are immutable by default
- Libraries are at https://crates.io/
- You can get all the docs for your crates by using `cargo doc --open`
- `match` expressions have "arms"
- i32 => signed number, u32 => unsigned number, i64 u64
- In Rust you shadow when converting types

Reading the Rust book Chapter 3

- `mut` makes things mutable.
- You have constants like: `const MAX_POINTS: u32 = 100_000;`
- Shadowing is used when changing type, `mut` won't let you change the type
- `usize` is the system archiectures interger size. 64bit vs 32bit.
- `i32` is a good default as it's faster. `usize` usually comes from using collections
- If a number goes beyond the size of the type then it wraps around from 0 again (don't do this though)
- `f64` for floats (called a double in Java). Use it over `f32` as it gives you more precision
- "Compound types" are non scalar things. Rust has Array and Tuples.
- Tuple: `let tup: (i32, f64, u8) = (500, 6.4, 1);` cannot grow
- Get values by "pattern matching (destructuring)": `let (x, y, z) = tup;`
- You can index into Tuples like `x.0` and `x.1`
- Arrays have fixed length (like Java arrays) - favour Vectors instead
- You specify size with: `let a: [i32; 5] = [1, 2, 3, 4, 5];`
- You can get a repeate value array with: `let a = [3; 5]`
- if you try to index into an array out of bounds - the compiler will catch it (cool!)

# Exercise 3

-

# Adrian recommends

- https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1
- https://github.com/rust-lang/rust-clippy

## Others

- https://github.com/XorJoep/AoC_2020
- https://github.com/vildapavlicek/AoC2020/
- https://github.com/pedantic79/advent-of-code-2020
- https://github.com/kvrhdn/Advent-of-Code
- https://github.com/CapnRat/adventofcode2020
