use crate::advent::Solver;

pub struct Day04;
impl Solver for Day04 {
    fn run(&self, input: &str) -> (u64, u64) {
        let parsed = parse(input);
        (part_1(&parsed), part_2(&parsed))
    }

    fn expected(&self) -> (u64, u64) {
        (2599, 1948)
    }
}

fn part_1(parsed: &[Vec<char>]) -> u64 {
    let xs: Vec<Pos> = find_all(parsed, 'X');
    const MAS: &str = "MAS";

    let all_directions = vec![
        Dir::new(0, 1),   // down
        Dir::new(0, -1),  // up
        Dir::new(-1, 0),  // left
        Dir::new(1, 0),   // right
        Dir::new(1, 1),   // down right
        Dir::new(1, -1),  // up right
        Dir::new(-1, 1),  // down left
        Dir::new(-1, -1), // up left
    ];

    let mut xmases = 0;
    for pos in xs {
        for dir in &all_directions {
            let mut pos = pos.clone();
            for expected_next in MAS.chars() {
                if let Some(next_pos) = pos.travel(dir) {
                    if let Some(found) = next_pos.char_at(parsed) {
                        if found == expected_next {
                            if found == 'S' {
                                xmases += 1;
                            } else {
                                pos = next_pos;
                                continue;
                            }
                        }
                    }
                }
                break;
            }
        }
    }

    xmases as u64
}

fn part_2(parsed: &[Vec<char>]) -> u64 {
    fn is_mas(parsed: &[Vec<char>], tuple: (Option<Pos>, Option<Pos>)) -> bool {
        if let (Some(first), Some(second)) = tuple {
            if let (Some(c1), Some(c2)) = (first.char_at(parsed), second.char_at(parsed)) {
                return c1 == 'S' && c2 == 'M' || c1 == 'M' && c2 == 'S';
            }
        }
        false
    }

    let right_down = Dir::new(1, 1);
    let left_up = Dir::new(-1, -1);

    let left_down = Dir::new(-1, 1);
    let right_up = Dir::new(1, -1);

    let xmases = find_all(parsed, 'A')
        .iter()
        .filter(|pos| {
            let right_down_to_left_up = (pos.travel(&right_down), pos.travel(&left_up));
            let left_up_to_right_down = (pos.travel(&left_down), pos.travel(&right_up));
            is_mas(parsed, right_down_to_left_up) && is_mas(parsed, left_up_to_right_down)
        })
        .count();

    xmases as u64
}

fn find_all(parsed: &[Vec<char>], target: char) -> Vec<Pos> {
    parsed
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, c)| if *c == target { Some(Pos { x, y }) } else { None })
        })
        .collect()
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut parsed = Vec::new();
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        parsed.push(row);
    }

    parsed
}

#[derive(Debug)]
struct Dir {
    x_dir: i32,
    y_dir: i32,
}

impl Dir {
    fn new(x_dir: i32, y_dir: i32) -> Self {
        Dir { x_dir, y_dir }
    }
}

#[derive(Debug, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn char_at(&self, parsed: &[Vec<char>]) -> Option<char> {
        parsed.get(self.y).and_then(|row| row.get(self.x)).copied()
    }

    fn travel(&self, dir: &Dir) -> Option<Pos> {
        let x = self.x as i32 + dir.x_dir;
        let y = self.y as i32 + dir.y_dir;
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        Some(Pos { x, y })
    }
}
