use eyre::Result;
use std::collections::VecDeque;

const A: usize = 'a' as usize;

fn main() -> Result<()> {
    let input = utils::get_input(6)?;
    let mut iter = input.chars().enumerate();
    let mut four = VecDeque::new();
    let mut chars = vec![0; 26];
    let mut count = 0;
    for _ in 0..4 {
        if let Some((_, c)) = iter.next() {
            four.push_front(c);
            chars[c as usize - A] += 1;
            if chars[c as usize - A] == 1 {
                count += 1;
            }
        }
    }

    while count < 4 {
        let (_, c) = iter.next().unwrap();
        let l = four.pop_back().unwrap();
        chars[l as usize - A] -= 1;
        if chars[l as usize - A] == 0 {
            count -= 1;
        }

        four.push_front(c);
        chars[c as usize - A] += 1;
        if chars[c as usize - A] == 1 {
            count += 1;
        }
    }

    println!("{:?}", iter.next());
    Ok(())
}
