use eyre::Result;
use std::collections::VecDeque;
use std::ops::Fn;

struct Monkey {
    items: VecDeque<i64>,
    op: Box<dyn Fn(i64) -> i64>,
    throw: Box<dyn Fn(i64) -> usize>,
}
impl Monkey {
    // inspect an item, returning worry level an where it's thrown
    fn inspect(&mut self, lcm: i64) -> Option<(i64, usize)> {
        self.items.pop_front().map(|mut w| {
            w = (self.op)(w);
            w %= lcm;
            (w, (self.throw)(w))
        })
    }
}

fn main() -> Result<()> {
    let (mut monkeys, lcm) = get_monkeys(utils::get_input(11)?);
    let mut inspects: Vec<i64> = vec![0; monkeys.len()];

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while let Some((item, t)) = monkeys[i].inspect(lcm) {
                inspects[i] += 1;
                monkeys[t].items.push_back(item);
            }
        }
    }

    inspects.sort();
    inspects.reverse();
    println!("{:?}", inspects[0] * inspects[1]);

    Ok(())
}

// parse input to a vec of monkeys, and get lcm of test divisors
fn get_monkeys(input: String) -> (Vec<Monkey>, i64) {
    let mut monkeys = Vec::new();
    let mut lcm = 1;
    for m in input.lines().collect::<Vec<&str>>().chunks(7) {
        let items = m[1]
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|i| i.parse::<i64>().unwrap())
            .collect();
        let op: Box<dyn Fn(i64) -> i64> = {
            let mut op = m[2]
                .strip_prefix("  Operation: new = old ")
                .unwrap()
                .split_whitespace();
            match op.next().unwrap() {
                "+" => match op.next().unwrap() {
                    "old" => Box::new(|w| w + w),
                    v => {
                        let v = v.parse::<i64>().unwrap();
                        Box::new(move |w| w + v)
                    }
                },
                "*" => match op.next().unwrap() {
                    "old" => Box::new(|w| w * w),
                    v => {
                        let v = v.parse::<i64>().unwrap();
                        Box::new(move |w| w * v)
                    }
                },
                o => panic!("unexpected op: {}", o),
            }
        };
        let (throw, div) = {
            let div = m[3]
                .strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let t = m[4]
                .strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let f = m[5]
                .strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            (Box::new(move |w| if w % div == 0 { t } else { f }), div)
        };
        monkeys.push(Monkey { items, op, throw });
        lcm = num_integer::lcm(lcm, div);
    }
    (monkeys, lcm)
}
