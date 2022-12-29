use eyre::Result;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Land {
    elves: HashSet<(i32, i32)>,
    priority: VecDeque<Direction>,
}
impl Land {
    fn new(input: String) -> Self {
        let elves = input
            .lines()
            .enumerate()
            .flat_map(|(x, l)| {
                l.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(y, _)| {
                        let x = x as i32;
                        let y = y as i32;
                        (x, y)
                    })
            })
            .collect::<HashSet<(i32, i32)>>();
        Self {
            elves,
            priority: {
                let mut p = VecDeque::new();
                p.push_back(Direction::North);
                p.push_back(Direction::South);
                p.push_back(Direction::West);
                p.push_back(Direction::East);
                p
            },
        }
    }

    fn next_round(&mut self) -> bool {
        let mut moved = false;
        let mut target: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
        'elf: for (x, y) in &self.elves {
            if !self.elves.contains(&(x - 1, y - 1))
                && !self.elves.contains(&(x - 1, *y))
                && !self.elves.contains(&(x - 1, y + 1))
                && !self.elves.contains(&(*x, y - 1))
                && !self.elves.contains(&(*x, y + 1))
                && !self.elves.contains(&(x + 1, y - 1))
                && !self.elves.contains(&(x + 1, *y))
                && !self.elves.contains(&(x + 1, y + 1))
            {
                target.insert((*x, *y), vec![(*x, *y)]);
                continue 'elf;
            }
            moved = true;
            for d in &self.priority {
                match d {
                    Direction::North => {
                        if !self.elves.contains(&(x - 1, y - 1))
                            && !self.elves.contains(&(x - 1, *y))
                            && !self.elves.contains(&(x - 1, y + 1))
                        {
                            target
                                .entry((x - 1, *y))
                                .and_modify(|from| from.push((*x, *y)))
                                .or_insert_with(|| vec![(*x, *y)]);
                            continue 'elf;
                        }
                    }
                    Direction::South => {
                        if !self.elves.contains(&(x + 1, y - 1))
                            && !self.elves.contains(&(x + 1, *y))
                            && !self.elves.contains(&(x + 1, y + 1))
                        {
                            target
                                .entry((x + 1, *y))
                                .and_modify(|from| from.push((*x, *y)))
                                .or_insert_with(|| vec![(*x, *y)]);
                            continue 'elf;
                        }
                    }
                    Direction::West => {
                        if !self.elves.contains(&(x - 1, y - 1))
                            && !self.elves.contains(&(*x, y - 1))
                            && !self.elves.contains(&(x + 1, y - 1))
                        {
                            target
                                .entry((*x, y - 1))
                                .and_modify(|from| from.push((*x, *y)))
                                .or_insert_with(|| vec![(*x, *y)]);
                            continue 'elf;
                        }
                    }
                    Direction::East => {
                        if !self.elves.contains(&(x - 1, y + 1))
                            && !self.elves.contains(&(*x, y + 1))
                            && !self.elves.contains(&(x + 1, y + 1))
                        {
                            target
                                .entry((*x, y + 1))
                                .and_modify(|from| from.push((*x, *y)))
                                .or_insert_with(|| vec![(*x, *y)]);
                            continue 'elf;
                        }
                    }
                }
            }
            target.insert((*x, *y), vec![(*x, *y)]);
        }

        self.elves = target
            .into_iter()
            .flat_map(
                |((x, y), from)| {
                    if from.len() == 1 {
                        vec![(x, y)]
                    } else {
                        from
                    }
                },
            )
            .collect();

        self.priority.rotate_left(1);
        moved
    }

    #[allow(dead_code)]
    fn area(&self) -> i32 {
        (1 + self.elves.iter().map(|(x, _)| x).max().unwrap()
            - self.elves.iter().map(|(x, _)| x).min().unwrap())
            * (1 + self.elves.iter().map(|(_, y)| y).max().unwrap()
                - self.elves.iter().map(|(_, y)| y).min().unwrap())
    }
}

fn main() -> Result<()> {
    let mut land = Land::new(utils::get_input(23)?);
    let mut rounds = 1;
    while land.next_round() {
        rounds += 1;
    }
    println!("{:?}", rounds);
    Ok(())
}
