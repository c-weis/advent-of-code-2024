use rusty_advent_2024::maps::*;
use rusty_advent_2024::utils;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input12.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input12.txt"));
}

type Plant = char;
type Field = Map2D<Plant>;
#[derive(Debug)]
struct Plot {
    _plant_type: char,
    plants: HashSet<Position>,
}

impl Plot {
    fn area(&self) -> usize {
        self.plants.len()
    }

    fn perimeter(&self) -> usize {
        self.plants
            .iter()
            .map(|plant| -> usize {
                plant
                    .neighbours()
                    .iter()
                    .filter(|pos| !self.plants.contains(pos))
                    .count()
            })
            .sum()
    }

    // For each Direction, store the positions who have a boundary that way
    fn boundary_map(&self) -> HashMap<Direction, HashSet<Position>> {
        let mut boundary_map: HashMap<Direction, HashSet<Position>> = HashMap::new();

        for direction in Direction::iter_all() {
            boundary_map.insert(
                direction,
                self.plants
                    .iter()
                    .copied()
                    .filter(|pos| !self.plants.contains(&pos.step(&direction)))
                    .collect(),
            );
        }

        boundary_map
    }

    fn sides(&self) -> usize {
        let boundary_map = self.boundary_map();
        let mut sides: HashMap<Direction, usize> = HashMap::new();
        // now find contiguous groups in the boundary_map
        // easier to search as we only go straight, no flooding needed
        for (dir, set) in boundary_map {
            let mut visited: HashSet<Position> = HashSet::new();
            let search_dirs = [dir.turned_left(), dir.turned_right()];
            for pos in &set {
                if !visited.insert(pos.clone()) {
                    continue;
                }

                // explore side
                for search_dir in search_dirs {
                    let mut search_pos = pos.clone();
                    while set.contains(&search_pos) {
                        visited.insert(search_pos);
                        search_pos = search_pos.step(&search_dir);
                    }
                }

                // record side
                *sides.entry(dir).or_insert(0) += 1;
            }
        }

        sides.values().sum()
    }
}

fn find_plots(field: &Field) -> Vec<Plot> {
    let mut recorded_plants: HashSet<Position> = HashSet::new();
    let mut plots: Vec<Plot> = Vec::new();
    for pos in field.position_iter() {
        if recorded_plants.contains(&pos.into()) {
            continue;
        }

        let plot = Plot {
            _plant_type: *field.value(&pos),
            plants: field
                .contiguous_region(&pos)
                .iter()
                .map(|pos| (*pos).into())
                .collect(),
        };

        recorded_plants.extend(plot.plants.iter().copied());
        plots.push(plot);
    }

    plots
}

fn part1(path: &str) -> usize {
    let field: Field = Map2D::from(utils::lines_from_file(path));
    let plots: Vec<Plot> = find_plots(&field);
    plots
        .iter()
        .map(|plot| -> usize { plot.area() * plot.perimeter() })
        .sum()
}

fn part2(path: &str) -> usize {
    let field: Field = Map2D::from(utils::lines_from_file(path));
    let plots: Vec<Plot> = find_plots(&field);
    plots
        .iter()
        .map(|plot| -> usize { plot.area() * plot.sides() })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1("input/input12.txt.test1") == 140);
        assert!(part1("input/input12.txt.test2") == 772);
        assert!(part1("input/input12.txt.test3") == 1930);
    }

    #[test]
    fn test_part2() {
        assert!(part2("input/input12.txt.test1") == 80);
        assert!(part2("input/input12.txt.test2") == 436);
        assert!(part2("input/input12.txt.test4") == 236);
        assert!(part2("input/input12.txt.test5") == 368);
    }
}
