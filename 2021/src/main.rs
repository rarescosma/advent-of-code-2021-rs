use aoc_prelude::*;

use regex::Regex;
use std::process::Command;

lazy_static! {
    static ref TIME_REGEX: Regex = Regex::new(r"Time: (\d+)ms").unwrap();
}

fn extract_time(s: &str) -> u32 {
    let capture = TIME_REGEX.captures_iter(s).next().unwrap();
    capture[1].parse().unwrap()
}

fn main() {
    let dot_dir = std::env::current_exe()
        .map(|x| x.parent().unwrap().to_owned())
        .expect("cannot get current exe");

    let total_time = (1..=25)
        .map(|day_num| {
            let cmd = Command::new(dot_dir.join(format!("day{:0>2}", day_num)))
                .output()
                .unwrap();
            let output = String::from_utf8(cmd.stdout).unwrap();
            println!("Day {}:\n{}", day_num, output);
            extract_time(&output)
        })
        .sum::<u32>();
    println!("Total time: {}ms", total_time);
}
