use regex::Regex;
use itertools::Itertools;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();
    let mut passports: Vec<Passport> = Vec::new();

    let mut cur_passport = Passport::new();
    for l in input.lines().map(|l| l.trim()) {
        if l.is_empty() {
            passports.push(cur_passport);
            cur_passport = Passport::new();
        } else {
            for field in l.split_whitespace() {
                let (k, v) = field.split(":").next_tuple().unwrap();
                match k {
                    "byr" => cur_passport.byr = v.to_string(),
                    "iyr" => cur_passport.iyr = v.to_string(),
                    "eyr" => cur_passport.eyr = v.to_string(),
                    "hgt" => cur_passport.hgt = v.to_string(),
                    "hcl" => cur_passport.hcl = v.to_string(),
                    "ecl" => cur_passport.ecl = v.to_string(),
                    "pid" => cur_passport.pid = v.to_string(),
                    "cid" => cur_passport.cid = v.to_string(),
                    x => panic!("Unknown field `{}`", x)
                }
            }
        }
    }
    passports.push(cur_passport);

    let valid: Vec<Passport> = passports.into_iter().filter(|p| p.is_valid()).collect();
    answers.push(format!("{}", valid.len()));
    answers.push(format!("{}", valid.into_iter().filter(|p| p.is_correct()).count()));

    return answers;
}

#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String
}

lazy_static! {
    static ref RE_BYR: Regex = Regex::new("^(19[2-9][0-9]|200[0-2])$").unwrap();
    static ref RE_IYR: Regex = Regex::new("^20(1[0-9]|20)$").unwrap();
    static ref RE_EYR: Regex = Regex::new("^20(2[0-9]|30)$").unwrap();
    static ref RE_HGT: Regex = Regex::new("^(1([5-8][0-9]|9[0-3])cm|(59|6[0-9]|7[0-6])in)$").unwrap();
    static ref RE_HCL: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
    static ref RE_ECL: Regex = Regex::new("^(amb|b(lu|rn)|g(ry|rn)|hzl|oth)$").unwrap();
    static ref RE_PID: Regex = Regex::new("^[0-9]{9}$").unwrap();
}

impl Passport {
    const fn new() -> Passport {
        Passport {
            byr: String::new(),
            eyr: String::new(),
            iyr: String::new(),
            hgt: String::new(),
            hcl: String::new(),
            ecl: String::new(),
            pid: String::new(),
            cid: String::new()
        }
    }

    fn is_valid(&self) -> bool {
        !self.byr.is_empty() && !self.iyr.is_empty() && !self.eyr.is_empty() && !self.hgt.is_empty() &&
            !self.hcl.is_empty() && !self.ecl.is_empty() && !self.pid.is_empty()
    }

    fn is_correct(&self) -> bool {
        RE_BYR.is_match(&self.byr) &&
        RE_IYR.is_match(&self.iyr) &&
        RE_EYR.is_match(&self.eyr) &&
        RE_HGT.is_match(&self.hgt) &&
        RE_HCL.is_match(&self.hcl) &&
        RE_ECL.is_match(&self.ecl) &&
        RE_PID.is_match(&self.pid)
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in".to_string();
        assert_eq!(run(input), vec!["2".to_string(), "2".to_string()]);
    }

    #[test]
    fn example_valid_and_correct() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".to_string();
        assert_eq!(run(input), vec!["4".to_string(), "4".to_string()]);
    }

    #[test]
    fn example_valid_and_incorrect() {
        let input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007".to_string();
        assert_eq!(run(input), vec!["4".to_string(), "0".to_string()]);
    }
}
