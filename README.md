# Advent of Code 2022
My solutions for advent of code 2022 in Rust.
https://adventofcode.com/2022 

## Running

### Input from website
Take the session cookie from browser (using network developer tool and look at request header `cookie: session=<session_value>`.
Set the `AOC_SESSION` env var to the session value then run:
```
cargo run -p day_i_j
```
where `i` is aoc day and `j` is `1` or `2` for first or second daily task.

### Input from file
Create file containing input then run:
```
cargo run -p day_i_j -- <input_file>
```


