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
