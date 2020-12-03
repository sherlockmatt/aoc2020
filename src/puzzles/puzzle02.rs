use failure::_core::str::FromStr;
use itertools::Itertools;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();
    let mut p1 = 0u32;
    let mut p2 = 0u32;

    for l in input.lines() {
        let (spec, c, p): (String, String, String) = l.split_whitespace().map(|i| i.replace(":", "")).next_tuple().unwrap();
        let (start, end): (usize, usize) = spec.split("-").map(|i| usize::from_str(i).unwrap()).next_tuple().unwrap();

        if start == 0 { panic!("Start cannot be 0: {}", l); }
        if end == 0 { panic!("End cannot be 0: {}", l); }

        if (start .. end + 1).contains(&p.matches(&c).count()) {
            p1 += 1;
        }
        if (&p.chars().nth(start - 1).unwrap() == &char::from_str(&c).unwrap())
            ^ (&p.chars().nth(end - 1).unwrap() == &char::from_str(&c).unwrap()) {
            p2 += 1;
        }
    };

    answers.push(format!("{}", p1));
    answers.push(format!("{}", p2));

    return answers;
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc".to_string();
        assert_eq!(run(input), vec!["2".to_string(), "1".to_string()]);
    }
}
