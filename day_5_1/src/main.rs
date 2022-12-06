use eyre::Result;

fn main() -> Result<()> {
    let input = utils::get_input(5)?;
    let mut in_iter = input.lines();

    let mut stacks = {
        let mut stacks = Vec::new();
        'w: for l in in_iter.by_ref() {
            if l.is_empty() {
                break 'w;
            }
            stacks.push(l);
        }

        let mut stacks_iter = stacks.iter().rev();
        let num_stacks = count_stacks(stacks_iter.next().unwrap());
        let mut stacks = vec![Vec::new(); num_stacks];
        for l in stacks_iter {
            let mut l_iter = l.chars().skip(1).step_by(4);
            for stack in stacks.iter_mut().take(num_stacks) {
                if let Some(c) = l_iter.next() {
                    if c != ' ' {
                        stack.push(c);
                    }
                }
            }
        }
        stacks
    };

    for l in in_iter {
        let mut l_iter = l.split_whitespace().skip(1).step_by(2);
        let count = l_iter.next().unwrap().parse::<usize>().unwrap();
        let from = l_iter.next().unwrap().parse::<usize>().unwrap() - 1;
        let to = l_iter.next().unwrap().parse::<usize>().unwrap() - 1;
        for _ in 0..count {
            let c = stacks[from].pop().unwrap();
            stacks[to].push(c);
        }
    }

    println!(
        "{:?}",
        stacks
            .into_iter()
            .map(|s| *s.last().unwrap())
            .collect::<String>()
    );

    Ok(())
}

fn count_stacks(l: &str) -> usize {
    l.split_whitespace().count()
}
