use failure::_core::str::FromStr;
use std::collections::HashSet;
use itertools::Itertools;
use num::FromPrimitive;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();
    let instr: Vec<(&str, i64)> = input.lines().map(|l| {
        match l.split_whitespace().next_tuple().unwrap() {
            ("acc", x) => ("acc", i64::from_str(x).unwrap()),
            ("jmp", x) => ("jmp", i64::from_str(x).unwrap()),
            ("nop", _) => ("nop", 0i64),
            _ => panic!("Invalid instruction `{}`", l)
        }
    }).collect();

    let (acc, seen) = match run_code(&instr) {
        Ok(x) => x,
        Err(x) => panic!(x)
    };
    answers.push(format!("{}", acc));

    for i in seen {
        let new_instr: Vec<(&str, i64)> = instr.iter().enumerate().map(|(idx, (s, v))| {
            if idx == i {
                if s == &"nop" {
                    ("jmp", *v)
                } else {
                    ("nop", *v)
                }
            } else {
                (*s, *v)
            }
        }).collect();
        match run_code(&new_instr) {
            // Find the run which tries to read the instruction immediately after the end
            Err(x) if x.contains(&format!("Tried to read `{}`", instr.len())) => {
                answers.push(format!("{}", x.split("=").last().unwrap()));
                break;
            },
            _ => continue
        }
    }

    return answers;
}

fn run_code(instr: &Vec<(&str, i64)>) -> Result<(i64, HashSet<usize>), String> {
    let mut seen: HashSet<usize> = HashSet::new();
    let mut ptr = 0usize;
    let mut acc = 0i64;

    while !seen.contains(&ptr) {
        seen.insert(ptr);
        match instr.get(ptr) {
            Some(("acc", x)) => {
                acc += x;
                ptr += 1;
            },
            Some(("jmp", x)) => {
                match usize::from_i64(ptr as i64 + *x) {
                    Some(y) => ptr = y,
                    None => return Err(format!("Tried to jump out of bounds from {} by {}", ptr, x))
                };
            },
            Some(("nop", _)) => ptr += 1,
            Some(x) => return Err(format!("Can't execute invalid instruction `{:?}`", x)),
            None => return Err(format!("Out of bounds! Tried to read `{}`. Acc={}", ptr, acc))
        };
    };

    Ok((acc, seen))
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6".to_string();
        assert_eq!(run(input), vec!["5".to_string(), "8".to_string()]);
    }
}
