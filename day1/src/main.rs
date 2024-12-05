use itertools::Itertools;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let (mut v1, mut v2) = load_from_file("input.txt");
    v1.sort();
    v2.sort();

    let sum = v1
        .into_iter()
        .zip(v2)
        .map(|(a, b)| -> i32 { (a - b).abs() })
        .sum::<i32>();

    println!("{}", sum)
}

fn load_from_file(path: &str) -> (Vec<i32>, Vec<i32>) {
    let file = File::open(path).expect("File not found!");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| -> (i32, i32) {
            line.unwrap()
                .split_whitespace()
                .map(|word| word.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip()
}
