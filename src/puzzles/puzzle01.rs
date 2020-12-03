use failure::_core::str::FromStr;
use std::collections::HashSet;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();
    let mut inputs: Vec<i32> = input.lines().map(|i| i32::from_str(i).unwrap()).collect();

    // PART 1
    let mut seen: HashSet<i32> = HashSet::new();
    // All pairs of numbers that sum to 2020 have unique products, so we don't need to actually find the other number
    for x in &inputs {
        let prod = x * (2020 - x);
        if seen.contains(&prod) {
            answers.push(format!("{}", prod));
            break;
        } else {
            seen.insert(prod);
        }
    };

    // PART 2
    inputs.sort();
    // Check every combination, but stop early if we've progressed past the target sum
    'x: for x in &inputs {
        'y: for y in &inputs {
            let s = x + y;
            if s >= 2020 {
                break 'y;
            } else {
                'z: for z in &inputs {
                    let s2 = s + z;
                    if s2 > 2020 {
                        break 'z;
                    } else if s2 == 2020 {
                        answers.push(format!("{}", x * y * z));
                        break 'x;
                    }
                }
            }
        }
    }

    return answers;
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "1721
979
366
299
675
1456".to_string();
        assert_eq!(run(input), vec!["514579".to_string(), "241861950".to_string()]);
    }
}
