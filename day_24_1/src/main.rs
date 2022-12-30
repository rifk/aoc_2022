use eyre::Result;
use std::collections::HashSet;

static NORTH: u8 = 0b0000_0001;
static SOUTH: u8 = 0b0000_0010;
static EAST: u8 = 0b0000_0100;
static WEST: u8 = 0b0000_1000;

#[derive(Clone, Debug, PartialEq)]
enum State {
    Clear,
    Blizzards,
    End,
}

#[derive(Clone, Debug, PartialEq)]
struct Blizzards {
    blizzards: Vec<Vec<u8>>,
}
impl Blizzards {
    fn new(input: String) -> Self {
        Self {
            blizzards: input
                .lines()
                .filter(|l| !l.contains("###"))
                .map(|l| {
                    l.chars()
                        .filter(|c| *c != '#')
                        .map(|c| match c {
                            '.' => 0,
                            '^' => NORTH,
                            'v' => SOUTH,
                            '>' => EAST,
                            '<' => WEST,
                            _ => panic!(),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn next(&mut self) {
        let n = self.blizzards.len();
        let m = self.blizzards[0].len();
        let mut next = vec![vec![0; m]; n];

        for i in 0..n {
            for j in 0..m {
                if self.blizzards[i][j] & NORTH != 0 {
                    next[(i + n - 1) % n][j] ^= NORTH;
                }
                if self.blizzards[i][j] & SOUTH != 0 {
                    next[(i + 1) % n][j] ^= SOUTH;
                }
                if self.blizzards[i][j] & EAST != 0 {
                    next[i][(j + 1) % m] ^= EAST;
                }
                if self.blizzards[i][j] & WEST != 0 {
                    next[i][(j + m - 1) % m] ^= WEST;
                }
            }
        }

        self.blizzards = next;
    }

    fn state(&self, i: usize, j: usize) -> State {
        if j >= self.blizzards[0].len() {
            return State::Blizzards;
        } else if i == self.blizzards.len() {
            if j == self.blizzards[0].len() - 1 {
                return State::End;
            } else {
                return State::Blizzards;
            }
        }
        if self.blizzards[i][j] == 0 {
            State::Clear
        } else {
            State::Blizzards
        }
    }
}

fn main() -> Result<()> {
    let mut b = Blizzards::new(utils::get_input(24)?);
    let mut search = HashSet::new();
    let mut turns = 0;
    'end: loop {
        turns += 1;
        b.next();
        let mut check = HashSet::new();
        std::mem::swap(&mut check, &mut search);
        if b.state(0, 0) == State::Clear {
            search.insert((0, 0));
        }
        for (i, j) in check.drain() {
            match b.state(i + 1, j) {
                State::End => break 'end,
                State::Clear => {
                    search.insert((i + 1, j));
                }
                State::Blizzards => {}
            }
            if b.state(i, j) == State::Clear {
                search.insert((i, j));
            }
            if b.state(i, j + 1) == State::Clear {
                search.insert((i, j + 1));
            }
            if i > 0 && b.state(i - 1, j) == State::Clear {
                search.insert((i - 1, j));
            }
            if j > 0 && b.state(i, j - 1) == State::Clear {
                search.insert((i, j - 1));
            }
        }
    }
    println!("{:?}", turns);
    Ok(())
}
