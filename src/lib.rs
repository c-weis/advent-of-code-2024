pub mod utils {
    use std::{
        fmt::Debug,
        fs::File,
        io::{BufRead, BufReader, Lines},
        str::FromStr,
    };

    use itertools::Itertools;

    pub fn lines_from_file(path: &str) -> Lines<BufReader<File>> {
        let file = File::open(path).expect("Failed to open file.");
        BufReader::new(file).lines()
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
                    .collect_vec()
            })
            .collect_vec()
    }
}
