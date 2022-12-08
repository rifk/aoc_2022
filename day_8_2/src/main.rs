use eyre::Result;

static C0: i32 = '0' as i32;

fn main() -> Result<()> {
    let heights = utils::get_input(8)?
        .lines()
        .map(|l| l.chars().map(|h| h as i32 - C0).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let n = heights.len();
    let m = heights[0].len();
    let mut scores = vec![vec![1; m]; n];

    for i in 1..n - 1 {
        // look left
        let mut view = vec![0; m];
        for j in 1..(m - 1) {
            let mut temp_j = j - 1;
            while temp_j > 0 && heights[i][j] > heights[i][temp_j] {
                temp_j -= view[temp_j];
            }
            view[j] = j - temp_j;
            scores[i][j] *= view[j];
        }
        // look right
        view = vec![0; m];
        for j in (1..(m - 1)).rev() {
            let mut temp_j = j + 1;
            while temp_j < m - 1 && heights[i][j] > heights[i][temp_j] {
                temp_j += view[temp_j];
            }
            view[j] = temp_j - j;
            scores[i][j] *= view[j];
        }
    }
    for j in 1..m - 1 {
        // look up
        let mut view = vec![0; n];
        for i in 1..(n - 1) {
            let mut temp_i = i - 1;
            while temp_i > 0 && heights[i][j] > heights[temp_i][j] {
                temp_i -= view[temp_i];
            }
            view[i] = i - temp_i;
            scores[i][j] *= view[i];
        }
        // look down
        view = vec![0; n];
        for i in (1..(n - 1)).rev() {
            let mut temp_i = i + 1;
            while temp_i < n - 1 && heights[i][j] > heights[temp_i][j] {
                temp_i += view[temp_i];
            }
            view[i] = temp_i - i;
            scores[i][j] *= view[i];
        }
    }

    println!("{:?}", scores.iter().flat_map(|v| v.iter()).max());

    Ok(())
}
