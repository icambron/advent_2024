use core::fmt;
use std::fmt::{Display, Formatter};

use crate::advent::Solver;

pub struct Day20;

impl Solver for Day20 {
    type Input = Maze;

    fn parse(&self, input: &str, is_sample: bool) -> Self::Input {
        let mut width = 0;
        let mut height = 0;
        let mut data = Vec::new();
        let mut start = 0;

        let min_savings = if is_sample { 1 } else { 100 };

        for (y, line) in input.lines().enumerate() {
            width = line.len();
            for (x, c) in line.chars().enumerate() {
                let tile = match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    'S' => {
                        start = y * width + x;
                        Tile::Start
                    }
                    'E' => Tile::End,
                    _ => panic!("Invalid tile: {}", c),
                };
                data.push(tile);
            }
            height += 1;
        }

        Maze {
            width,
            height,
            data,
            start,
            min_savings,
        }
    }

    fn part_1(&self, input: &mut Self::Input) -> String {
        let path = update_path(input);
        // println!("Path: {:?}", path);
        find_cheats(input, path).to_string()
    }

    fn part_2(&self, _input: &mut Self::Input) -> String {
        todo!()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        todo!()
    }

    fn name(&self) -> &'static str {
        "Race Condition"
    }
}

fn update_path(maze: &mut Maze) -> Vec<usize> {
    maze.data[maze.start] = Tile::Used(0);

    let mut i = 0;
    let mut next = Some(maze.start);
    let mut path = vec![maze.start];

    while let Some(pos) = next {
        //println!("examining pos {:?}", pos);
        for neighbor in maze.neighbors(pos) {
            // println!("examining neighbor {:?}", neighbor);
            match neighbor {
                Some((new_pos, new_tile)) => match new_tile {
                    Tile::End => {
                        i += 1;
                        maze.data[new_pos] = Tile::Used(i);
                        path.push(new_pos);
                        return path;
                    }
                    Tile::Empty => {
                        i += 1;
                        maze.data[new_pos] = Tile::Used(i);
                        path.push(new_pos);
                        next = Some(new_pos);
                        // only one path forward
                        break;
                    }
                    _ => {
                        next = None;
                    }
                },
                None => next = None,
            }
        }
    }

    print_maze(maze);
    panic!("Couldn't find the full path")
}

fn find_cheats(maze: &Maze, path: Vec<usize>) -> u32 {
    let mut savings = 0;

    for (pre_index, pos) in path.iter().enumerate() {
        for (cheat_start, tile) in maze.neighbors(*pos).into_iter().flatten() {
            if tile == Tile::Wall {
                for (_, cheat_end_tile) in maze.neighbors(cheat_start).into_iter().flatten() {
                    if let Tile::Used(end_index) = cheat_end_tile {
                        // println!("Cheating from {} to {}", cheat_start, end_index);
                        if end_index > pre_index + 2 {
                            let saved = end_index - pre_index - 2;
                            if saved >= maze.min_savings {
                                savings += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    savings
}

fn print_maze(maze: &Maze) {
    for y in 0..maze.height {
        for x in 0..maze.width {
            let pos = y * maze.width + x;
            match maze.data[pos] {
                Tile::Empty => print!("."),
                Tile::Wall => print!("#"),
                Tile::Used(_) => print!("X"),
                Tile::Start => print!("S"),
                Tile::End => print!("E"),
            };
        }
        println!();
    }
}

#[derive(Debug)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Tile>,
    pub start: usize,
    pub min_savings: usize,
}

impl Maze {
    fn neighbors(&self, pos: usize) -> [Option<(usize, Tile)>; 4] {
        [
            self.tile_at(self.up(pos)),
            self.tile_at(self.down(pos)),
            self.tile_at(self.left(pos)),
            self.tile_at(self.right(pos)),
        ]
    }

    fn up(&self, pos: usize) -> Option<usize> {
        if pos < self.width {
            None
        } else {
            Some(pos - self.width)
        }
    }

    fn down(&self, pos: usize) -> Option<usize> {
        if pos >= self.width * (self.height - 1) {
            None
        } else {
            Some(pos + self.width)
        }
    }

    fn left(&self, pos: usize) -> Option<usize> {
        if pos.is_multiple_of(self.width) {
            None
        } else {
            Some(pos - 1)
        }
    }

    fn right(&self, pos: usize) -> Option<usize> {
        if pos % self.width == self.width - 1 {
            None
        } else {
            Some(pos + 1)
        }
    }

    fn tile_at(&self, pos: Option<usize>) -> Option<(usize, Tile)> {
        pos.and_then(|p| self.data.get(p).copied().map(|tile| (p, tile)))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Tile {
    Wall,
    Empty,
    Used(usize),
    Start,
    End,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Wall => write!(f, "#"),
            Tile::Empty => write!(f, "."),
            Tile::Used(_) => write!(f, "."),
            Tile::Start => write!(f, "S"),
            Tile::End => write!(f, "E"),
        }
    }
}
