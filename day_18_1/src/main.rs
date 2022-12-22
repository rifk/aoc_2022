use eyre::Result;
use std::collections::HashSet;

fn main() -> Result<()> {
    let drops = utils::get_input(18)?
        .lines()
        .map(|l| {
            let mut coord = l.split(',');
            (
                coord.next().unwrap().parse::<i32>().unwrap(),
                coord.next().unwrap().parse::<i32>().unwrap(),
                coord.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<HashSet<(i32, i32, i32)>>();

    println!(
        "{}",
        drops
            .iter()
            .map(|(x, y, z)| {
                let mut uncovered = 0;
                if !drops.contains(&(x - 1, *y, *z)) {
                    uncovered += 1;
                }
                if !drops.contains(&(x + 1, *y, *z)) {
                    uncovered += 1;
                }
                if !drops.contains(&(*x, y - 1, *z)) {
                    uncovered += 1;
                }
                if !drops.contains(&(*x, y + 1, *z)) {
                    uncovered += 1;
                }
                if !drops.contains(&(*x, *y, z - 1)) {
                    uncovered += 1;
                }
                if !drops.contains(&(*x, *y, z + 1)) {
                    uncovered += 1;
                }
                uncovered
            })
            .sum::<i32>()
    );

    Ok(())
}
