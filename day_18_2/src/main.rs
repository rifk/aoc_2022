use eyre::Result;
use std::collections::HashSet;

fn main() -> Result<()> {
    let (drops, max, min) = {
        let mut max = (i32::MIN, i32::MIN, i32::MIN);
        let mut min = (i32::MAX, i32::MAX, i32::MAX);
        let d = utils::get_input(18)?
            .lines()
            .map(|l| {
                let mut coord = l.split(',');
                let xyz = (
                    coord.next().unwrap().parse::<i32>().unwrap(),
                    coord.next().unwrap().parse::<i32>().unwrap(),
                    coord.next().unwrap().parse::<i32>().unwrap(),
                );

                max.0 = max.0.max(xyz.0);
                max.1 = max.1.max(xyz.1);
                max.2 = max.2.max(xyz.2);

                min.0 = min.0.min(xyz.0);
                min.1 = min.1.min(xyz.1);
                min.2 = min.2.min(xyz.2);

                xyz
            })
            .collect::<HashSet<(i32, i32, i32)>>();
        (d, max, min)
    };

    let inner = calc_inner(&drops, &max, &min);

    let exclude: HashSet<_> = drops.union(&inner).collect();

    println!(
        "{}",
        drops
            .iter()
            .map(|(x, y, z)| {
                let mut uncovered = 0;
                if !exclude.contains(&(x - 1, *y, *z)) {
                    uncovered += 1;
                }
                if !exclude.contains(&(x + 1, *y, *z)) {
                    uncovered += 1;
                }
                if !exclude.contains(&(*x, y - 1, *z)) {
                    uncovered += 1;
                }
                if !exclude.contains(&(*x, y + 1, *z)) {
                    uncovered += 1;
                }
                if !exclude.contains(&(*x, *y, z - 1)) {
                    uncovered += 1;
                }
                if !exclude.contains(&(*x, *y, z + 1)) {
                    uncovered += 1;
                }
                uncovered
            })
            .sum::<i32>()
    );

    Ok(())
}

fn calc_inner(
    drops: &HashSet<(i32, i32, i32)>,
    max: &(i32, i32, i32),
    min: &(i32, i32, i32),
) -> HashSet<(i32, i32, i32)> {
    let mut inner = HashSet::new();
    for (x, y, z) in drops {
        'check: for (x, y, z) in vec![
            (x - 1, *y, *z),
            (x + 1, *y, *z),
            (*x, y - 1, *z),
            (*x, y + 1, *z),
            (*x, *y, z - 1),
            (*x, *y, z + 1),
        ]
        .into_iter()
        {
            if drops.contains(&(x, y, z)) || is_outer(x, y, z, max, min) {
                continue;
            }

            let mut seen = HashSet::new();
            let mut search = vec![];
            search.push((x, y, z));
            seen.insert((x, y, z));

            while !search.is_empty() {
                let mut check = vec![];
                std::mem::swap(&mut check, &mut search);
                for (x, y, z) in check {
                    for (x, y, z) in vec![
                        (x - 1, y, z),
                        (x + 1, y, z),
                        (x, y - 1, z),
                        (x, y + 1, z),
                        (x, y, z - 1),
                        (x, y, z + 1),
                    ]
                    .into_iter()
                    {
                        if is_outer(x, y, z, max, min) {
                            continue 'check;
                        }
                        if seen.contains(&(x, y, z)) || drops.contains(&(x, y, z)) {
                            continue;
                        }
                        search.push((x, y, z));
                        seen.insert((x, y, z));
                    }
                }
            }

            inner.insert((x, y, z));
        }
    }
    inner
}

fn is_outer(x: i32, y: i32, z: i32, max: &(i32, i32, i32), min: &(i32, i32, i32)) -> bool {
    x <= min.0 || y <= min.1 || z <= min.2 || x >= max.0 || y >= max.1 || z >= max.2
}
