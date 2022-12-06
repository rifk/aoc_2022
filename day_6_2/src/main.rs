use eyre::Result;
use std::collections::VecDeque;

const A: usize = 'a' as usize;

fn main() -> Result<()> {
    let input = utils::get_input(6)?;
    let mut iter = input.chars().enumerate();
    let mut window = VecDeque::new();
    let mut chars = vec![0; 26];
    let mut count = 0;

    for _ in 0..4 {
        if let Some((_, c)) = iter.next() {
            add_char(c, &mut count, &mut window, &mut chars);
        }
    }
    while count < 4 {
        let (_, c) = iter.next().unwrap();
        next_char(c, &mut count, &mut window, &mut chars);
    }

    count = 0;
    window.clear();
    chars = vec![0; 26];

    for _ in 0..14 {
        if let Some((_, c)) = iter.next() {
            add_char(c, &mut count, &mut window, &mut chars);
        }
    }
    while count < 14 {
        let (_, c) = iter.next().unwrap();
        next_char(c, &mut count, &mut window, &mut chars);
    }

    println!("{:?}", iter.next());

    Ok(())
}

fn add_char(next: char, count: &mut i32, window: &mut VecDeque<char>, chars: &mut [i32]) {
    window.push_front(next);
    chars[next as usize - A] += 1;
    if chars[next as usize - A] == 1 {
        *count += 1;
    }
}

fn next_char(next: char, count: &mut i32, window: &mut VecDeque<char>, chars: &mut [i32]) {
    let l = window.pop_back().unwrap();
    chars[l as usize - A] -= 1;
    if chars[l as usize - A] == 0 {
        *count -= 1;
    }

    window.push_front(next);
    chars[next as usize - A] += 1;
    if chars[next as usize - A] == 1 {
        *count += 1;
    }
}
