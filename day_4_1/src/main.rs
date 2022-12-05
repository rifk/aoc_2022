use eyre::Result;

fn main() -> Result<()> {
    let total = utils::get_input(4)?
        .lines()
        .filter(|l| is_fully_contained(l))
        .count();

    println!("{:?}", total);

    Ok(())
}

fn is_fully_contained(line: &str) -> bool {
    let areas = line
        .split(',')
        .map(|area| {
            let v = area
                .split('-')
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            v[0]..=v[1]
        })
        .collect::<Vec<_>>();
    (areas[0].start() <= areas[1].start() && areas[0].end() >= areas[1].end())
        || (areas[1].start() <= areas[0].start() && areas[1].end() >= areas[0].end())
}
