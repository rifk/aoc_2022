use eyre::Result;
use std::collections::HashMap;
use std::ops::Fn;

type Val = Box<dyn Fn(&HashMap<String, Monkey>, &mut HashMap<String, i64>) -> i64>;

struct Monkey {
    val: Val,
}

fn main() -> Result<()> {
    let monkeys = utils::get_input(21)?
        .lines()
        .map(line_to_monkey)
        .collect::<HashMap<String, Monkey>>();
    let mut vals = HashMap::new();
    let root = monkeys.get("root").unwrap();
    println!("{:?}", (root.val)(&monkeys, &mut vals));
    Ok(())
}

fn line_to_monkey(line: &str) -> (String, Monkey) {
    let (name, line) = line.split_once(": ").unwrap();
    let mut l_i = line.split_whitespace();
    let left = l_i.next().unwrap().to_string();
    if let Some(o) = l_i.next() {
        let o = o.to_string();
        let right = l_i.next().unwrap().to_string();
        (
            name.to_string(),
            Monkey {
                val: Box::new(move |monkeys, vals| {
                    let left = left.clone();
                    let left = if let Some(l) = vals.get(&left) {
                        *l
                    } else {
                        let l = (monkeys.get(&left).unwrap().val)(monkeys, vals);
                        vals.insert(left, l);
                        l
                    };
                    let right = right.clone();
                    let right = if let Some(r) = vals.get(&right) {
                        *r
                    } else {
                        let r = (monkeys.get(&right).unwrap().val)(monkeys, vals);
                        vals.insert(right, r);
                        r
                    };
                    let o = o.clone();
                    match o.as_ref() {
                        "+" => left + right,
                        "-" => left - right,
                        "/" => left / right,
                        "*" => left * right,
                        _ => panic!(),
                    }
                }),
            },
        )
    } else {
        let out = left.parse::<i64>().unwrap();
        (
            name.to_string(),
            Monkey {
                val: Box::new(move |_, _| out),
            },
        )
    }
}
