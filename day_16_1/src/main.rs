use eyre::Result;
use std::collections::{HashMap, HashSet};

static AA: &str = "AA";

#[derive(Debug)]
struct Valve {
    name: String,
    rate: i32,
    conn: HashSet<String>,
}

fn main() -> Result<()> {
    let valves = utils::get_input(16)?
        .lines()
        .map(line_to_valve)
        .map(|v| (v.name.clone(), v))
        .collect::<HashMap<String, Valve>>();

    let with_pressure = valves
        .values()
        .filter(|v| v.rate > 0)
        .map(|v| (v.name.clone(), v.rate))
        .collect::<Vec<(String, i32)>>();

    let shortest = {
        let mut s = HashMap::new();
        with_pressure.iter().map(|(to, _)| to).for_each(|to| {
            s.insert(
                (AA.to_string(), to.to_string()),
                find_shortest(AA, to, &valves),
            );
        });

        for i in 0..(with_pressure.len() - 1) {
            for j in (i + 1)..with_pressure.len() {
                let f = &with_pressure[i].0;
                let t = &with_pressure[j].0;
                let d = find_shortest(f, t, &valves);
                s.insert((f.to_string(), t.to_string()), d);
                s.insert((t.to_string(), f.to_string()), d);
            }
        }
        s
    };

    let mut routes = HashMap::new();
    routes.insert((AA.to_string(), 0, vec![false; with_pressure.len()]), 0);
    let mut cont = true;
    while cont {
        cont = false;
        let mut new_routes = HashMap::new();

        for ((cur, min, seen), rel) in routes.drain() {
            let mut progress = false;
            for i in 0..seen.len() {
                if min >= 28 || seen[i] {
                    continue;
                }
                let (to, rate) = &with_pressure[i];
                let dist = shortest.get(&(cur.clone(), to.to_string())).unwrap();
                let min = min + dist + 1;
                let rel = if min < 30 {
                    rel + ((30 - min) * rate)
                } else {
                    rel
                };
                let mut seen = seen.clone();
                seen[i] = true;
                let k = (to.to_string(), min, seen);
                if new_routes.get(&k).map(|r| *r < rel).unwrap_or(true) {
                    new_routes.insert(k, rel);
                    progress = true;
                }
            }
            if !progress {
                let k = (cur, min, seen);
                if new_routes.get(&k).map(|r| *r < rel).unwrap_or(true) {
                    new_routes.insert(k, rel);
                }
            } else {
                cont = true;
            }
        }

        routes = new_routes;
    }

    println!("{:?}", routes.values().max());
    Ok(())
}

fn find_shortest(from: &str, to: &str, valves: &HashMap<String, Valve>) -> i32 {
    let mut steps = 0;
    let mut seen = HashSet::new();
    let mut next = HashSet::new();
    next.insert(from.to_string());
    seen.insert(from.to_string());
    let mut found = false;
    while !found {
        steps += 1;
        next = next
            .into_iter()
            .flat_map(|f| {
                let v = valves.get(&f).unwrap();
                let mut n = vec![];
                for t in &v.conn {
                    if found {
                        break;
                    }
                    if t == to {
                        found = true;
                    } else if seen.insert(t.to_string()) {
                        n.push(t.to_string());
                    }
                }
                n
            })
            .collect();
    }
    steps
}

fn line_to_valve(l: &str) -> Valve {
    let l = l.strip_prefix("Valve ").unwrap();
    let (name, l) = l.split_once(" has flow rate=").unwrap();
    let (rate, conn) = if let Some((rate, l)) = l.split_once("; tunnels lead to valves ") {
        (rate, l.split(", ").map(|c| c.to_string()).collect())
    } else {
        let (rate, conn) = l.split_once("; tunnel leads to valve ").unwrap();
        (rate, {
            let mut s = HashSet::new();
            s.insert(conn.to_string());
            s
        })
    };

    Valve {
        name: name.to_string(),
        rate: rate.parse::<i32>().unwrap(),
        conn,
    }
}
