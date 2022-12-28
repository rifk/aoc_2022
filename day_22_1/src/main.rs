use eyre::Result;

enum Directions {
    North,
    South,
    East,
    West,
}
impl Directions {
    fn right(&self) -> Self {
        match self {
            Directions::North => Directions::East,
            Directions::South => Directions::West,
            Directions::East => Directions::South,
            Directions::West => Directions::North,
        }
    }
    fn left(&self) -> Self {
        match self {
            Directions::North => Directions::West,
            Directions::South => Directions::East,
            Directions::East => Directions::North,
            Directions::West => Directions::South,
        }
    }
}

enum Path {
    Right,
    Left,
    Forward(u32),
}

struct Board {
    board: Vec<Vec<i8>>,
    pos: (usize, usize),
    direction: Directions,
}
impl Board {
    fn new(board: Vec<&str>) -> Self {
        let board = board
            .into_iter()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        ' ' => 0,
                        '.' => 1,
                        '#' => 2,
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect::<Vec<Vec<i8>>>();
        let max_l = board.iter().map(|l| l.len()).max().unwrap();
        let board = board
            .into_iter()
            .map(|mut l| {
                if l.len() < max_l {
                    l.resize(max_l, 0);
                    l
                } else {
                    l
                }
            })
            .collect::<Vec<Vec<i8>>>();
        let j = board[0]
            .iter()
            .enumerate()
            .find(|(_, &v)| v == 1)
            .unwrap()
            .0;
        Self {
            board,
            pos: (0, j),
            direction: Directions::East,
        }
    }
    fn right(&mut self) {
        self.direction = self.direction.right();
    }
    fn left(&mut self) {
        self.direction = self.direction.left();
    }
    fn forward(&mut self, v: u32) {
        for _ in 0..v {
            match self.direction {
                Directions::North => {
                    let next_i = if self.pos.0 == 0 || self.board[self.pos.0 - 1][self.pos.1] == 0 {
                        let mut i = self.pos.0;
                        while i < self.board.len() - 1 && self.board[i + 1][self.pos.1] != 0 {
                            i += 1;
                        }
                        if i == self.pos.0 {
                            return;
                        }
                        i
                    } else {
                        self.pos.0 - 1
                    };
                    if self.board[next_i][self.pos.1] == 2 {
                        return;
                    }
                    self.pos.0 = next_i;
                }
                Directions::South => {
                    let next_i = if self.pos.0 >= self.board.len() - 1
                        || self.board[self.pos.0 + 1][self.pos.1] == 0
                    {
                        let mut i = self.pos.0;
                        while i > 0 && self.board[i - 1][self.pos.1] != 0 {
                            i -= 1;
                        }
                        if i == self.pos.0 {
                            return;
                        }
                        i
                    } else {
                        self.pos.0 + 1
                    };
                    if self.board[next_i][self.pos.1] == 2 {
                        return;
                    }
                    self.pos.0 = next_i;
                }
                Directions::East => {
                    let next_j = if self.pos.1 >= (self.board[0].len() - 1)
                        || self.board[self.pos.0][self.pos.1 + 1] == 0
                    {
                        let mut j = self.pos.1;
                        while j > 0 && self.board[self.pos.0][j - 1] != 0 {
                            j -= 1;
                        }
                        if j == self.pos.1 {
                            return;
                        }
                        j
                    } else {
                        self.pos.1 + 1
                    };
                    if self.board[self.pos.0][next_j] == 2 {
                        return;
                    }
                    self.pos.1 = next_j;
                }
                Directions::West => {
                    let next_j = if self.pos.1 == 0 || self.board[self.pos.0][self.pos.1 - 1] == 0 {
                        let mut j = self.pos.1;
                        while j < self.board[0].len() - 1 && self.board[self.pos.0][j + 1] != 0 {
                            j += 1;
                        }
                        if j == self.pos.1 {
                            return;
                        }
                        j
                    } else {
                        self.pos.1 - 1
                    };
                    if self.board[self.pos.0][next_j] == 2 {
                        return;
                    }
                    self.pos.1 = next_j;
                }
            }
        }
    }
    fn score(&self) -> i32 {
        (1000 * (self.pos.0 + 1) as i32)
            + (4 * (self.pos.1 + 1) as i32)
            + match self.direction {
                Directions::East => 0,
                Directions::South => 1,
                Directions::West => 2,
                Directions::North => 3,
            }
    }
}

fn main() -> Result<()> {
    let (mut board, path) = parse_input(utils::get_input(22)?);
    for p in path {
        match p {
            Path::Right => board.right(),
            Path::Left => board.left(),
            Path::Forward(v) => board.forward(v),
        }
    }
    println!("{:?}", board.score());
    Ok(())
}

fn parse_input(input: String) -> (Board, Vec<Path>) {
    let mut i = input.lines();

    let mut board = vec![];
    for l in i.by_ref() {
        if l.is_empty() {
            break;
        }
        board.push(l);
    }

    let path = i
        .next()
        .unwrap()
        .split_inclusive(&['R', 'L'])
        .flat_map(|p| {
            if p == "R" {
                vec![Path::Right]
            } else if p == "L" {
                vec![Path::Left]
            } else if p.ends_with('R') {
                vec![
                    Path::Forward(p[0..(p.len() - 1)].parse::<u32>().unwrap()),
                    Path::Right,
                ]
            } else if p.ends_with('L') {
                vec![
                    Path::Forward(p[0..(p.len() - 1)].parse::<u32>().unwrap()),
                    Path::Left,
                ]
            } else {
                vec![Path::Forward(p.parse::<u32>().unwrap())]
            }
        })
        .collect::<Vec<_>>();

    (Board::new(board), path)
}
