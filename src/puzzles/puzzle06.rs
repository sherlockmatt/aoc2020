use std::collections::HashSet;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    answers.push(format!("{}", input.split("\n\n").map(|s| s.chars().filter(|c| ('a'..='z').contains(c)).collect::<HashSet<char>>().len()).sum::<usize>()));
    answers.push(format!("{}", input.split("\n\n").map(|s| ('a'..='z').filter(|c| s.lines().all(|l| l.chars().any(|i| *c == i))).count()).sum::<usize>()));

    return answers;
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b".to_string();
        assert_eq!(run(input), vec!["11".to_string(), "6".to_string()]);
    }
}
