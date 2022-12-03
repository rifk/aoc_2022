use eyre::Result;

const R_W: &str = "C X";
const R_L: &str = "B X";
const R_D: &str = "A X";
const P_W: &str = "A Y";
const P_L: &str = "C Y";
const P_D: &str = "B Y";
const S_W: &str = "B Z";
const S_L: &str = "A Z";
const S_D: &str = "C Z";

fn main() -> Result<()> {
    let total = utils::get_input(2)?.lines().map(points).sum::<i32>();

    println!("TOTAL = {:?}", total);

    Ok(())
}

fn points(line: &str) -> i32 {
    match line {
        R_W => 7,
        R_L => 1,
        R_D => 4,
        P_W => 8,
        P_L => 2,
        P_D => 5,
        S_W => 9,
        S_L => 3,
        S_D => 6,
        other => panic!("unknown line {}", other),
    }
}
