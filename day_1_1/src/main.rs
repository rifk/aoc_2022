use eyre::Result;

fn main() -> Result<()> {
    let max = utils::get_input(1)?
        .lines()
        .collect::<Vec<&str>>()
        .split(|l| l.is_empty())
        .map(|inv| inv.iter().map(|v| v.parse::<i64>().unwrap()).sum::<i64>())
        .max();

    println!("{:?}", max);

    Ok(())
}
