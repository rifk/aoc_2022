use eyre::Result;

struct Cpu {
    reg: i32,
    cycles: Vec<i32>,
    next: usize,
}
impl Cpu {
    fn new(input: String) -> Self {
        Self {
            reg: 1,
            cycles: input
                .lines()
                .flat_map(|l| match l {
                    "noop" => vec![0],
                    _ => {
                        let mut i = l.split_whitespace();
                        i.next().unwrap();
                        vec![0, i.next().unwrap().parse::<i32>().unwrap()]
                    }
                })
                .collect(),
            next: 0,
        }
    }
}
impl Iterator for Cpu {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next <= self.cycles.len() {
            let cur = self.reg;
            if self.next < self.cycles.len() {
                self.reg += self.cycles[self.next];
            }
            self.next += 1;
            Some(cur)
        } else {
            None
        }
    }
}

fn main() -> Result<()> {
    let cpu = Cpu::new(utils::get_input(10)?);
    println!(
        "{}",
        cpu.enumerate()
            .filter(|(c, _)| c >= &19 && (c - 19) % 40 == 0)
            .map(|(c, r)| {
                println!("{} - {}", c, r);
                (c + 1) as i32 * r
            })
            .sum::<i32>()
    );
    Ok(())
}
