use core::fmt;
use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

use crate::advent::Solver;

pub struct Day20;

impl Solver for Day20 {
    type Input = (Maze, Vec<usize>);

    fn parse(&self, input: &str, is_sample: bool) -> Self::Input {
        let mut width = 0;
        let mut height = 0;
        let mut data = Vec::new();
        let mut start = 0;

        let min_savings = if is_sample { 50 } else { 100 };

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

        let mut maze = Maze {
            width,
            height,
            data,
            start,
            min_savings,
        };

        let path = update_path(&mut maze);

        (maze, path)
    }

    fn part_1(&self, input: &mut Self::Input) -> String {
        // println!("Path: {:?}", path);
        find_cheats(&input.0, &input.1, 2).to_string()
    }

    fn part_2(&self, input: &mut Self::Input) -> String {
        find_cheats(&input.0, &input.1, 20).to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("1307", "986545")
    }

    fn name(&self) -> &'static str {
        "Race Condition"
    }
}

fn update_path(maze: &mut Maze) -> Vec<usize> {
    maze.data[maze.start] = Tile::Path(0);

    let mut i = 0;
    let mut next = Some(maze.start);
    let mut path = vec![maze.start];

    while let Some(pos) = next {
        for neighbor in maze.neighbors(pos) {
            match neighbor {
                Some((new_pos, new_tile)) => match new_tile {
                    Tile::End => {
                        i += 1;
                        maze.data[new_pos] = Tile::Path(i);
                        path.push(new_pos);
                        return path;
                    }
                    Tile::Empty => {
                        i += 1;
                        maze.data[new_pos] = Tile::Path(i);
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

    panic!("Couldn't find the full path")
}

fn find_cheats(maze: &Maze, path: &[usize], cheat_max: usize) -> u32 {
    let mut savings = 0;

    let mut mini_queue = VecDeque::new();
    let mut mini_visited: Vec<isize> = vec![-1; maze.width * maze.height];

    for (start_index, pos) in path.iter().enumerate() {
        for (neighbor_loc, neighbor_tile) in maze.neighbors(*pos).into_iter().flatten() {
            mini_queue.push_back((neighbor_loc, neighbor_tile, 1));
        }

        while let Some((pos, tile, steps)) = mini_queue.pop_front() {
            if mini_visited[pos] == start_index as isize {
                continue;
            }

            mini_visited[pos] = start_index as isize;

            if let Tile::Path(end_index) = tile {
                if end_index > start_index + steps {
                    let saved = end_index - start_index - steps;
                    if saved >= maze.min_savings {
                        savings += 1;
                    }
                }
            }

            if steps < cheat_max {
                for (neighbor_loc, neighbor_tile) in maze.neighbors(pos).into_iter().flatten() {
                    mini_queue.push_back((neighbor_loc, neighbor_tile, steps + 1));
                }
            }
        }
    }

    savings
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
    Path(usize),
    Start,
    End,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Wall => write!(f, "#"),
            Tile::Empty => write!(f, "."),
            Tile::Path(_) => write!(f, "."),
            Tile::Start => write!(f, "S"),
            Tile::End => write!(f, "E"),
        }
    }
}
