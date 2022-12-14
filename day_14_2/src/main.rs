use eyre::Result;
use std::collections::HashSet;

fn main() -> Result<()> {
    let rock = utils::get_input(14)?
        .lines()
        .flat_map(|l| {
            l.split(" -> ")
                .map(|xy| {
                    let mut xy = xy.split(',');
                    let x = xy.next().unwrap().parse::<u32>().unwrap();
                    let y = xy.next().unwrap().parse::<u32>().unwrap();
                    (x, y)
                })
                .collect::<Vec<(u32, u32)>>()
                .windows(2)
                .flat_map(|w| {
                    if w[0].0 != w[1].0 {
                        ((w[0].0.min(w[1].0))..=(w[0].0.max(w[1].0)))
                            .map(|x| (x, w[0].1))
                            .collect::<Vec<(u32, u32)>>()
                    } else {
                        ((w[0].1.min(w[1].1))..=(w[0].1.max(w[1].1)))
                            .map(|y| (w[0].0, y))
                            .collect::<Vec<(u32, u32)>>()
                    }
                })
                .collect::<Vec<(u32, u32)>>()
        })
        .collect::<HashSet<(u32, u32)>>();
    let max_y = *rock.iter().map(|(_, y)| y).max().unwrap();

    let mut sand = HashSet::new();

    'o: loop {
        let mut s = (500, 0);
        if sand.contains(&s) {
            break 'o;
        }
        'fall: loop {
            if can_fall(s.0, s.1 + 1, &sand, &rock, max_y + 2) {
                s.1 += 1;
            } else if can_fall(s.0 - 1, s.1 + 1, &sand, &rock, max_y + 2) {
                s.1 += 1;
                s.0 -= 1;
            } else if can_fall(s.0 + 1, s.1 + 1, &sand, &rock, max_y + 2) {
                s.1 += 1;
                s.0 += 1;
            } else {
                break 'fall;
            }
        }
        sand.insert(s);
    }

    println!("{}", sand.len());

    Ok(())
}

fn can_fall(
    x: u32,
    y: u32,
    sand: &HashSet<(u32, u32)>,
    rock: &HashSet<(u32, u32)>,
    floor: u32,
) -> bool {
    y < floor && !sand.contains(&(x, y)) && !rock.contains(&(x, y))
}
