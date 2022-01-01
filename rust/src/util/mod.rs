use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::AddAssign;


pub fn load<T>(filename: &str) -> Vec<T>
    where T: FromStr, <T as FromStr>::Err: Debug
{
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}

// Copy of Itertools.counts but with a generic count type
pub trait Frequencies: Iterator {
    fn frequencies<T>(self) -> HashMap<Self::Item, T>
        where
            Self: Sized,
            Self::Item: Eq + Hash,
            T: Default + AddAssign + From<u32>,
    {
        let mut counts = HashMap::new();
        self.for_each(|item| *counts.entry(item).or_default() += T::from(1));
        counts
    }
}

impl<I: Iterator> Frequencies for I {}
