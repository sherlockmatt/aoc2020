use std::collections::HashMap;
use crate::utils::Pos;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();
    let parts: Vec<(for<'r, 's> fn(&'r Pos, usize, usize, &'s HashMap<Pos, bool>) -> Vec<Pos>, usize)> = vec![(neighbouring_seats, 4usize), (visible_seats, 5usize)];

    for (seat_fn, min_chairs) in parts {
        let mut grid: HashMap<Pos, bool> = HashMap::new();//input.lines().enumerate().flat_map(|(y, l)| l.chars().enumerate().map(|(x, c)| (Pos { x:x.clone(), y:y.clone() }, c == '#'))).collect();
        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if c != '.' {
                    grid.insert(Pos { x, y }, c == '#');
                }
            }
        }
        let Pos { x: max_x, y: max_y } = grid.keys().map(|&p| p).max().unwrap();

        loop {
            let mut new_grid: HashMap<Pos, bool> = HashMap::new();
            for (p, s) in grid.clone() {
                new_grid.insert(p, match (s, seat_fn(&p, max_x, max_y, &grid).iter().filter(|p| grid.contains_key(p) && *grid.get(p).unwrap()).count()) {
                    (false, 0) => true,
                    (true, x) if (min_chairs..=8).contains(&x) => false,
                    _ => s
                });
            }
            if new_grid == grid {
                answers.push(format!("{}", grid.values().filter(|b| **b).count()));
                break;
            }
            grid = new_grid;
        }
    }

    return answers;
}

fn neighbouring_seats(p: &Pos, max_x: usize, max_y: usize, _: &HashMap<Pos, bool>) -> Vec<Pos> {
    let mut seats = Vec::new();
    for (dx, dy) in vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
        if (dx >= 0 || p.x > 0) && (dx <= 0 || p.x < max_x) &&
            (dy >= 0 || p.y > 0) && (dy <= 0 || p.y < max_y) {
            seats.push(Pos { x: (p.x as i128 + dx) as usize, y: (p.y as i128 + dy) as usize });
        }
    }
    seats
}

fn visible_seats(p: &Pos, max_x: usize, max_y: usize, grid: &HashMap<Pos, bool>) -> Vec<Pos> {
    let mut seats = Vec::new();
    for (dx, dy) in vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
        let mut cur_pos = p.clone();
        while (dx >= 0 || cur_pos.x > 0) && (dx <= 0 || cur_pos.x < max_x) &&
            (dy >= 0 || cur_pos.y > 0) && (dy <= 0 || cur_pos.y < max_y) {
            cur_pos = Pos { x: (cur_pos.x as i128 + dx) as usize, y: (cur_pos.y as i128 + dy) as usize };
            if grid.contains_key(&cur_pos) {
                seats.push(cur_pos);
                break;
            }
        }
    }
    seats
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example1() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL".to_string();
        assert_eq!(run(input), vec!["37".to_string(), "26".to_string()]);
    }
}
