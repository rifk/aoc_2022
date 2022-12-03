use eyre::Result;
use std::collections::HashSet;

fn main() -> Result<()> {
    let total = utils::get_input(3)?
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(group_to_priority)
        .sum::<i32>();

    println!("{:?}", total);

    Ok(())
}

fn group_to_priority(group: &[&str]) -> i32 {
    let badge = group
        .iter()
        .map(|l| l.chars().collect::<HashSet<char>>())
        .reduce(|acc, item| {
            acc.intersection(&item)
                .map(|c| c.clone())
                .collect::<HashSet<char>>()
        })
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    match badge {
        'a'..='z' => 1 + (badge as i32 - 'a' as i32),
        'A'..='Z' => 27 + (badge as i32 - 'A' as i32),
        other => panic!("unexpected char: {}", other),
    }
}
