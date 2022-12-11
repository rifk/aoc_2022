use eyre::Result;
use std::collections::VecDeque;
use std::ops::Fn;

struct Monkey {
    items: VecDeque<i32>,
    op: Box<dyn Fn(i32) -> i32>,
    throw: Box<dyn Fn(i32) -> usize>,
}
impl Monkey {
    // inspect an item, returning worry level an where it's thrown
    fn inspect(&mut self) -> Option<(i32, usize)> {
        self.items.pop_front().map(|mut w| {
            w = (self.op)(w);
            w /= 3;
            (w, (self.throw)(w))
        })
    }
}

fn main() -> Result<()> {
    let mut monkeys = get_monkeys(utils::get_input(11)?);
    let mut inspects = vec![0; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some((item, t)) = monkeys[i].inspect() {
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

// parse input to a vec of monkeys
fn get_monkeys(input: String) -> Vec<Monkey> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(7)
        .map(|m| Monkey {
            items: m[1]
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(", ")
                .map(|i| i.parse::<i32>().unwrap())
                .collect(),
            op: {
                let mut op = m[2]
                    .strip_prefix("  Operation: new = old ")
                    .unwrap()
                    .split_whitespace();
                match op.next().unwrap() {
                    "+" => match op.next().unwrap() {
                        "old" => Box::new(|w| w + w),
                        v => {
                            let v = v.parse::<i32>().unwrap();
                            Box::new(move |w| w + v)
                        }
                    },
                    "*" => match op.next().unwrap() {
                        "old" => Box::new(|w| w * w),
                        v => {
                            let v = v.parse::<i32>().unwrap();
                            Box::new(move |w| w * v)
                        }
                    },
                    o => panic!("unexpected op: {}", o),
                }
            },
            throw: {
                let div = m[3]
                    .strip_prefix("  Test: divisible by ")
                    .unwrap()
                    .parse::<i32>()
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
                Box::new(move |w| if w % div == 0 { t } else { f })
            },
        })
        .collect()
}
