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
