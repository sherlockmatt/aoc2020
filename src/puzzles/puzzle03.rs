use crate::utils::Pos;
use std::collections::{HashSet, HashMap};

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();
    let inputs: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let width = inputs.get(0usize).unwrap().len();
    let height = inputs.len();

    let mut map: HashSet<Pos> = HashSet::new();
    for (y, line) in inputs.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert(Pos{x, y});
            }
        }
    }

    let mut slopes: HashMap<Pos, usize> = [(Pos{x:1, y:1}, 0), (Pos{x:1, y:3}, 0), (Pos{x:1, y:5}, 0), (Pos{x:1, y:7}, 0), (Pos{x:2, y:1}, 0)].iter().cloned().collect();
    for (slope, trees) in slopes.iter_mut() {
        for y in 0..height {
            if y % slope.x == 0 {  // Only check rows for which the slope is on an integer position
                let x = ((slope.y * y) / slope.x) % width;  // This divide will always produce an integer, because we can only get here when y % s.x == 0
                if map.contains(&Pos { x, y }) {
                    *trees += 1;
                }
            }
        }
    }

    answers.push(format!("{}", slopes.get(&Pos{x:1, y:3}).unwrap()));
    answers.push(format!("{}", slopes.values().cloned().product::<usize>()));

    return answers;
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#".to_string();
        assert_eq!(run(input), vec!["7".to_string(), "336".to_string()]);
    }
}
