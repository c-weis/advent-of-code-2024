use crate::utils::map2d::grid::Bounds;
use crate::utils::map2d::grid::Grid;
use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader, Lines},
    str::FromStr,
};

use itertools::Itertools;

pub trait HasCharConverter {
    fn convert(c: char) -> Self;
}

impl HasCharConverter for u32 {
    fn convert(c: char) -> Self {
        c.to_digit(10).expect("Error converting digit.")
    }
}

impl HasCharConverter for char {
    fn convert(c: char) -> Self {
        c
    }
}

impl<T: HasCharConverter> From<Vec<String>> for Grid<T> {
    fn from(lines: Vec<String>) -> Self {
        let data = lines
            .iter()
            .map(|line| -> Vec<T> { line.chars().map(|c| -> T { T::convert(c) }).collect_vec() })
            .collect_vec();
        let bounds = Bounds(data[0].len(), data.len());
        Grid { data, bounds }
    }
}

pub fn lines_from_file(path: &str) -> Lines<BufReader<File>> {
    let file = File::open(path).expect("Failed to open file.");
    BufReader::new(file).lines()
}

pub fn strings_from_file(path: &str) -> impl Iterator<Item = String> {
    lines_from_file(path).map(|line| line.unwrap())
}

pub fn two_columns_from_file<T: FromStr>(path: &str) -> (Vec<T>, Vec<T>)
where
    T::Err: Debug,
{
    lines_from_file(path)
        .map(|line| -> (T, T) {
            line.unwrap()
                .split_whitespace()
                .map(|word| word.parse().expect(&format!("Failed to parse: {}.", word)))
                .collect_tuple()
                .expect("Each line must contain exactly two elements.")
        })
        .unzip()
}

pub fn rows_from_file<T: FromStr>(path: &str) -> Vec<Vec<T>>
where
    T::Err: Debug,
{
    lines_from_file(path)
        .map(|line| -> Vec<T> {
            line.unwrap()
                .split_whitespace()
                .map(|word: &str| {
                    word.parse::<T>()
                        .expect(&format!("Failed to parse: {}.", word))
                })
                .collect()
        })
        .collect()
}
