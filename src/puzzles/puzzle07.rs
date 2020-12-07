use regex::{Regex, Captures};
use std::collections::{HashMap, HashSet};
use failure::_core::str::FromStr;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"^(?P<outer>.+?) bags contain (?P<inner>(?:\d .+? bags?,? ?|no other bags)+)\.$").unwrap();
        static ref BAG_RE: Regex = Regex::new(r"(?P<count>\d+) (?P<bag>.+?) bags?,? ?").unwrap();
    }

    let bag_map = {
        let mut m: HashMap<String, Bag> = HashMap::new();
        for l in input.lines() {
            let capture: Captures = LINE_RE.captures(l).unwrap();
            let (outer, inner) = (capture.name("outer").unwrap().as_str(), capture.name("inner").unwrap().as_str());
            m.insert(outer.to_string(), Bag {
                name: outer.to_string(),
                children: BAG_RE.captures_iter(inner).map(|c|
                    (c.name("bag").unwrap().as_str().to_string(),
                     usize::from_str(c.name("count").unwrap().as_str()).unwrap())
                ).collect()
            });
        }
        m
    };

    answers.push(format!("{}", (&bag_map).get(&"shiny gold".to_string()).unwrap().get_unique_ancestors(&bag_map).len()));
    answers.push(format!("{}", (&bag_map).get(&"shiny gold".to_string()).unwrap().count_inner_bags(&bag_map)));

    return answers;
}

#[derive(Debug)]
struct Bag {
    name: String,
    children: Vec<(String, usize)>
}

impl Bag {
    fn get_unique_ancestors(&self, bag_map: &HashMap<String, Bag>) -> HashSet<String> {
        let direct_ancestors: HashSet<String> = bag_map.iter().filter(|(_, b)| b.children.iter().any(|(name, _)| &self.name == name)).map(|(n, _)| n.clone()).collect();
        let mut ancestors= direct_ancestors.clone();
        for a in direct_ancestors {
            ancestors.extend(bag_map.get(a.as_str()).unwrap().get_unique_ancestors(bag_map))
        }
        ancestors
    }

    fn count_inner_bags(&self, bag_map: &HashMap<String, Bag>) -> usize {
        self.children.iter().fold(0, |acc, (name, count)| acc + (count * (bag_map.get(name).unwrap().count_inner_bags(bag_map) + 1)))
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.".to_string();
        assert_eq!(run(input), vec!["4".to_string(), "32".to_string()]);
    }

    #[test]
    fn example2() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.".to_string();
        assert_eq!(run(input), vec!["0".to_string(), "126".to_string()]);
    }
}
