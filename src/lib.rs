use once_cell::sync::Lazy;
use regex::Regex;
use std::env;
use std::fs;
use std::path::PathBuf;

static RE_DIGITS: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

pub fn input_path(day: u8, example: bool) -> PathBuf {
    let file = if example {
        format!("day{:02}-example.txt", day)
    } else {
        format!("day{:02}.txt", day)
    };

    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("inputs")
        .join(file)
}

pub fn read_input(day: u8) -> String {
    let filename = input_path(day, false);
    fs::read_to_string(&filename).expect(&format!(
        "failed to read input file: {}",
        filename.display()
    ))
}

pub fn read_example(day: u8) -> String {
    let filename = input_path(day, true);
    fs::read_to_string(&filename).expect(&format!(
        "failed to read example input file: {}",
        filename.display()
    ))
}

pub fn extract_day_from_exe() -> u8 {
    let name = env::current_exe()
        .ok()
        .and_then(|p| p.file_name().map(|s| s.to_os_string()))
        .unwrap_or_default();

    match name.into_string() {
        Ok(s) => {
            let num_str = RE_DIGITS.find(&s);
            match num_str {
                Some(num) => num.as_str().parse().unwrap(),
                None => 0,
            }
        }
        Err(_os) => 0,
    }
}
