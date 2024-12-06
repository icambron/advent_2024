use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use crate::advent::Advent;

pub fn run(advent: Advent) {
    let (grid, guard) = parse_input(&advent.path());
    let candidate_pos = part_1(&grid, guard.clone());
    part_2(&grid, guard, candidate_pos);
}

fn part_1(grid: &Grid, mut guard: Guard) -> HashSet<Pos> {
    let mut visited = HashSet::new();

    while guard.step(grid, &None) {
        visited.insert(guard.pos.clone());
    }

    println!("Part 1: {}", visited.len());

    visited
}

fn part_2(grid: &Grid, guard: Guard, candidate_pos: HashSet<Pos>) {

    let mut obstacles_that_worked = 0;

    let mut i = 1;

    // a hashset was way too slow, switched to a 1-d vec of dir; only need to store last dir because can only loop in rectangles
    // ideally, we'd integrate this into the grid to avoid the second lookup, but that looked complicated and this is plenty fast
    let mut visited = vec![(0, Dir::Up); grid.width * grid.height];

    for pos in candidate_pos {

        // can't put a new obstacle in the same place as the guard
        if pos == guard.pos {
            continue;
        }

        let mut new_guard = guard.clone();
        let pos = Some(pos);

        while new_guard.step(grid, &pos) {
            let (j, dir) = visited.get_mut(new_guard.pos.y * grid.width + new_guard.pos.x).unwrap();
            if *j == i && *dir == new_guard.dir {
                obstacles_that_worked += 1;
                break;
            }

            *j = i;
            *dir = new_guard.dir;
        }

        i += 1;
    }

    println!("Part 2: {}", obstacles_that_worked);
}

fn parse_input(file: &str) -> (Grid, Guard) {
    let file = File::open(file).expect("Should be able to open file");
    let lines = io::BufReader::new(file).lines();
    let mut map = Vec::new();
    let mut guard = None;
    let mut height = 0;
    let mut width = 0;
    for (y, line) in lines.enumerate().map_while(|(y, l)| l.ok().map(|line| (y, line))) {
        height += 1;
        if width == 0 {
            width = line.len();
        }

        for (x, c) in line.chars().enumerate() {
            let square = match c {
                '#' => { Square::Obstacle },
                '.' => { Square::Empty },
                _  => {
                    guard = Some(Guard {
                        dir: Dir::from_str(c),
                        pos: Pos { x, y },
                    });
                    Square::Empty
                },
            };
            map.push(square);
        }
    }

    if let Some(g) = guard {
        (Grid {
            width,
            height,
            map
        }, g)
    } else {
        panic!("No guard found");
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    map: Vec<Square>,
}

impl Grid {
    fn get(&self, pos: &Pos) -> Option<&Square> {
        self.map.get(pos.y * self.width + pos.x)
    }

    fn travel(&self, pos: &Pos, dir: &Dir) -> Option<Pos> {
        let delta = dir.delta();
        let x = pos.x as i32 + delta.0;
        let y = pos.y as i32 + delta.1;

        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }

        Some(Pos { x: x as usize, y: y as usize })
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
    Empty
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right
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
            _ => panic!("Invalid character")
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

#[derive(Debug, Clone)]
struct Guard {
    dir: Dir,
    pos: Pos,
}

impl Guard {
    pub fn step(&mut self, grid: &Grid, obstacle: &Option<Pos>) -> bool {
        let new_pos = grid.travel(&self.pos, &self.dir);
        if let Some(new_pos) = new_pos {
            if let Some(obstacle) = obstacle {
                if new_pos == *obstacle {
                    self.dir = self.dir.rotate_right();
                    return self.step(grid, &Some(obstacle.clone()))
                }
            }

            let found = grid.get(&new_pos);

            match found {
                None => false,
                Some(Square::Empty) => {
                    self.pos = new_pos;
                    true
                },
                Some(Square::Obstacle) => {
                    self.dir = self.dir.rotate_right();
                    self.step(grid, obstacle)
                }
            }
        } else {
            false
        }
    }
}