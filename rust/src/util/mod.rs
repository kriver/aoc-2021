use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::fmt::Debug;


pub fn load<T>(filename: &str) -> Vec<T>
    where T: FromStr, <T as FromStr>::Err: Debug
{
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}
