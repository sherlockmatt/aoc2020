use failure::_core::str::FromStr;
use std::collections::HashSet;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();
    let mut inputs: Vec<u32> = input.lines().map(|i| u32::from_str(i).unwrap()).collect();
    inputs.sort();

    // Add the implied zero at the start
    inputs.insert(0, 0);

    // There's only one way to traverse a gap of 3, so we can solve each "chunk" of the problem separately
    let mut chunks: Vec<Vec<u32>> = Vec::new();
    let mut current_chunk: Vec<u32> = Vec::new();
    for i in inputs {
        if current_chunk.len() == 0 {
            current_chunk.push(i);
        } else {
            match i - *current_chunk.iter().last().unwrap() {
                1..=2 => current_chunk.push(i),
                3 => {
                    chunks.push(current_chunk.clone());
                    current_chunk = vec![i];
                },
                _ => panic!("Oversized jump found: {} -> {}", current_chunk.iter().last().unwrap(), i)
            };
        }
    }
    chunks.push(current_chunk);

    answers.push(format!("{}", (chunks.iter().map(|c| 2 * (c.len() - 1) - ((c.last().unwrap() - c.first().unwrap()) as usize)).sum::<usize>()) * chunks.len()));

    // Calculate the number of ways we can traverse each chunk, and multiply them together for our answer
    answers.push(format!("{}", chunks.iter().map(|c| {
        match c.len() {
            1..=2 => 1,
            _ => {
                let mut paths: HashSet<Vec<u32>> = HashSet::new();
                let new_path = vec![*c.first().unwrap()];
                paths.insert(new_path);
                for i in c.iter().skip(1) {
                    let mut new_paths: HashSet<Vec<u32>> = HashSet::new();
                    for p in paths {
                        for d in 1..=3 {
                            if *i >= d && (&p).contains(&(i - d)) {
                                new_paths.insert(p.clone());
                                let mut new_path = p.clone();
                                new_path.push(*i);
                                new_paths.insert(new_path);
                            }
                        }
                    }
                    paths = new_paths;
                }
                paths.iter().filter(|p| p.contains(c.last().unwrap())).count()
            }
        }
    }).product::<usize>()));

    return answers;
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3".to_string();
        assert_eq!(run(input), vec!["220".to_string(), "19208".to_string()]);
    }
}
