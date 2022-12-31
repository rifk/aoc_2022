use eyre::Result;

fn main() -> Result<()> {
    let total: i64 = utils::get_input(25)?.lines().map(snaf_to_dec).sum();
    println!("{:?}", dec_to_snaf(total));
    Ok(())
}

fn snaf_to_dec(snaf: &str) -> i64 {
    let mut dec = 0;
    let mut pos = 1;
    for c in snaf.chars().rev() {
        match c {
            '2' => {
                dec += 2 * pos;
            }
            '1' => {
                dec += pos;
            }
            '0' => {}
            '-' => {
                dec -= pos;
            }
            '=' => {
                dec -= 2 * pos;
            }
            _ => panic!("unexpected char: {c}"),
        }
        pos *= 5;
    }
    dec
}

fn dec_to_snaf(dec: i64) -> String {
    let mut snaf = "1".to_string();
    let mut temp_snaf = "2".to_string();
    while dec >= snaf_to_dec(&temp_snaf) {
        snaf = temp_snaf.clone();
        match &temp_snaf[0..1] {
            "1" => {
                temp_snaf.replace_range(0..1, "2");
            }
            "2" => {
                temp_snaf.replace_range(0..1, "1=");
            }
            _ => panic!(),
        }
    }

    let mut rem = dec - snaf_to_dec(&snaf);
    let mut v = {
        let mut v = 1;
        for _ in 0..snaf.len() as i32 - 2 {
            v *= 5;
        }
        v
    };
    let mut p = 1;
    while rem > 0 {
        if p >= snaf.len() {
            panic!(
                "dec={}, snaf = {}, snaf_to_dec={}, rem={}, v={}",
                dec,
                snaf,
                snaf_to_dec(&snaf),
                rem,
                v
            );
        }
        match rem / v {
            0 => {}
            1 => {
                rem -= v;
                snaf.replace_range(p..p + 1, "-");
            }
            2 => {
                rem -= v * 2;
                snaf.replace_range(p..p + 1, "0");
            }
            3 => {
                rem -= v * 3;
                snaf.replace_range(p..p + 1, "1");
            }
            4 => {
                rem -= v * 4;
                snaf.replace_range(p..p + 1, "2");
            }
            _ => panic!(),
        }
        p += 1;
        v /= 5;
    }

    snaf
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        let p = &[
            (1, "1".to_string()),
            (2, "2".to_string()),
            (3, "1=".to_string()),
            (4, "1-".to_string()),
            (5, "10".to_string()),
            (6, "11".to_string()),
            (7, "12".to_string()),
            (8, "2=".to_string()),
            (9, "2-".to_string()),
            (10, "20".to_string()),
            (15, "1=0".to_string()),
            (20, "1-0".to_string()),
            (2022, "1=11-2".to_string()),
            (12345, "1-0---0".to_string()),
            (314159265, "1121-1110-1=0".to_string()),
        ];
        for (d, s) in p {
            assert_eq!(*d, snaf_to_dec(&s));
            assert_eq!(s, &dec_to_snaf(*d));
        }
    }
}
