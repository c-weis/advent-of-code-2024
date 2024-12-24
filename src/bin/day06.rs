use std::{collections::HashSet, hash::Hash};
use rusty_advent_2024::utils::{file_io::lines_from_file, map2d::{direction::Direction, grid::Bounds, position::Position}};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Guard {
    pos: Position,
    dir: Direction,
}

impl MazeState {
    fn in_bounds(&self, position: &Position) -> bool {
        position.0 >= 0
            && position.1 >= 0
            && position.0 < self.bounds.0 as i32
            && position.1 < self.bounds.1 as i32
    }

    fn step_guard(self: &mut Self) -> Option<Position> {
        let next_pos = self.guard.pos.step(&self.guard.dir);

        if self.obstacles.contains(&next_pos) {
            self.guard.dir.turn_right();
            return Some(self.guard.pos.clone());
        }

        if self.in_bounds(&next_pos) {
            self.guard.pos = next_pos;
            Some(next_pos)
        } else {
            None
        }
    }
}

struct MazeState {
    guard: Guard,
    obstacles: HashSet<Position>,
    bounds: Bounds,
}

fn read_maze(path: &str) -> MazeState {
    let mut guard: Guard = Guard {
        pos: Position(0, 0),
        dir: Direction::UP,
    };
    let mut obstacles: HashSet<Position> = HashSet::new();
    let mut bounds: Bounds = Bounds(0, 0);
    for (y, line) in lines_from_file(path).into_iter().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            match c {
                '#' => {
                    obstacles.insert(Position(x as i32, y as i32));
                }
                '^' | '>' | 'v' | '<' => {
                    guard = Guard {
                        pos: Position(x as i32, y as i32),
                        dir: c.into(),
                    }
                }
                _ => {}
            }
            bounds = Bounds(x + 1, y + 1);
        }
    }

    MazeState {
        guard,
        obstacles,
        bounds,
    }
}

fn get_visited_positions(maze: &mut MazeState) -> HashSet<Position> {
    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(maze.guard.pos);

    while let Some(new_pos) = maze.step_guard() {
        visited.insert(new_pos);
    }

    visited
}

fn creates_loop(maze: &mut MazeState, obstacle: Position) -> bool {
    let guard_start = maze.guard;
    maze.obstacles.insert(obstacle);

    let mut visited_guard_states: HashSet<Guard> = HashSet::new();
    visited_guard_states.insert(maze.guard);

    let mut creates_loop: bool = false;

    while let Some(_) = maze.step_guard() {
        if !visited_guard_states.insert(maze.guard) {
            creates_loop = true;
            break;
        }
    }

    maze.obstacles.remove(&obstacle);
    maze.guard = guard_start;

    creates_loop
}

fn part1(path: &str) -> usize {
    let mut maze = read_maze(path);
    get_visited_positions(&mut maze).len()
}

fn part2(path: &str) -> usize {
    let mut maze = read_maze(path);
    let guard_start = maze.guard;
    let obstacle_candidates = get_visited_positions(&mut maze);
    maze.guard = guard_start;

    obstacle_candidates
        .iter()
        .filter(|&&obstacle| creates_loop(&mut maze, obstacle))
        .count()
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input06.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input06.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1("input/input06.txt.test1") == 41);
    }

    #[test]
    fn test_part2() {
        assert!(part2("input/input06.txt.test1") == 6);
    }
}
