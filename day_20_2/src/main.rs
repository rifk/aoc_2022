use eyre::Result;

fn main() -> Result<()> {
    let list = utils::get_input(20)?
        .lines()
        .map(|v| v.parse::<i64>().unwrap() * 811589153)
        .collect::<Vec<i64>>();
    let mixed = mix(list);
    let values = get_values(mixed);
    println!("{:?}", values.into_iter().sum::<i64>());
    Ok(())
}

fn mix(list: Vec<i64>) -> Vec<i64> {
    let mut order = (0..list.len()).into_iter().collect::<Vec<usize>>();
    let mut list = list.into_iter().enumerate().collect::<Vec<(usize, i64)>>();

    for _ in 0..10 {
        for o_i in 0..order.len() {
            let l_i = order[o_i];
            if list[l_i].1 == 0 {
                continue;
            }
            let (_, v) = list.remove(l_i);
            let new_l_i = {
                let n = (l_i as i64 + v) % list.len() as i64;
                if n < 0 {
                    n + list.len() as i64
                } else {
                    n
                }
            } as usize;

            list.insert(new_l_i, (o_i, v));
            order[o_i] = new_l_i;
            if new_l_i < l_i {
                for j in (new_l_i + 1)..=l_i {
                    order[list[j].0] += 1;
                }
            } else {
                for j in l_i..new_l_i {
                    order[list[j].0] -= 1;
                }
            }
        }
    }

    list.into_iter().map(|(_, v)| v).collect()
}

fn get_values(list: Vec<i64>) -> Vec<i64> {
    let (mut i, _) = list.iter().enumerate().find(|(_, &v)| v == 0).unwrap();
    let mut vals = vec![];
    for _ in 0..3 {
        i = (i + 1000) % list.len();
        vals.push(list[i]);
    }
    vals
}
