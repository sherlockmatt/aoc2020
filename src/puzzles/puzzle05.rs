use std::collections::HashSet;
use itertools::{Itertools, MinMaxResult::{MinMax, OneElement, NoElements}};

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();
    // The puzzle is set up in such a way as the input string is a binary encoding of the seat ID,
    // but with letters instead of digits
    let seats: HashSet<u16> = input.lines().map(|l| l.chars().map(|c| {
        match c {
            'F' => 0u16,
            'B' => 1u16,
            'L' => 0u16,
            'R' => 1u16,
            x => panic!("Invalid character {}", x)
        }
    }).fold(0u16, |acc, i| (acc << 1) + i)).collect();

    let (min_id, max_id) = match seats.iter().minmax() {
        MinMax(a, b) => (a, b),
        OneElement(a) => (a, a),
        NoElements => panic!("No seats found!")
    };
    answers.push(format!("{}", max_id));

    for (before, id, after) in (*min_id..=*max_id).tuple_windows() {
        if seats.contains(&before) && seats.contains(&after) && !seats.contains(&id) {
            answers.push(format!("{}", id));
            break;
        }
    }

    return answers;
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "FBFBBFFRLR
FBFBBFFLRR".to_string();
        assert_eq!(run(input), vec!["357".to_string(), "356".to_string()]);
    }
}
