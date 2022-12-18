use eyre::Result;
use std::collections::{HashMap, HashSet};

static Y_2000000: i32 = 2_000_000;

fn main() -> Result<()> {
    let sensors = utils::get_input(15)?
        .lines()
        .map(line_to_sensor_beacon)
        .collect::<HashMap<(i32, i32), (i32, i32)>>();
    let b_x_2000000 = sensors
        .values()
        .filter_map(|(x, y)| if *y == Y_2000000 { Some(x) } else { None })
        .copied()
        .collect::<HashSet<i32>>();

    println!(
        "{}",
        sensors
            .into_iter()
            .flat_map(|((s_x, s_y), (b_x, b_y))| {
                let d = (s_x - b_x).abs() + (s_y - b_y).abs();
                if (s_y - Y_2000000).abs() <= d {
                    let x_d = d - (s_y - Y_2000000).abs();
                    ((s_x - x_d)..=(s_x + x_d))
                        .into_iter()
                        .filter(|x| !b_x_2000000.contains(x))
                        .collect::<Vec<i32>>()
                } else {
                    vec![]
                }
            })
            .collect::<HashSet<i32>>()
            .len()
    );

    Ok(())
}

fn line_to_sensor_beacon(l: &str) -> ((i32, i32), (i32, i32)) {
    let l = l.strip_prefix("Sensor at x=").unwrap();
    let (s_x, l) = l.split_once(", y=").unwrap();
    let (s_y, l) = l.split_once(": closest beacon is at x=").unwrap();
    let (b_x, b_y) = l.split_once(", y=").unwrap();
    (
        (s_x.parse::<i32>().unwrap(), s_y.parse::<i32>().unwrap()),
        (b_x.parse::<i32>().unwrap(), b_y.parse::<i32>().unwrap()),
    )
}
