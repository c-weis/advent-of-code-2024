use rusty_advent_2024::utils::file_io;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Stripe {
    White,
    Blue,
    Black,
    Red,
    Green,
}

type Pattern = Vec<Stripe>;
type SubPattern<'a> = &'a [Stripe];

struct PatternTrieNode {
    is_end_of_pattern: bool,
    children: HashMap<Stripe, PatternTrieNode>,
}

struct PatternTrie {
    root: PatternTrieNode,
}

impl PatternTrieNode {
    fn new(is_end_of_pattern: bool) -> Self {
        PatternTrieNode {
            is_end_of_pattern,
            children: HashMap::new(),
        }
    }
}

impl PatternTrie {
    fn new() -> Self {
        PatternTrie {
            root: PatternTrieNode::new(true),
        }
    }

    fn from(patterns: &[Pattern]) -> Self {
        let mut trie = PatternTrie::new();
        for pattern in patterns {
            trie.insert(pattern);
        }
        trie
    }

    fn insert(&mut self, pattern: SubPattern) {
        let mut node = &mut self.root;
        for &stripe in pattern {
            node = node
                .children
                .entry(stripe)
                .or_insert(PatternTrieNode::new(false))
        }
        node.is_end_of_pattern = true;
    }

    fn contains(&self, pattern: SubPattern) -> bool {
        let mut node = &self.root;
        for stripe in pattern {
            match node.children.get(stripe) {
                Some(child_node) => node = child_node,
                None => return false,
            }
        }
        node.is_end_of_pattern
    }

    fn can_make(&self, pattern: SubPattern) -> bool {
        if self.contains(pattern) {
            return true;
        }
        if pattern.len() == 1 {
            return false;
        }

        (1..pattern.len()).any(|i| self.contains(&pattern[i..]) && self.can_make(&pattern[..i]))
    }

    fn ways_to_make(&self, pattern: SubPattern) -> usize {
        let mut cache = HashMap::new();
        self.cached_ways_to_make(pattern, &mut cache)
    }

    fn cached_ways_to_make(
        &self,
        pattern: SubPattern,
        cache: &mut HashMap<Pattern, usize>,
    ) -> usize {
        if let Some(&stored_number) = cache.get(pattern) {
            return stored_number;
        }

        if pattern.len() <= 1 {
            return self.contains(&pattern).into();
        }

        let ways_to_make = (1..=pattern.len())
            .map(|i| pattern.split_at(i))
            .filter_map(|(left, right)| {
                self.contains(left)
                    .then_some(self.cached_ways_to_make(right, cache))
            })
            .sum();

        cache.insert(pattern.to_vec(), ways_to_make);
        return ways_to_make;
    }
}

impl From<char> for Stripe {
    fn from(c: char) -> Self {
        match c {
            'w' => Self::White,
            'u' => Self::Blue,
            'b' => Self::Black,
            'r' => Self::Red,
            'g' => Self::Green,
            _ => panic!("Invalid character for parsing stripe."),
        }
    }
}

fn pattern_from_word(word: &str) -> Pattern {
    word.trim()
        .chars()
        .map(|c| -> Stripe { c.into() })
        .collect()
}

fn load_input(path: &str) -> (PatternTrie, Vec<Pattern>) {
    let mut lines = file_io::strings_from_file(path);

    let towels: Vec<Pattern> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|word| -> Pattern { pattern_from_word(word) })
        .collect();

    let towel_trie: PatternTrie = PatternTrie::from(&towels);

    let designs: Vec<Pattern> = lines
        .filter(|line| !line.is_empty())
        .map(|line| pattern_from_word(&line))
        .collect();

    (towel_trie, designs)
}

fn part1(path: &str) -> usize {
    let (towel_trie, designs) = load_input(path);

    designs
        .iter()
        .filter(|design| towel_trie.can_make(design))
        .count()
}

fn part2(path: &str) -> usize {
    let (towel_trie, designs) = load_input(path);

    designs
        .iter()
        .map(|design| towel_trie.ways_to_make(design))
        .sum()
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input19.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input19.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trie_from_string(pattern_string: &str) -> PatternTrie {
        let patterns: Vec<Pattern> = pattern_string
            .split(",")
            .map(|word| -> Pattern { pattern_from_word(word) })
            .collect();

        let mut trie = PatternTrie::new();

        for pattern in patterns {
            trie.insert(&pattern);
        }

        trie
    }

    #[test]
    fn test_trie() {
        let mut trie = PatternTrie::new();

        let empty = &pattern_from_word("");
        let b = &pattern_from_word("b");
        let w = &pattern_from_word("w");
        let r = &pattern_from_word("r");
        let bw = &pattern_from_word("bw");
        let wr = &pattern_from_word("wr");
        let br = &pattern_from_word("br");
        let bwr = &pattern_from_word("bwr");

        assert!(trie.contains(empty));
        for p in [b, w, r, bw, wr, br, bwr] {
            assert!(!trie.contains(p));
        }

        trie.insert(bw);
        assert!(trie.contains(bw));
        for p in [b, w, r, wr, br, bwr] {
            assert!(!trie.contains(p));
        }

        trie.insert(bwr);
        assert!(trie.contains(bw));
        assert!(trie.contains(bwr));
        for p in [b, w, r, wr, br] {
            assert!(!trie.contains(p));
        }
    }

    #[test]
    fn test_can_make() {
        let trie = trie_from_string("g, u, bw, brb, rr");

        for word in ["gu", "bwu", "brb", "bwrr", "brbrrgubw"] {
            assert!(
                trie.can_make(&pattern_from_word(word)),
                "Should be able to make '{word}'."
            );
        }

        for word in ["bgu", "gurb"] {
            assert!(
                !trie.can_make(&pattern_from_word(word)),
                "Should not be able to make '{word}'."
            )
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/input19.txt.test1"), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/input19.txt.test1"), 16);
    }
}
