use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use crate::advent::Advent;

pub fn run(advent: Advent) {
    let (map, guard) = parse_input(&advent.path());
    let candidate_pos = part_1(&map, guard.clone());
    part_2(&map, guard, candidate_pos);
}

fn part_1(map: &Vec<Vec<Square>>, mut guard: Guard) -> HashSet<Pos> {
    let mut visited = HashSet::new();
    
    while guard.step(map, &None) {
        visited.insert(guard.pos.clone());
    }

    println!("Part 1: {}", visited.len());
    
    visited
}

fn part_2(map: &Vec<Vec<Square>>, guard: Guard, candidate_pos: HashSet<Pos>) {
    
    let mut obstacles_that_worked = 0;
    
    for pos in candidate_pos {
        
        // a hashset was way too slow, switched to a 2d vec of dir
        let mut visited = vec![vec![None; map[0].len()]; map.len()];
        
        // can't put a new obstacle in the same place as the guard
        if pos == guard.pos {
            continue;
        }

        let mut new_guard = guard.clone();
        let pos = Some(pos);
        
        while new_guard.step(map, &pos) {
            if let Some(dir) = visited[new_guard.pos.y][new_guard.pos.x] {
                if dir == new_guard.dir {
                    obstacles_that_worked += 1;
                    break;
                }
            }
            
            visited[new_guard.pos.y][new_guard.pos.x] = Some(new_guard.dir);
        }
    }

    println!("Part 2: {}", obstacles_that_worked);
}

fn parse_input(file: &str) -> (Vec<Vec<Square>>, Guard) {
    let file = File::open(file).expect("Should be able to open file");
    let lines = io::BufReader::new(file).lines();
    let mut map = Vec::new();
    let mut guard = None;
    for (y, line) in lines.enumerate().map_while(|(y, l)| l.ok().map(|line| (y, line))) {
        let mut row = Vec::with_capacity(line.len());
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => { row.push(Square::Obstacle); },
                '.' => { row.push(Square::Empty); },
                _  => {
                    row.push(Square::Empty);
                    guard = Some(Guard {
                        dir: Dir::from_str(c),
                        pos: Pos { x, y },
                    });
                },
            }
        }
        
        map.push(row);
    }
    
    if let Some(g) = guard {
        (map, g)
    } else {
        panic!("No guard found");
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn travel(&self, dir: &Dir) -> Option<Pos> {
        let delta = dir.delta();
        let x = self.x as i32 + delta.0;
        let y = self.y as i32 + delta.1;
        
        if x < 0 || y < 0 {
            return None;
        }
        
        Some(Pos { x: x as usize, y: y as usize })
    }
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
    pub fn step(&mut self, map: &Vec<Vec<Square>>, obstacle: &Option<Pos>) -> bool {
        let new_pos = self.pos.travel(&self.dir);
        if let Some(new_pos) = new_pos {
            
            let square = if let Some(obstacle) = obstacle {
                if obstacle == &new_pos {
                    Some(&Square::Obstacle)
                } else {
                    map.get(new_pos.y).and_then(|row| row.get(new_pos.x))
                }
            } else {
                map.get(new_pos.y).and_then(|row| row.get(new_pos.x))
            };

            match square {
                None => false,
                Some(Square::Empty) => {
                    self.pos = new_pos;
                    true
                },
                Some(Square::Obstacle) => {
                    self.dir = self.dir.rotate_right();
                    self.step(map, obstacle)
                }
            }
            
        } else {
            false
        }
    }
}