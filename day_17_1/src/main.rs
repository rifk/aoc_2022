use eyre::Result;

struct Chamber {
    v: Vec<Vec<bool>>,
    falling: Vec<(usize, usize)>,
}
impl Chamber {
    #[allow(dead_code)]
    fn pretty_print(&self) {
        println!("{}", self.v.len());
        println!("---------");
        for l in &self.v {
            print!("|");
            for c in l {
                if *c {
                    print!("@");
                } else {
                    print!(" ");
                }
            }
            println!("|");
        }
    }

    fn left(&mut self) -> bool {
        assert!(!self.falling.is_empty());
        let mut left = vec![];
        for (i, j) in &self.falling {
            if *j == 0 || self.contains(*i, j - 1) {
                return false;
            }
            left.push((*i, j - 1));
        }
        self.falling = left;
        true
    }

    fn right(&mut self) -> bool {
        assert!(!self.falling.is_empty());
        let mut right = vec![];
        for (i, j) in &self.falling {
            if *j == 6 || self.contains(*i, j + 1) {
                return false;
            }
            right.push((*i, j + 1));
        }
        self.falling = right;
        true
    }

    fn down(&mut self) -> bool {
        assert!(!self.falling.is_empty());
        let mut m = true;
        let mut down = vec![];
        'o: for (i, j) in &self.falling {
            if *i == 0 || self.contains(i - 1, *j) {
                m = false;
                break 'o;
            }
            down.push((i - 1, *j));
        }
        if m {
            self.falling = down;
            m
        } else {
            let mut to_add = vec![];
            std::mem::swap(&mut to_add, &mut self.falling);

            for (i, j) in to_add {
                self.add(i, j);
            }
            false
        }
    }

    fn has_falling(&self) -> bool {
        !self.falling.is_empty()
    }

    fn contains(&self, i: usize, j: usize) -> bool {
        self.v.get(i).map(|l| l[j]).unwrap_or(false)
    }

    fn add(&mut self, i: usize, j: usize) {
        if i >= self.v.len() {
            let l = 1 + i - self.v.len();
            for _ in 0..l {
                self.v.push(vec![false; 7]);
            }
        }
        self.v[i][j] = true;
    }

    fn new_rock(&mut self, i: i32) {
        assert!(self.falling.is_empty());
        let h = self.v.len();
        match i {
            0 => {
                self.falling.push((h + 3, 2));
                self.falling.push((h + 3, 3));
                self.falling.push((h + 3, 4));
                self.falling.push((h + 3, 5));
            }
            1 => {
                self.falling.push((h + 3, 3));
                self.falling.push((h + 4, 2));
                self.falling.push((h + 4, 3));
                self.falling.push((h + 4, 4));
                self.falling.push((h + 5, 3));
            }
            2 => {
                self.falling.push((h + 3, 2));
                self.falling.push((h + 3, 3));
                self.falling.push((h + 3, 4));
                self.falling.push((h + 4, 4));
                self.falling.push((h + 5, 4));
            }
            3 => {
                self.falling.push((h + 3, 2));
                self.falling.push((h + 4, 2));
                self.falling.push((h + 5, 2));
                self.falling.push((h + 6, 2));
            }
            4 => {
                self.falling.push((h + 3, 2));
                self.falling.push((h + 3, 3));
                self.falling.push((h + 4, 2));
                self.falling.push((h + 4, 3));
            }
            _ => panic!(),
        }
    }
}

fn main() -> Result<()> {
    let input = utils::get_input(17)?;
    let mut input = input.trim().chars().cycle();

    let mut chamber = Chamber {
        v: vec![],
        falling: vec![],
    };

    let mut rock = 0;

    while rock <= 2022 {
        if !chamber.has_falling() {
            chamber.new_rock(rock % 5);
            rock += 1;
        }
        match input.next().unwrap() {
            '>' => {
                chamber.right();
            }
            '<' => {
                chamber.left();
            }
            other => panic!("unexpected char \"{other}\", rock = {rock}"),
        }
        chamber.down();
        //chamber.pretty_print();
    }

    println!("{}", chamber.v.len());

    Ok(())
}
