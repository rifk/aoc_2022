use eyre::Result;
use std::collections::HashSet;

static MAX: i32 = 4_000_000;

type SensorBeaconDistance = ((i32, i32), (i32, i32), i32);

fn main() -> Result<()> {
    let sensors = utils::get_input(15)?
        .lines()
        .map(line_to_sensor_beacon)
        .map(|((s_x, s_y), (b_x, b_y))| {
            (
                (s_x, s_y),
                (b_x, b_y),
                (s_x - b_x).abs() + (s_y - b_y).abs(),
            )
        })
        .collect::<Vec<SensorBeaconDistance>>();
    let beacons = sensors
        .iter()
        .map(|(_, b, _)| b)
        .copied()
        .collect::<HashSet<(i32, i32)>>();

    println!(
        "{:?}",
        (0..=MAX)
            .into_iter()
            .find_map(|y| find_undetected(y, &sensors, &beacons).map(|x| (x, y)))
    );

    Ok(())
}

/// Get undetected x value between 0,MAX; fo a set y
fn find_undetected(
    y: i32,
    sensors: &[SensorBeaconDistance],
    beacons: &HashSet<(i32, i32)>,
) -> Option<i32> {
    println!("{y}");
    // get all undetected ranges
    let undetected = sensors
        .iter()
        .filter_map(|((s_x, s_y), _, d)| {
            let d_x = d - (s_y - y).abs();
            // sensor reaches y coord we care about
            if d_x >= 0 {
                let l = s_x - d_x;
                let r = s_x + d_x;
                if r < 0 || l > MAX {
                    // sensor range outside 0,MAX
                    None
                } else if l <= 0 {
                    // detection start before or at 0
                    if r >= MAX {
                        // full range detected
                        Some(vec![])
                    } else {
                        Some(vec![(r + 1, MAX)])
                    }
                } else if r >= MAX {
                    // detection ends after or at MAX
                    if l <= 0 {
                        // full range detected
                        Some(vec![])
                    } else {
                        Some(vec![(0, l - 1)])
                    }
                } else {
                    //detection splits range
                    Some(vec![(0, l - 1), (r + 1, MAX)])
                }
            }
            // doesnt reach y coord
            else {
                None
            }
        })
        .reduce(|v1, v2| {
            if v1.is_empty() {
                v1
            } else if v2.is_empty() {
                v2
            } else {
                let mut overlap = vec![];
                let mut i1 = v1.into_iter();
                let mut i2 = v2.into_iter();

                let mut range1 = i1.next();
                let mut range2 = i2.next();

                while let (Some((l1, r1)), Some((l2, r2))) = (range1, range2) {
                    if l1 <= r2 || l2 <= r1 {
                        overlap.push((l1.max(l2), r1.min(r2)));
                    }

                    if r1 < r2 {
                        range1 = i1.next();
                    } else {
                        range2 = i2.next();
                    }
                }

                overlap
            }
        })
        .unwrap();
    let undetected = undetected
        .into_iter()
        .filter_map(|(f, l)| {
            let v = (f..=l)
                .into_iter()
                .filter(|x| !beacons.contains(&(*x, y)))
                .collect::<Vec<i32>>();
            assert!(v.len() < 2);
            v.first().copied()
        })
        .collect::<Vec<i32>>();

    assert!(undetected.len() < 2);
    undetected.first().copied()
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
