use eyre::Result;

static RIGHT: u8 = 0b0000_0001;
static LEFT: u8 = 0b0100_0000;

static FULL: u8 = 0b0111_1111;

static R0: [u8; 1] = [0b0001_1110];
static R1: [u8; 3] = [0b0000_1000, 0b0001_1100, 0b0000_1000];
static R2: [u8; 3] = [0b0001_1100, 0b0000_0100, 0b0000_0100];
static R3: [u8; 4] = [0b0001_0000, 0b0001_0000, 0b0001_0000, 0b0001_0000];
static R4: [u8; 2] = [0b0001_1000, 0b0001_1000];

struct Chamber {
    v: Vec<u8>,
    falling: Vec<u8>,
    bottom: usize,
    cleared: usize,
}
impl Chamber {
    fn new() -> Self {
        Self {
            v: vec![],
            falling: vec![],
            bottom: 0,
            cleared: 0,
        }
    }

    fn left(&mut self) -> bool {
        debug_assert!(!self.falling.is_empty());
        for i in 0..self.falling.len() {
            if !self.can_left(self.falling[i], self.bottom + i) {
                // cannot move left, reverse pervious moves
                for j in 0..i {
                    self.falling[j] >>= 1;
                }
                return false;
            } else {
                self.falling[i] <<= 1;
            }
        }
        true
    }
    fn can_left(&self, r: u8, i: usize) -> bool {
        ((r & LEFT) == 0) && (i >= self.v.len() || (self.v[i] & (r << 1)) == 0)
    }

    fn right(&mut self) -> bool {
        debug_assert!(!self.falling.is_empty());
        for i in 0..self.falling.len() {
            if !self.can_right(self.falling[i], self.bottom + i) {
                // cannot move right, reverse pervious moves
                for j in 0..i {
                    self.falling[j] <<= 1;
                }
                return false;
            } else {
                self.falling[i] >>= 1;
            }
        }
        true
    }
    fn can_right(&self, r: u8, i: usize) -> bool {
        ((r & RIGHT) == 0) && (i >= self.v.len() || (self.v[i] & (r >> 1)) == 0)
    }

    fn down(&mut self) -> bool {
        debug_assert!(!self.falling.is_empty());
        if self.can_down() {
            self.bottom -= 1;
            true
        } else {
            let mut to_add = vec![];
            std::mem::swap(&mut self.falling, &mut to_add);
            let mut clear = None;
            for (i, r) in to_add.into_iter().enumerate() {
                if self.add(r, self.bottom + i) {
                    clear = Some(self.bottom + i);
                }
            }
            if let Some(c) = clear {
                self.cleared += c + 1;
                self.v = self.v.split_off(c + 1);
            }
            false
        }
    }
    fn can_down(&self) -> bool {
        if self.bottom == 0 {
            // at floor
            return false;
        }

        if self.bottom - 1 < self.v.len() {
            // check if moving clashes with rocks
            if (self.v[self.bottom - 1] & self.falling[0]) != 0 {
                // clashes
                return false;
            }
            // check next row if exists and clashes
            if self.falling.len() > 1
                && self.bottom < self.v.len()
                && (self.v[self.bottom] & self.falling[1]) != 0
            {
                // clashes
                return false;
            }
        }
        true
    }
    fn add(&mut self, r: u8, i: usize) -> bool {
        if i >= self.v.len() {
            self.v.push(r);
            false
        } else {
            self.v[i] |= r;
            self.v[i] == FULL
        }
    }

    fn has_falling(&self) -> bool {
        !self.falling.is_empty()
    }

    fn new_rock(&mut self, i: u64) {
        debug_assert!(self.falling.is_empty());
        self.bottom = self.v.len();
        match i {
            0 => {
                self.falling = R0.to_vec();
            }
            1 => {
                self.falling = R1.to_vec();
            }
            2 => {
                self.falling = R2.to_vec();
            }
            3 => {
                self.falling = R3.to_vec();
            }
            4 => {
                self.falling = R4.to_vec();
            }
            _ => panic!(),
        }
    }
}

fn main() -> Result<()> {
    let input = utils::get_input(17)?;
    let mut input = input.trim().chars().cycle();

    let mut chamber = Chamber::new();

    let mut rock = 0;

    let mut cleared_changes = vec![];
    let mut repeated = false;

    loop {
        let range = if !chamber.has_falling() {
            chamber.new_rock(rock % 5);
            rock += 1;
            0..4
        } else {
            0..1
        };
        for _ in range {
            match input.next().unwrap() {
                '>' => {
                    chamber.right();
                }
                '<' => {
                    chamber.left();
                }
                other => panic!("unexpected char \"{other}\", rock = {rock}"),
            }
        }
        if !chamber.down() {
            if rock == 1_000_000_000_000 {
                break;
            }
            if !repeated && rock > 100000 {
                // check for repeated pattern in cleared numbers if not used and after 30 rocks
                if cleared_changes.is_empty() {
                    cleared_changes.push((chamber.cleared, rock, chamber.cleared, rock));
                    continue;
                }
                if let Some((_, _, c, r)) = cleared_changes.last() {
                    if *c != chamber.cleared {
                        cleared_changes.push((
                            chamber.cleared - c,
                            rock - r,
                            chamber.cleared,
                            rock,
                        ));
                    }
                }

                if cleared_changes.len() < 6 {
                    continue;
                }

                let mut repeat = vec![];
                repeat.push((cleared_changes[2].0, cleared_changes[2].1));
                repeat.push((cleared_changes[3].0, cleared_changes[3].1));
                let mut is_repeat = false;
                for i in 4..cleared_changes.len() {
                    if (cleared_changes.len() - (i + 1)) < repeat.len() {
                        break;
                    }
                    let mut r = true;
                    for j in 0..repeat.len() {
                        if repeat[j].0 != cleared_changes[i + j].0
                            || repeat[j].1 != cleared_changes[i + j].1
                        {
                            r = false;
                            break;
                        }
                    }
                    if r {
                        is_repeat = true;
                        break;
                    } else {
                        repeat.push((cleared_changes[i].0, cleared_changes[i].1));
                    }
                }
                if is_repeat {
                    repeated = true;
                    let repeat = repeat.into_iter().fold((0, 0), |mut p, (c, r)| {
                        p.0 += c;
                        p.1 += r;
                        p
                    });
                    let times = (999_999_999_999 - rock) / repeat.1;
                    rock += times * repeat.1;
                    chamber.cleared += times as usize * repeat.0 as usize;
                }
            }
        }
    }

    println!("{}", chamber.v.len() + chamber.cleared);

    Ok(())
}
