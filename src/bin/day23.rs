use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use itertools::Itertools;
use rusty_advent_2024::utils::file_io;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Computer(char, char);

#[derive(Debug)]
struct ComputerGraph {
    data: HashMap<Computer, HashSet<Computer>>,
}

impl From<(char, char)> for Computer {
    fn from((c1, c2): (char, char)) -> Self {
        Self(c1, c2)
    }
}

impl ComputerGraph {
    fn from_file(path: &str) -> ComputerGraph {
        let edges: Vec<(Computer, Computer)> = file_io::strings_from_file(path)
            .map(|line: String| -> (Computer, Computer) {
                line.split_once("-")
                    .map(|(str1, str2)| -> (Computer, Computer) {
                        (
                            Computer::from(
                                str1.chars()
                                    .take(2)
                                    .collect_tuple::<(char, char)>()
                                    .expect("Computers should have 2-character names."),
                            ),
                            Computer::from(
                                str2.chars()
                                    .take(2)
                                    .collect_tuple::<(char, char)>()
                                    .expect("Computers should have 2-character names."),
                            ),
                        )
                    })
                    .expect("Computer names should be split by a single dash.")
            })
            .collect_vec();

        let mut graph: HashMap<Computer, HashSet<Computer>> = HashMap::new();
        for (c1, c2) in edges {
            graph.entry(c1).or_insert(HashSet::new()).insert(c2);
            graph.entry(c2).or_insert(HashSet::new()).insert(c1);
        }

        ComputerGraph { data: graph }
    }

    fn find_threeway_games(&self, initial: char) -> HashSet<[Computer; 3]> {
        let possible_computers = self
            .data
            .keys()
            .filter(|Computer(init, _)| init == &initial);

        let mut threeways: HashSet<[Computer; 3]> = HashSet::new();
        for c1 in possible_computers {
            let connected_computers = self.data.get(c1).unwrap();
            for c in connected_computers.into_iter().combinations(2) {
                let (c2, c3) = (c[0], c[1]);
                if self
                    .data
                    .get(c2)
                    .expect(
                        "Every graph node should have its connections recorded in the graph data.",
                    )
                    .contains(c3)
                {
                    let mut threeway = [c1.clone(), c2.clone(), c3.clone()];
                    threeway.sort();
                    threeways.insert(threeway);
                }
            }
        }

        threeways
    }

    fn pruned_bron_kerbosch(
        &self,
        clique: HashSet<Computer>,
        candidates: HashSet<Computer>,
        largest_found: usize,
    ) -> Option<HashSet<Computer>> {
        if clique.len() + candidates.len() <= largest_found {
            // cannot find larger clique here
            return None;
        } else if candidates.is_empty() {
            // unlike in normal bron_kerbosch, we don't need to check if forbiddens is empty here:
            // this would already be handled by the previous if statement
            return Some(clique);
        }

        let mut next_clique: HashSet<Computer> = clique.clone();
        let mut best_clique: Option<HashSet<Computer>> = None;
        let mut future_candidates = candidates.clone();
        for c in candidates {
            let largest_found = best_clique.as_ref().map_or(0, |best| best.len());

            next_clique.insert(c);
            let next_candidates: HashSet<Computer> = future_candidates
                .intersection(self.data.get(&c).unwrap())
                .cloned()
                .collect();
            if let Some(clique) =
                self.pruned_bron_kerbosch(next_clique.clone(), next_candidates, largest_found)
            {
                if clique.len() > largest_found {
                    best_clique = Some(clique);
                }
            }
            next_clique.remove(&c);
            future_candidates.remove(&c);
        }

        best_clique.clone()
    }

    fn largest_clique(&self) -> HashSet<Computer> {
        self.pruned_bron_kerbosch(HashSet::new(), self.data.keys().cloned().collect(), 0)
            .unwrap()
    }
}

fn part1(path: &str) -> usize {
    let graph = ComputerGraph::from_file(path);
    graph.find_threeway_games('t').len()
}

fn part2(path: &str) -> String {
    let graph = ComputerGraph::from_file(path);

    graph
        .largest_clique()
        .drain()
        .map(|computer| -> String { format!("{}{}", computer.0, computer.1).to_string() })
        .sorted()
        .join(",")
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input23.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input23.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/input23.txt.test1"), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/input23.txt.test1"), "co,de,ka,ta");
    }
}
