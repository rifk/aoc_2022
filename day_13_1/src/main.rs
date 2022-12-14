use eyre::Result;
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(l), Packet::Int(r)) => l.cmp(r),
            (Packet::List(_), Packet::Int(_)) => self.cmp(&Packet::List(vec![other.clone()])),
            (Packet::Int(_), Packet::List(_)) => Packet::List(vec![self.clone()]).cmp(other),
            (Packet::List(l), Packet::List(r)) => {
                let mut i = 0;
                while l.len() > i && r.len() > i {
                    match l[i].cmp(&r[i]) {
                        Ordering::Equal => {
                            i += 1;
                        }
                        ord => return ord,
                    }
                }
                l.len().cmp(&r.len())
            }
        }
    }
}

fn main() -> Result<()> {
    let pairs = utils::get_input(13)?
        .lines()
        .collect::<Vec<&str>>()
        .split(|l| l.is_empty())
        .map(|pair| (line_to_packet(pair[0]), line_to_packet(pair[1])))
        .collect::<Vec<(Packet, Packet)>>();

    println!(
        "{}",
        pairs
            .iter()
            .enumerate()
            .filter(|(_, (l, r))| l.cmp(r) == Ordering::Less)
            .map(|(i, _)| i as i32 + 1)
            .sum::<i32>()
    );
    Ok(())
}

fn line_to_packet(l: &str) -> Packet {
    let mut chars = l.chars();
    assert!(chars.next().unwrap() == '[');
    Packet::List(get_list(&mut chars))
}

fn get_list(chars: &mut std::str::Chars<'_>) -> Vec<Packet> {
    let mut v = vec![];
    'o: while let Some(c) = chars.next() {
        if c == ']' {
            break 'o;
        } else if c == '[' {
            v.push(Packet::List(get_list(chars)));
        } else if c != ',' {
            let mut i = c.to_digit(10).unwrap();
            for c in chars.by_ref() {
                if c == ',' {
                    v.push(Packet::Int(i));
                    continue 'o;
                } else if c == ']' {
                    v.push(Packet::Int(i));
                    return v;
                } else {
                    i *= 10;
                    i += c.to_digit(10).unwrap();
                }
            }
        }
    }
    v
}
