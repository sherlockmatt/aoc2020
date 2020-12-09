use failure::_core::str::FromStr;
use itertools::Itertools;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let inputs: Vec<u64> = input.lines().map(|l| u64::from_str(l).unwrap()).collect();

    let invalid_number = inputs.windows(26).find(|t| !t[0..25].iter().cartesian_product(t[0..25].iter()).any(|(a, b)| a != b && a + b == t[25])).unwrap()[25];
    answers.push(format!("{}", invalid_number));

    for start_number in 0..inputs.len() {
        let mut sum = *inputs.get(start_number).unwrap();
        let mut end_number = start_number;
        while sum < invalid_number && end_number + 1 < inputs.len() {
            end_number += 1;
            sum += *inputs.get(end_number).unwrap();
        }
        if sum == invalid_number {
            answers.push(format!("{}", inputs.get(start_number..=end_number).unwrap().iter().min().unwrap() +
                inputs.get(start_number..=end_number).unwrap().iter().max().unwrap()));
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
        let input = "1
2
3
4
5
6
7
8
9
10
11
14
16
12
15
13
17
18
19
20
21
22
23
24
25
26
49
100".to_string();
        assert_eq!(run(input), vec!["100".to_string(), "25".to_string()]);
    }
}
