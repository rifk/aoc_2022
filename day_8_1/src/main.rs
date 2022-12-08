use eyre::Result;

static C0: i8 = '0' as i8;

fn main() -> Result<()> {
    let heights = utils::get_input(8)?
        .lines()
        .map(|l| l.chars().map(|h| h as i8 - C0).collect::<Vec<i8>>())
        .collect::<Vec<Vec<i8>>>();

    let n = heights.len();
    let m = heights[0].len();
    let mut can_see = vec![vec![false; m]; n];

    can_see[0][0] = true;
    can_see[n - 1][0] = true;
    can_see[0][m - 1] = true;
    can_see[n - 1][m - 1] = true;

    for i in 1..n - 1 {
        update_row(i, m, &mut can_see, &heights);
    }
    for j in 1..m - 1 {
        update_col(j, n, &mut can_see, &heights);
    }

    println!("{:?}", can_see.iter().flatten().filter(|see| **see).count());
    Ok(())
}

fn update_row(row: usize, col_len: usize, can_see: &mut [Vec<bool>], height: &[Vec<i8>]) {
    let ranges = {
        let range = 0..col_len;
        let v: Vec<Box<dyn Iterator<Item = usize>>> =
            vec![Box::new(range.clone()), Box::new(range.rev())];
        v
    };
    for range in ranges {
        let mut max = -1;
        for j in range {
            if height[row][j] > max {
                can_see[row][j] = true;
                max = height[row][j];
            }
        }
    }
}

fn update_col(col: usize, row_len: usize, can_see: &mut [Vec<bool>], height: &[Vec<i8>]) {
    let ranges = {
        let range = 0..row_len;
        let v: Vec<Box<dyn Iterator<Item = usize>>> =
            vec![Box::new(range.clone()), Box::new(range.rev())];
        v
    };
    for range in ranges {
        let mut max = -1;
        for i in range {
            if height[i][col] > max {
                can_see[i][col] = true;
                max = height[i][col];
            }
        }
    }
}
