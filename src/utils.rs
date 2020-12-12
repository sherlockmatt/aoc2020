use failure::{Error, ResultExt};
use failure::_core::str::FromStr;
use failure::_core::fmt;
use failure::_core::cmp::{Reverse, Ordering};
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::path::Path;
use failure::_core::ops::{Mul, Add, AddAssign};

pub fn download(puzzle_number: usize, session: &str) -> Result<String, Error> {
    let input_path_str = format!("./inputs/day{}.txt", puzzle_number);
    let input_path = Path::new(&input_path_str);

    // If we don't already have this input downloaded, download it now
    if !input_path.exists() {
        let url_to_get = format!("https://adventofcode.com/2020/day/{}/input", puzzle_number);
        let client = reqwest::Client::new();
        if !input_path.parent().unwrap().exists() {
            std::fs::create_dir(input_path.parent().unwrap())
                .with_context(|_| format!("Could not create folder `{}`", input_path.parent().unwrap().display()))?;
        }
        std::fs::write(input_path, client.get(&url_to_get)
            .header("cookie", format!("session={}", session))
            .send()?
            .text()
            .with_context(|_| format!("Could not download from URL {}", url_to_get))?)
            .with_context(|_| format!("Could not write input file `{}`", input_path.display()))?;
    }

    return Ok(std::fs::read_to_string(input_path)
        .with_context(|_| format!("Could not read input file `{}`", input_path.display()))?);
}

pub fn parse_range(input: String) -> Result<Vec<usize>, Error> {
    let mut ret: Vec<usize> = Vec::new();
    for s in input.split(',') {
        let split: Vec<String> = s.split('-').map(String::from).collect();
        if split.len() == 1 {
            ret.push(usize::from_str(&split[0]).with_context(|_| format!("Could not parse int from range value `{}`", split[0]))?);
        } else if split.len() == 2 {
            let start: usize = usize::from_str(&split[0]).with_context(|_| format!("Could not parse int from range value `{}`", split[0]))?;
            let end: usize = usize::from_str(&split[1]).with_context(|_| format!("Could not parse int from range value `{}`", split[1]))?;
            ret.extend(start..=end);
        } else {
            return Err(failure::err_msg(format!("Invalid range spec `{}`", s)));
        }
    }
    Ok(ret)
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, PartialOrd)]
pub struct Pos {
    pub x: usize,
    pub y: usize
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }

    pub fn distance_to(&self, other: &Self) -> usize {
        (if self.x > other.x { self.x - other.x } else { other.x - self.x }) + (if self.y > other.y { self.y - other.y } else { other.y - self.y })
    }
}

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance_to(&Pos::new(0, 0)).cmp(&other.distance_to(&Pos::new(0, 0)))
    }
}

impl Mul<usize> for Pos {
    type Output = Pos;

    fn mul(self, rhs: usize) -> Self::Output {
        Pos { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign<Pos> for Pos {
    fn add_assign(&mut self, rhs: Pos) {
        *self = *self + rhs
    }
}

impl Add<Vector> for Pos {
    type Output = Pos;

    fn add(self, rhs: Vector) -> Self::Output {
        Pos { x: (self.x as i128 + rhs.dx as i128) as usize, y: (self.y as i128 + rhs.dy as i128) as usize }
    }
}

impl AddAssign<Vector> for Pos {
    fn add_assign(&mut self, rhs: Vector) {
        *self = *self + rhs
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, PartialOrd)]
pub struct Vector {
    pub dx: isize,
    pub dy: isize
}

impl Vector {
    #[allow(dead_code)]
    pub fn new(dx: isize, dy: isize) -> Vector {
        Vector { dx, dy }
    }

    pub fn rotate_cw(&mut self) {
        *self = Vector {
            dx: self.dy,
            dy: -1 * self.dx
        };
    }

    pub fn rotate_ccw(&mut self) {
        *self = Vector {
            dx: -1 * self.dy,
            dy: self.dx
        }
    }

    pub fn distance_to(&self, other: &Self) -> usize {
        ((self.dx - other.dx).abs() + (self.dy - other.dy).abs()) as usize
    }
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.dx, self.dy)
    }
}

impl Mul<isize> for Vector {
    type Output = Vector;

    fn mul(self, rhs: isize) -> Self::Output {
        Vector { dx: self.dx * rhs, dy: self.dy * rhs }
    }
}

impl Mul<usize> for Vector {
    type Output = Vector;

    fn mul(self, rhs: usize) -> Self::Output {
        Vector { dx: (self.dx as i128 * rhs as i128) as isize, dy: (self.dy as i128 * rhs as i128) as isize }
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector { dx: self.dx + rhs.dx, dy: self.dy + rhs.dy }
    }
}

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        *self = *self + rhs
    }
}

#[allow(dead_code)]
pub fn astar(room_map: &HashMap<Pos, char>, from: Pos, to: Pos) -> Option<usize> {
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut queue = BinaryHeap::new();
    // The queue stores (estimated total distance, current distance, current pos)
    // Store things in reverse order to get a min-heap
    queue.push(Reverse((from.distance_to(&to), 0, from)));

    while queue.len() > 0 {
        let (_, distance_travelled, current_pos) = queue.pop().unwrap().0;
        visited.insert(current_pos);
        for compare_pos in vec![
            Pos::new(current_pos.x + 1, current_pos.y),
            Pos::new(current_pos.x, current_pos.y + 1),
            Pos::new(current_pos.x - 1, current_pos.y),
            Pos::new(current_pos.x, current_pos.y - 1),
        ] {
            if compare_pos == to {
                return Some(distance_travelled + 1);
            } else if !visited.contains(&compare_pos) && *room_map.get(&compare_pos).unwrap() != '#' {
                queue.push(Reverse((distance_travelled + compare_pos.distance_to(&to), distance_travelled + 1, compare_pos)));
            }
        };
    }

    None
}
