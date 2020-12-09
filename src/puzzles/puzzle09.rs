use failure::_core::str::FromStr;
use itertools::Itertools;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let inputs: Vec<u64> = input.lines().map(|l| u64::from_str(l).unwrap()).collect();

    let invalid_number = inputs.windows(26).find(|t| !t[0..25].iter().cartesian_product(t[0..25].iter()).any(|(a, b)| a != b && a + b == t[25])).unwrap()[25];
    answers.push(format!("{}", invalid_number));

    let mut sum = *inputs.get(0).unwrap();
    let mut start = 0;
    let mut end = 0;
    while sum != invalid_number && start < inputs.len() && end + 1 < inputs.len() {
        if sum < invalid_number {  // If we need to add more numbers, extend the selection to the right
            end += 1;
            sum += *inputs.get(end).unwrap();
        } else {  // If we need to remove some numbers, contract the selection from the left
            sum -= *inputs.get(start).unwrap();
            start += 1;
        }
    }
    answers.push(format!("{}", inputs.get(start..=end).unwrap().iter().min().unwrap() +
        inputs.get(start..=end).unwrap().iter().max().unwrap()));


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
