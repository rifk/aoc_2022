use eyre::Result;
use std::collections::HashSet;

#[derive(Clone, Debug, Default)]
struct Blueprint {
    ore_robot: i32,
    clay_robot: i32,
    obsidian_robot: (i32, i32),
    geode_robot: (i32, i32),
}
#[derive(Clone, Debug, Default, Hash, Eq, PartialEq)]
struct State {
    building: Option<i32>,
    ore: i32,
    ore_robot: i32,
    clay: i32,
    clay_robot: i32,
    obsidian: i32,
    obsidian_robot: i32,
    geode: i32,
    geode_robot: i32,
}
impl State {
    fn new() -> Self {
        Self {
            ore_robot: 1,
            ..Self::default()
        }
    }
    fn finish_build(&mut self) {
        match self.building.take() {
            Some(0) => self.ore_robot += 1,
            Some(1) => self.clay_robot += 1,
            Some(2) => self.obsidian_robot += 1,
            Some(3) => self.geode_robot += 1,
            _ => (),
        }
    }
    fn build_ore_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        if self.ore >= blueprint.ore_robot {
            let mut s = self.clone();
            s.ore -= blueprint.ore_robot;
            s.building = Some(0);
            Some(s)
        } else {
            None
        }
    }
    fn build_clay_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        if self.ore >= blueprint.clay_robot {
            let mut s = self.clone();
            s.ore -= blueprint.clay_robot;
            s.building = Some(1);
            Some(s)
        } else {
            None
        }
    }
    fn build_obsidian_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        if self.ore >= blueprint.obsidian_robot.0 && self.clay >= blueprint.obsidian_robot.1 {
            let mut s = self.clone();
            s.ore -= blueprint.obsidian_robot.0;
            s.clay -= blueprint.obsidian_robot.1;
            s.building = Some(2);
            Some(s)
        } else {
            None
        }
    }
    fn build_geode_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        if self.ore >= blueprint.geode_robot.0 && self.obsidian >= blueprint.geode_robot.1 {
            let mut s = self.clone();
            s.ore -= blueprint.geode_robot.0;
            s.obsidian -= blueprint.geode_robot.1;
            s.building = Some(3);
            Some(s)
        } else {
            None
        }
    }
    fn add_resources(&mut self) {
        self.ore += self.ore_robot;
        self.clay += self.clay_robot;
        self.obsidian += self.obsidian_robot;
        self.geode += self.geode_robot;
    }
}

fn main() -> Result<()> {
    let blueprints = utils::get_input(19)?
        .lines()
        .take(3)
        .map(line_to_blueprint)
        .collect::<Vec<Blueprint>>();
    println!(
        "{:?}",
        blueprints
            .iter()
            .map(|bp| process(32, bp).geode)
            .product::<i32>()
    );
    Ok(())
}

fn process(mins: i32, bp: &Blueprint) -> State {
    let max = (
        bp.ore_robot
            .max(bp.clay_robot)
            .max(bp.obsidian_robot.0)
            .max(bp.geode_robot.0),
        bp.obsidian_robot.1,
        bp.geode_robot.1,
    );
    let mut states = HashSet::new();
    states.insert(State::new());
    let mut max_geode = 0;

    for rem in (1..=mins).rev() {
        println!("{rem}");
        states = states
            .into_iter()
            .flat_map(|mut s| {
                let mut ret = vec![];
                s.finish_build();

                if max_geode >= (s.geode + (s.geode_robot * rem) + (1..=rem).sum::<i32>()) {
                    return ret;
                }

                if let Some(mut s) = s.build_ore_robot(bp) {
                    s.add_resources();
                    max_geode = max_geode.max(s.geode);
                    ret.push(s);
                }
                if let Some(mut sc) = s.build_clay_robot(bp) {
                    sc.add_resources();
                    max_geode = max_geode.max(sc.geode);
                    ret.push(sc);
                }
                if let Some(mut s) = s.build_obsidian_robot(bp) {
                    s.add_resources();
                    max_geode = max_geode.max(s.geode);
                    ret.push(s);
                }
                if let Some(mut s) = s.build_geode_robot(bp) {
                    s.add_resources();
                    max_geode = max_geode.max(s.geode);
                    ret.push(s);
                }
                if max.0 >= s.ore && max.1 >= s.clay && max.2 >= s.obsidian {
                    s.add_resources();
                    max_geode = max_geode.max(s.geode);
                    ret.push(s);
                }

                ret
            })
            .collect();
    }

    states.into_iter().max_by_key(|s| s.geode).unwrap()
}

fn line_to_blueprint(l: &str) -> Blueprint {
    //  Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 8 clay. Each geode robot costs 2 ore and 15 obsidian.
    let (_, l) = l.split_once("Each ore robot costs ").unwrap();
    let (ore, l) = l.split_once(" ore. Each clay robot costs ").unwrap();
    let (clay, l) = l.split_once(" ore. Each obsidian robot costs ").unwrap();
    let (obs1, l) = l.split_once(" ore and ").unwrap();
    let (obs2, l) = l.split_once(" clay. Each geode robot costs ").unwrap();
    let (geo1, l) = l.split_once(" ore and ").unwrap();
    let geo2 = l.strip_suffix(" obsidian.").unwrap();
    Blueprint {
        ore_robot: ore.parse::<i32>().unwrap(),
        clay_robot: clay.parse::<i32>().unwrap(),
        obsidian_robot: (obs1.parse::<i32>().unwrap(), obs2.parse::<i32>().unwrap()),
        geode_robot: (geo1.parse::<i32>().unwrap(), geo2.parse::<i32>().unwrap()),
    }
}
