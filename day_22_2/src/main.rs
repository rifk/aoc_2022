use eyre::Result;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
enum Side {
    Current,
    Opposite,
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
    fn left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Path {
    Right,
    Left,
    Forward(u32),
}

#[derive(Clone, Debug)]
struct Board {
    board: Vec<Vec<i8>>,
    pos: (usize, usize),
    direction: Direction,
    sides: Vec<Vec<bool>>,
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
        let mut sides = vec![vec![false; max_l / 50]; board.len() / 50];
        for i in 0..sides.len() {
            for j in 0..sides[0].len() {
                if board[i * 50][j * 50] != 0 {
                    sides[i][j] = true;
                }
            }
        }
        let j = board[0]
            .iter()
            .enumerate()
            .find(|(_, &v)| v == 1)
            .unwrap()
            .0;
        Self {
            board,
            pos: (0, j),
            direction: Direction::East,
            sides,
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
            let (i, j, d) = match self.direction {
                Direction::North => {
                    if self.pos.0 == 0 || self.board[self.pos.0 - 1][self.pos.1] == 0 {
                        self.around_corner()
                    } else {
                        (self.pos.0 - 1, self.pos.1, self.direction.clone())
                    }
                }
                Direction::South => {
                    if self.pos.0 >= self.board.len() - 1
                        || self.board[self.pos.0 + 1][self.pos.1] == 0
                    {
                        self.around_corner()
                    } else {
                        (self.pos.0 + 1, self.pos.1, self.direction.clone())
                    }
                }
                Direction::East => {
                    if self.pos.1 >= (self.board[0].len() - 1)
                        || self.board[self.pos.0][self.pos.1 + 1] == 0
                    {
                        self.around_corner()
                    } else {
                        (self.pos.0, self.pos.1 + 1, self.direction.clone())
                    }
                }
                Direction::West => {
                    if self.pos.1 == 0 || self.board[self.pos.0][self.pos.1 - 1] == 0 {
                        self.around_corner()
                    } else {
                        (self.pos.0, self.pos.1 - 1, self.direction.clone())
                    }
                }
            };
            if self.board[i][j] == 2 {
                return;
            }
            self.pos = (i, j);
            self.direction = d;
        }
    }
    // next forward is around a corner of the cube which is not joined on our input
    // calulate were it ends up and which direction it will face
    // this does a BFS between the sides which are joined in our input, to find where the new
    // position will be
    fn around_corner(&self) -> (usize, usize, Direction) {
        let aim = match self.direction {
            Direction::East => Side::East,
            Direction::South => Side::South,
            Direction::West => Side::West,
            Direction::North => Side::North,
        };
        let (c_i, c_j) = (self.pos.0 / 50, self.pos.1 / 50);
        let mut seen = HashSet::new();
        seen.insert((c_i, c_j));
        let mut search = HashMap::new();
        search.insert((c_i, c_j), (Side::Current, Direction::North));
        while !search.is_empty() {
            let mut next = HashMap::new();

            for ((i, j), (s, d)) in search.drain() {
                if i > 0 && !seen.contains(&(i - 1, j)) && self.sides[i - 1][j] {
                    let (n_s, n_d) =
                        self.next_side_direction(Direction::North, s.clone(), d.clone());
                    if n_s == aim {
                        return self.enter_side(i - 1, j, n_d);
                    }
                    seen.insert((i - 1, j));
                    next.insert((i - 1, j), (n_s, n_d));
                }
                if i < self.sides.len() - 1 && !seen.contains(&(i + 1, j)) && self.sides[i + 1][j] {
                    let (n_s, n_d) =
                        self.next_side_direction(Direction::South, s.clone(), d.clone());
                    if n_s == aim {
                        return self.enter_side(i + 1, j, n_d);
                    }
                    seen.insert((i + 1, j));
                    next.insert((i + 1, j), (n_s, n_d));
                }
                if j > 0 && !seen.contains(&(i, j - 1)) && self.sides[i][j - 1] {
                    let (n_s, n_d) =
                        self.next_side_direction(Direction::West, s.clone(), d.clone());
                    if n_s == aim {
                        return self.enter_side(i, j - 1, n_d);
                    }
                    seen.insert((i, j - 1));
                    next.insert((i, j - 1), (n_s, n_d));
                }
                if j < self.sides[0].len() - 1
                    && !seen.contains(&(i, j + 1))
                    && self.sides[i][j + 1]
                {
                    let (n_s, n_d) = self.next_side_direction(Direction::East, s, d);
                    if n_s == aim {
                        return self.enter_side(i, j + 1, n_d);
                    }
                    seen.insert((i, j + 1));
                    next.insert((i, j + 1), (n_s, n_d));
                }
            }

            search = next;
        }
        panic!()
    }
    // moving between joined sides in our input, calculate which side we will be on
    fn next_side_direction(&self, m_d: Direction, s: Side, s_d: Direction) -> (Side, Direction) {
        match s {
            Side::Current => match m_d {
                Direction::East => (Side::East, m_d),
                Direction::South => (Side::South, m_d),
                Direction::West => (Side::West, m_d),
                Direction::North => (Side::North, m_d),
            },
            Side::Opposite => {
                let n_d = m_d.left().left();
                let m_d = match s_d {
                    Direction::North => m_d,
                    Direction::East => m_d.left(),
                    Direction::South => m_d.left().left(),
                    Direction::West => m_d.right(),
                };
                match m_d {
                    Direction::East => (Side::West, n_d),
                    Direction::South => (Side::South, n_d),
                    Direction::West => (Side::East, n_d),
                    Direction::North => (Side::North, n_d),
                }
            }
            Side::North => {
                let n_d = s_d.clone();
                let m_d = match s_d {
                    Direction::North => m_d,
                    Direction::East => m_d.left(),
                    Direction::South => m_d.left().left(),
                    Direction::West => m_d.right(),
                };
                match m_d {
                    Direction::North => (Side::Opposite, n_d.left().left()),
                    Direction::East => (Side::East, n_d),
                    Direction::South => panic!("back to current"),
                    Direction::West => (Side::West, n_d),
                }
            }
            Side::South => {
                let n_d = s_d.clone();
                let m_d = match s_d {
                    Direction::North => m_d,
                    Direction::East => m_d.left(),
                    Direction::South => m_d.left().left(),
                    Direction::West => m_d.right(),
                };
                match m_d {
                    Direction::North => (Side::Opposite, n_d),
                    Direction::East => (Side::West, n_d),
                    Direction::South => panic!("back to current"),
                    Direction::West => (Side::East, n_d),
                }
            }
            Side::East => {
                let n_d = s_d.clone();
                let m_d = match s_d {
                    Direction::North => m_d,
                    Direction::East => m_d.left(),
                    Direction::South => m_d.left().left(),
                    Direction::West => m_d.right(),
                };
                match m_d {
                    Direction::North => (Side::Opposite, n_d.left()),
                    Direction::East => (Side::South, n_d),
                    Direction::South => panic!("back to current"),
                    Direction::West => (Side::North, n_d),
                }
            }
            Side::West => {
                let n_d = s_d.clone();
                let m_d = match s_d {
                    Direction::North => m_d,
                    Direction::East => m_d.left(),
                    Direction::South => m_d.left().left(),
                    Direction::West => m_d.right(),
                };
                match m_d {
                    Direction::North => (Side::Opposite, n_d.right()),
                    Direction::East => (Side::North, n_d),
                    Direction::South => panic!("back to current"),
                    Direction::West => (Side::South, n_d),
                }
            }
        }
    }
    // we know where the side is we will enter onto, calculate exact position
    fn enter_side(&self, i: usize, j: usize, d: Direction) -> (usize, usize, Direction) {
        let offset = match self.direction {
            Direction::North => self.pos.1 % 50,
            Direction::East => self.pos.0 % 50,
            Direction::South => 49 - (self.pos.1 % 50),
            Direction::West => 49 - (self.pos.0 % 50),
        };
        match d {
            Direction::North => (((i + 1) * 50) - 1, (j * 50) + offset, d),
            Direction::East => ((i * 50) + offset, j * 50, d),
            Direction::South => (i * 50, (j * 50) + 49 - offset, d),
            Direction::West => ((i * 50) + 49 - offset, ((j + 1) * 50) - 1, d),
        }
    }

    fn score(&self) -> i32 {
        (1000 * (self.pos.0 + 1) as i32)
            + (4 * (self.pos.1 + 1) as i32)
            + match self.direction {
                Direction::East => 0,
                Direction::South => 1,
                Direction::West => 2,
                Direction::North => 3,
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
