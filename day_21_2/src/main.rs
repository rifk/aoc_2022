use eyre::Result;
use std::collections::HashMap;

#[derive(Debug)]
enum Math {
    Unknown,
    Val(i64),
    Add(String, String),
    Minus(String, String),
    Div(String, String),
    Mult(String, String),
}

impl Math {
    fn eval(&self, monkeys: &HashMap<String, Monkey>) -> Self {
        let (left, right) = match self {
            Math::Val(v) => return Math::Val(*v),
            Math::Add(left,right) => (left,right),
            Math::Minus(left,right) => (left,right),
            Math::Div(left,right) => (left,right),
            Math::Mult(left,right) => (left,right),
            _ => panic!()
        };
        if left == "humn" || right  == "humn" {
            return Math::Unknown;
        }

        let left = {
            let l = monkeys.get(left).unwrap();
            let l = l.math.eval(monkeys);
            match l {
                Math::Unknown => return Math::Unknown,
                Math::Val(v) => v,
                _ => panic!(),
            }
        };
        let right = {
            let r = monkeys.get(right).unwrap();
            let r = r.math.eval(monkeys);
            match r {
                Math::Unknown => return Math::Unknown,
                Math::Val(v) => v,
                _ => panic!(),
            }
        };
        match self {
            Math::Add(_,_) => Math::Val(left+right),
            Math::Minus(_,_) => Math::Val(left-right),
            Math::Div(_,_) => Math::Val(left/right),
            Math::Mult(_,_) => Math::Val(left*right),
            _ => panic!(),
        }
    }
    
    fn eval_unknown(&self, monkeys: &HashMap<String, Monkey>, eq: i64) -> i64 {
        let (left,right) = match self {
            Math::Add(left,right) => (left,right),
            Math::Minus(left,right) => (left,right),
            Math::Div(left,right) => (left,right),
            Math::Mult(left,right) => (left,right),
            _ => panic!(),
        };
        let left_m = monkeys.get(left).unwrap();
        let right_m = monkeys.get(right).unwrap();
        let left_ev = left_m.math.eval(monkeys);
        let right_ev = right_m.math.eval(monkeys);
        if left == "humn" {
            let v = match right_ev {
                Math::Val(v) => v,
                _=> panic!(),
            };
            return match self {
                Math::Add(_, _) => eq - v,
                Math::Minus(_, _) => eq + v,
                Math::Div(_, _) => eq * v,
                Math::Mult(_, _) => eq / v,
                _ => panic!(),
            }
        } else if right == "humn" {
            let v = match left_ev {
                Math::Val(v) => v,
                _=> panic!(),
            };
            return match self {
                Math::Add(_, _) => eq - v,
                Math::Minus(_, _) => v - eq,
                Math::Div(_, _) => v / eq,
                Math::Mult(_, _) => eq / v,
                _ => panic!(),
            }
        }
        let (unknown, val) = match (&left_ev, &right_ev) {
            (Math::Unknown, Math::Val(v)) => (left_m, match self {
                Math::Add(_, _) => eq - v,
                Math::Minus(_, _) => eq + v,
                Math::Div(_, _) => eq * v,
                Math::Mult(_, _) => eq / v,
            _ => panic!(),
            }),
            (Math::Val(v), Math::Unknown) => (right_m, match self {
                Math::Add(_, _) => eq - v,
                Math::Minus(_, _) => v - eq,
                Math::Div(_, _) => v / eq,
                Math::Mult(_, _) => eq / v,
            _ => panic!(),
            }),
            _ => panic!("{:?}, {:?}", left_ev, right_ev),
        };
        unknown.math.eval_unknown(monkeys,val)
    }
}

struct Monkey {
    math: Math,
}

fn main() -> Result<()> {
    let mut monkeys = utils::get_input(21)?
        .lines()
        .map(line_to_monkey)
        .collect::<HashMap<String, Monkey>>();
    let root = monkeys.remove("root").unwrap();
    println!("{:?}", humn_val(monkeys, root));
    Ok(())
}

fn humn_val(monkeys: HashMap<String, Monkey>, root: Monkey) -> i64 {
    let (left,right) = match root.math {
        Math::Add(left,right) => (left,right),
        Math::Minus(left,right) => (left,right),
        Math::Div(left,right) => (left,right),
        Math::Mult(left,right) => (left,right),
        _ => panic!(),
    };
    let left_m = monkeys.get(&left).unwrap();
    let right_m = monkeys.get(&right).unwrap();
    let left_ev = left_m.math.eval(&monkeys);
    let right_ev = right_m.math.eval(&monkeys);
    let (unknown, val) = match (left_ev, right_ev) {
        (Math::Unknown, Math::Val(v)) => (left_m, v),
        (Math::Val(v), Math::Unknown) => (right_m, v),
        _ => panic!(),
    };
    unknown.math.eval_unknown(&monkeys,val)
}

fn line_to_monkey(line: &str) -> (String, Monkey) {
    let (name, line) = line.split_once(": ").unwrap();
    let mut l_i = line.split_whitespace();
    let left = l_i.next().unwrap().to_string();
    if let Some(o) = l_i.next() {
        let right = l_i.next().unwrap().to_string();
        (
            name.to_string(),
            Monkey {
                math: match o {
                    "+" => Math::Add(left, right),
                    "-" => Math::Minus(left, right),
                    "/" => Math::Div(left, right),
                    "*" => Math::Mult(left, right),
                    _ => panic!(),
                },
            },
        )
    } else {
        let val = left.parse::<i64>().unwrap();
        (
            name.to_string(),
            Monkey {
                math: Math::Val(val),
            },
        )
    }
}
