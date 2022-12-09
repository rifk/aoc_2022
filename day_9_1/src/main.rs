use eyre::Result;
use std::cmp::Ordering;
use std::collections::HashSet;

fn main() -> Result<()> {
    let mut visited = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    visited.insert(tail);
    for s in utils::get_input(9)?.lines() {
        let (d, v) = {
            let mut s = s.split_whitespace();
            (s.next().unwrap(), s.next().unwrap().parse::<i32>().unwrap())
        };

        match d {
            "R" => {
                for _ in 0..v {
                    head.1 += 1;
                    tail = move_tail(&head, tail);
                    visited.insert(tail);
                }
            }
            "L" => {
                for _ in 0..v {
                    head.1 -= 1;
                    tail = move_tail(&head, tail);
                    visited.insert(tail);
                }
            }
            "U" => {
                for _ in 0..v {
                    head.0 += 1;
                    tail = move_tail(&head, tail);
                    visited.insert(tail);
                }
            }
            "D" => {
                for _ in 0..v {
                    head.0 -= 1;
                    tail = move_tail(&head, tail);
                    visited.insert(tail);
                }
            }
            d => panic!("unexpected direction: {}", d),
        }
    }

    println!("{:?}", visited.len());
    Ok(())
}

fn move_tail(head: &(i32, i32), mut tail: (i32, i32)) -> (i32, i32) {
    let diff = ((head.0 - tail.0).abs(), (head.1 - tail.1).abs());
    if diff.0 > 1 || diff.1 > 1 || (diff.0 + diff.1) > 2 {
        match tail.0.cmp(&head.0) {
            Ordering::Less => tail.0 += 1,
            Ordering::Equal => {}
            Ordering::Greater => tail.0 -= 1,
        }
        match tail.1.cmp(&head.1) {
            Ordering::Less => tail.1 += 1,
            Ordering::Equal => {}
            Ordering::Greater => tail.1 -= 1,
        }
    }
    //println!("{:?} - {:?}", head, tail);
    tail
}
