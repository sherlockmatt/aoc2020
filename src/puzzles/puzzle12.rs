use crate::utils::Vector;
use itertools::Itertools;
use failure::_core::str::FromStr;

const NORTH: Vector = Vector { dx:  0, dy:  1 };
const EAST : Vector = Vector { dx:  1, dy:  0 };
const SOUTH: Vector = Vector { dx:  0, dy: -1 };
const WEST : Vector = Vector { dx: -1, dy:  0 };

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let mut dir = EAST.clone();
    let mut pos = Vector { dx: 0, dy: 0 };
    for l in input.lines() {
        let v = usize::from_str(&l.chars().skip(1).join("")).unwrap();
        match l.chars().nth(0).unwrap() {
            'F' => pos += dir * v,
            'N' => pos += NORTH * v,
            'E' => pos += EAST * v,
            'S' => pos += SOUTH * v,
            'W' => pos += WEST * v,
            'L' => (0..(v / 90)).for_each(|_| dir.rotate_ccw()),
            'R' => (0..(v / 90)).for_each(|_| dir.rotate_cw()),
            x => panic!("Invalid operation `{}`", x)
        };
    }
    answers.push(format!("{}", pos.distance_to(&Vector::new(0, 0))));

    pos = Vector { dx: 0, dy: 0 };
    let mut wp = Vector { dx: 10, dy: 1 };
    for l in input.lines() {
        let v = usize::from_str(&l.chars().skip(1).join("")).unwrap();
        match l.chars().nth(0).unwrap() {
            'F' => pos += wp * v,
            'N' => wp += NORTH * v,
            'E' => wp += EAST * v,
            'S' => wp += SOUTH * v,
            'W' => wp += WEST * v,
            'L' => (0..(v / 90)).for_each(|_| wp.rotate_ccw()),
            'R' => (0..(v / 90)).for_each(|_| wp.rotate_cw()),
            x => panic!("Invalid operation `{}`", x)
        };
    }
    answers.push(format!("{}", pos.distance_to(&Vector::new(0, 0))));

    return answers;
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "F10
N3
F7
R90
F11".to_string();
        assert_eq!(run(input), vec!["25".to_string(), "286".to_string()]);
    }

    #[test]
    fn test_rotation() {
        let input = "L90
L90
L90
L90
L180
L90
L180
L270
L360
R90
R90
R90
R90
R180
R90
R180
R270
R360".to_string();
        assert_eq!(run(input), vec!["0".to_string(), "0".to_string()]);
    }
}
