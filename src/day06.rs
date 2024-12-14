use crate::advent::Solver;
use std::collections::HashSet;

pub struct Day06;
impl Solver for Day06 {
    type Input = (Grid, Guard);

    fn parse(&self, input: &str) -> Self::Input {
        let mut map = Vec::new();
        let mut guard = None;
        let mut height = 0;
        let mut width = 0;
        for (y, line) in input.lines().enumerate() {
            height += 1;
            if width == 0 {
                width = line.len();
            }

            for (x, c) in line.chars().enumerate() {
                let square = match c {
                    '#' => Square::Obstacle,
                    '.' => Square::Empty,
                    _ => {
                        guard = Some(Guard {
                            dir: Dir::from_str(c),
                            pos: Pos { x, y },
                        });
                        Square::Empty
                    }
                };
                map.push((square, Marker { round: 0, dir: Dir::Up }));
            }
        }

        if let Some(g) = guard {
            (Grid { width, height, map }, g)
        } else {
            panic!("No guard found");
        }
    }

    fn part_1(&self, (grid, guard): &mut Self::Input) -> u64 {
        let candidate_pos = part_1(grid, guard.clone());
        (candidate_pos.len() + 1) as u64
    }

    fn part_2(&self, (grid, guard): &mut Self::Input) -> u64 {
        let candidate_pos = part_1(grid, guard.clone());

        let mut obstacles_that_worked = 0;

        let mut i = 1;

        let mut last_pos = guard.pos.clone();
        let mut last_dir = guard.dir;

        for (pos, dir) in candidate_pos {
            if let Some((square, _)) = grid.map.get_mut(pos.y * grid.width + pos.x) {
                *square = Square::Obstacle;
            }
            if let Some((square, _)) = grid.map.get_mut(last_pos.y * grid.width + last_pos.x) {
                *square = Square::Empty;
            }

            guard.pos = last_pos;
            guard.dir = last_dir;

            while let Some(result) = guard.step(grid, i + 1) {
                if result == Advancement::Loop {
                    obstacles_that_worked += 1;
                    break;
                }
            }

            i += 1;

            last_pos = pos;
            last_dir = dir;
        }

        obstacles_that_worked
    }

    fn expected(&self) -> (u64, u64) {
        (4696, 1443)
    }

    fn name(&self) -> &'static str {
        "Guard Gallivant"
    }
}

fn part_1(grid: &mut Grid, mut guard: Guard) -> Vec<(Pos, Dir)> {
    let mut visited = HashSet::new();
    let mut in_order = Vec::new();

    visited.insert(guard.pos.clone());

    while guard.step(grid, 0).is_some() {
        if visited.insert(guard.pos.clone()) {
            in_order.push((guard.pos.clone(), guard.dir));
        }
    }

    in_order
}

#[derive(Debug)]
struct Marker {
    round: usize,
    dir: Dir,
}

#[derive(Debug)]
pub struct Grid {
    width: usize,
    height: usize,
    map: Vec<(Square, Marker)>,
}

impl Grid {
    fn get_mut(&mut self, pos: &Pos) -> Option<&mut (Square, Marker)> {
        self.map.get_mut(pos.y * self.width + pos.x)
    }

    fn travel(&self, pos: &Pos, dir: &Dir) -> Option<Pos> {
        let delta = dir.delta();
        let x = pos.x as i32 + delta.0;
        let y = pos.y as i32 + delta.1;

        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }

        Some(Pos {
            x: x as usize,
            y: y as usize,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
enum Square {
    Obstacle,
    Empty,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn delta(&self) -> (i32, i32) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }

    fn from_str(c: char) -> Self {
        match c {
            '^' => Dir::Up,
            'v' => Dir::Down,
            '<' => Dir::Left,
            '>' => Dir::Right,
            _ => panic!("Invalid character"),
        }
    }

    fn rotate_right(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Advancement {
    Normal,
    Loop,
}

#[derive(Debug, Clone)]
pub struct Guard {
    dir: Dir,
    pos: Pos,
}

impl Guard {
    fn step(&mut self, grid: &mut Grid, round: usize) -> Option<Advancement> {
        let new_pos = grid.travel(&self.pos, &self.dir);
        if let Some(new_pos) = new_pos {
            let found = grid.get_mut(&new_pos);

            match found {
                None => None,
                Some((Square::Empty, marker)) => {
                    self.pos = new_pos;

                    let result = if marker.round == round && marker.dir == self.dir {
                        Advancement::Loop
                    } else {
                        marker.round = round;
                        Advancement::Normal
                    };

                    marker.round = round;
                    marker.dir = self.dir;

                    Some(result)
                }
                Some((Square::Obstacle, _)) => {
                    self.dir = self.dir.rotate_right();
                    self.step(grid, round)
                }
            }
        } else {
            None
        }
    }
}
