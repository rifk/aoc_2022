use eyre::Result;
use std::collections::HashSet;

fn main() -> Result<()> {
    let total = utils::get_input(3)?
        .lines()
        .map(line_to_priority)
        .sum::<i32>();

    println!("{:?}", total);

    Ok(())
}

fn line_to_priority(line: &str) -> i32 {
    let char_set = line[0..line.len() / 2].chars().collect::<HashSet<char>>();
    let duplicate = line[line.len() / 2..]
        .chars()
        .find(|c| char_set.contains(c))
        .unwrap();
    match duplicate {
        'a'..='z' => 1 + (duplicate as i32 - 'a' as i32),
        'A'..='Z' => 27 + (duplicate as i32 - 'A' as i32),
        other => panic!("unexpected char: {}", other),
    }
}
