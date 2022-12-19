use eyre::Result;
use std::collections::{HashMap, HashSet};

static AA: &str = "AA";

#[derive(Debug)]
struct Valve {
    name: String,
    rate: i32,
    conn: HashSet<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Action {
    Moving(i32, usize),
    Opening(usize),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    seen: Vec<bool>,
    action1: Action,
    action2: Action,
    rate: i32,
}

fn main() -> Result<()> {
    let valves = utils::get_input(16)?
        .lines()
        .map(line_to_valve)
        .map(|v| (v.name.clone(), v))
        .collect::<HashMap<String, Valve>>();

    let with_pressure = {
        let mut wp = valves
            .values()
            .filter(|v| v.rate > 0)
            .map(|v| (v.name.clone(), v.rate))
            .collect::<Vec<(String, i32)>>();
        wp.push((AA.to_string(), 0));
        wp
    };
    let rates = with_pressure.iter().map(|(_, r)| *r).collect::<Vec<i32>>();

    let dists = {
        let mut ds = vec![vec![0; with_pressure.len()]; with_pressure.len()];

        for i in 0..(with_pressure.len() - 1) {
            for j in (i + 1)..with_pressure.len() {
                let f = &with_pressure[i].0;
                let t = &with_pressure[j].0;
                let d = find_shortest(f, t, &valves);
                ds[i][j] = d;
                ds[j][i] = d;
            }
        }
        ds
    };
    let shortest = dists.iter().flatten().filter(|&d| *d > 0).min().unwrap();

    let mut routes = HashMap::new();
    routes.insert(
        State {
            seen: {
                let mut s = vec![false; rates.len()];
                s[rates.len() - 1] = true;
                s
            },
            action1: Action::Opening(rates.len() - 1),
            action2: Action::Opening(rates.len() - 1),
            rate: 0,
        },
        0,
    );

    for min in 0..=26 {
        println!("{min}");
        routes = routes
            .into_iter()
            .flat_map(|(k, v)| next_min(k, v, &dists, &rates))
            .fold(HashMap::new(), |mut m, (k, v)| {
                if m.get(&k).map(|r| *r < v).unwrap_or(true) {
                    m.insert(k, v);
                }
                m
            });
        let max_released = routes.iter().map(|(_, v)| v).max().unwrap();
        let mut retained = routes.clone();
        retained.retain(|k, v| {
            let max_rate = k
                .seen
                .iter()
                .enumerate()
                .filter(|(_, &s)| !s)
                .map(|(i, _)| rates[i])
                .max()
                .unwrap_or(0);
            let poss = ((26 - min) / (shortest + 1)) * max_rate;
            (*v + poss) >= *max_released
        });
        routes = retained;
    }

    println!("{:?}", routes.values().max());
    Ok(())
}

fn next_min(state: State, released: i32, dists: &[Vec<i32>], rates: &[i32]) -> Vec<(State, i32)> {
    let mut next = vec![];

    let released = released + state.rate;

    match (&state.action1, &state.action2) {
        (Action::Moving(_, _), Action::Moving(_, _)) => {
            next.push((
                State {
                    action1: next_step(state.action1),
                    action2: next_step(state.action2),
                    ..state
                },
                released,
            ));
        }
        (Action::Opening(i1), Action::Opening(i2)) => {
            let rate = state.rate + rates[*i1] + rates[*i2];
            state
                .seen
                .iter()
                .enumerate()
                .filter(|(_, &s)| !s)
                .for_each(|(n1, _)| {
                    state
                        .seen
                        .iter()
                        .enumerate()
                        .skip(n1 + 1)
                        .filter(|(_, &s)| !s)
                        .for_each(|(n2, _)| {
                            let mut seen = state.seen.clone();
                            seen[n1] = true;
                            seen[n2] = true;
                            let d1 = (dists[*i1][n1], dists[*i2][n2]);
                            let d2 = (dists[*i2][n1], dists[*i1][n2]);
                            if d1.0 <= d2.0 && d1.1 <= d2.1 {
                                next.push((
                                    State {
                                        seen,
                                        action1: Action::Moving(d1.0, n1),
                                        action2: Action::Moving(d1.1, n2),
                                        rate,
                                    },
                                    released,
                                ));
                            } else if d2.0 <= d1.0 && d2.1 <= d1.1 {
                                next.push((
                                    State {
                                        seen,
                                        action1: Action::Moving(d2.0, n1),
                                        action2: Action::Moving(d2.1, n2),
                                        rate,
                                    },
                                    released,
                                ));
                            } else {
                                next.push((
                                    State {
                                        seen: seen.clone(),
                                        action1: Action::Moving(d1.0, n1),
                                        action2: Action::Moving(d1.1, n2),
                                        rate,
                                    },
                                    released,
                                ));
                                next.push((
                                    State {
                                        seen,
                                        action1: Action::Moving(d2.0, n1),
                                        action2: Action::Moving(d2.1, n2),
                                        rate,
                                    },
                                    released,
                                ));
                            }
                        });
                    if next.is_empty() {
                        let mut seen = state.seen.clone();
                        seen[n1] = true;
                        next.push((
                            State {
                                seen,
                                action1: Action::Moving(dists[*i1][n1].min(dists[*i2][n1]), n1),
                                action2: Action::Moving(30, 0),
                                rate,
                            },
                            released,
                        ));
                    }
                });
            if next.is_empty() {
                next.push((
                    State {
                        seen: state.seen,
                        action1: Action::Moving(30, 0),
                        action2: Action::Moving(30, 0),
                        rate,
                    },
                    released,
                ));
            }
        }
        (a1, a2) => {
            let (a, i) = match (a1, a2) {
                (Action::Moving(_, _), Action::Opening(i)) => (next_step(state.action1), i),
                (Action::Opening(i), Action::Moving(_, _)) => (next_step(state.action2), i),
                _ => panic!(),
            };
            let rate = state.rate + rates[*i];
            state
                .seen
                .iter()
                .enumerate()
                .filter(|(_, &s)| !s)
                .for_each(|(n, _)| {
                    let mut seen = state.seen.clone();
                    seen[n] = true;
                    next.push((
                        State {
                            seen,
                            action1: a.clone(),
                            action2: Action::Moving(dists[*i][n], n),
                            rate,
                        },
                        released,
                    ));
                });
            if next.is_empty() {
                next.push((
                    State {
                        seen: state.seen,
                        action1: Action::Moving(30, 0),
                        action2: Action::Moving(30, 0),
                        rate,
                    },
                    released,
                ));
            }
        }
    }

    next
}

fn next_step(action: Action) -> Action {
    match action {
        Action::Moving(s, i) => {
            if s > 1 {
                Action::Moving(s - 1, i)
            } else {
                Action::Opening(i)
            }
        }
        _ => panic!(),
    }
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
