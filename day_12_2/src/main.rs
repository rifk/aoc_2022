use eyre::Result;

fn main() -> Result<()> {
    let (map, _s, e) = {
        let mut s = (0, 0);
        let mut e = (0, 0);
        let map = utils::get_input(12)?
            .lines()
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        'S' => {
                            s = (i, j);
                            0
                        }
                        'E' => {
                            e = (i, j);
                            25
                        }
                        _ => c as i8 - 'a' as i8,
                    })
                    .collect::<Vec<i8>>()
            })
            .collect::<Vec<Vec<i8>>>();
        (map, s, e)
    };

    let mut seen = vec![vec![false; map[0].len()]; map.len()];
    seen[e.0][e.1] = true;
    let mut search = vec![e];
    let mut steps = 0;
    let mut end = false;

    while !end {
        steps += 1;
        search = search
            .into_iter()
            .flat_map(|(i, j)| {
                let mut next = vec![];

                if !end
                    && i > 0
                    && check_step((i, j), (i - 1, j), &mut next, &mut seen, &map)
                    && map[i - 1][j] == 0
                {
                    end = true;
                }
                if !end
                    && i < map.len() - 1
                    && check_step((i, j), (i + 1, j), &mut next, &mut seen, &map)
                    && map[i + 1][j] == 0
                {
                    end = true;
                }

                if !end
                    && j > 0
                    && check_step((i, j), (i, j - 1), &mut next, &mut seen, &map)
                    && map[i][j - 1] == 0
                {
                    end = true;
                }
                if !end
                    && j < map[0].len() - 1
                    && check_step((i, j), (i, j + 1), &mut next, &mut seen, &map)
                    && map[i][j + 1] == 0
                {
                    end = true;
                }

                next
            })
            .collect();
        assert!(!search.is_empty(), "no solutions, steps = {}", steps);
    }

    println!("{}", steps);

    Ok(())
}

// going backward, so can only step if its 1 below or higher
fn check_step(
    cur: (usize, usize),
    next: (usize, usize),
    search: &mut Vec<(usize, usize)>,
    seen: &mut [Vec<bool>],
    map: &[Vec<i8>],
) -> bool {
    if !seen[next.0][next.1] && map[next.0][next.1] >= map[cur.0][cur.1] - 1 {
        seen[next.0][next.1] = true;
        search.push(next);
        true
    } else {
        false
    }
}
