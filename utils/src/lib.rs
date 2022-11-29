use eyre::{bail, eyre, Result};
use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use std::env;
use std::fs;

pub fn get_input(day: i32) -> Result<String> {
    let mut args = env::args();
    args.next();
    if let Some(file) = args.next() {
        Ok(fs::read_to_string(file)?)
    } else if let Some(session) = env::var_os("AOC_SESSION") {
        let client = Client::new();
        Ok(client
            .get(format!("https://adventofcode.com/2022/day/{}/input", day))
            .header(
                COOKIE,
                format!(
                    "session={}",
                    session
                        .to_str()
                        .ok_or_else(|| eyre!("cannot convert env to str"))?
                ),
            )
            .send()?
            .text()?)
    } else {
        bail!("no input file provided or AOC_SESSION set");
    }
}
