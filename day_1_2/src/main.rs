use eyre::Result;

fn main() -> Result<()> {
    let mut sums = utils::get_input(1)?
        .lines()
        .collect::<Vec<&str>>()
        .split(|l| l.is_empty())
        .map(|inv| inv.iter().map(|v| v.parse::<i64>().unwrap()).sum::<i64>())
        .collect::<Vec<i64>>();
    sums.sort();
    sums.reverse();

    println!("{:?}", sums.iter().take(3).sum::<i64>());

    Ok(())
}
