use std::str::FromStr;

use pest::{
    RuleType,
    iterators::{Pair, Pairs},
};

pub struct Unpack<'i, R>(Pairs<'i, R>);

impl<'i, R> From<Pairs<'i, R>> for Unpack<'i, R> {
    fn from(value: Pairs<'i, R>) -> Self {
        Self(value)
    }
}

impl<'i, R: RuleType> Unpack<'i, R> {
    pub fn next(&mut self) -> Pair<'i, R> {
        self.0.next().unwrap()
    }

    pub fn next_str(&mut self) -> &'i str {
        self.next().as_str()
    }

    pub fn next_value<T: FromStr>(&mut self) -> T {
        self.next_str().parse().unwrap_or_else(|_| panic!())
    }

    pub fn next_char(&mut self) -> char {
        self.next_str().chars().next().unwrap()
    }

    pub fn map<T, F: Fn(Pair<'i, R>) -> T>(self, f: F) -> Vec<T> {
        self.0.map(f).collect()
    }
}
