use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use crate::advent::Solver;

pub struct Day15;

impl Solver for Day15 {
    type Input = (Map, Vec<char>);

    fn parse(&self, input: &str) -> Self::Input {
        
        let split: Vec<&str> = input.split("\n\n").collect();
        let [map_txt, dir_txt]= split[0..2] else { panic!("Can't parse") };
        
        let mut robot_location: Option<usize> = None;
        let width = map_txt.find('\n').unwrap();
        let map: Vec<Square> = map_txt.chars().filter(|c| *c != '\n').enumerate().map(|(i, c)| {
            match c {
                '@' => {
                    robot_location = Some(i);
                    Square::Empty
                },
                '.' => Square::Empty,
                'O' => Square::Box,
                '#' => Square::Wall,
                _ => panic!("Unknown character")
            }
        }).collect();
        
        let dirs: Vec<char> = dir_txt.chars().filter(|c| *c != '\n').collect();
        let robot_location = robot_location.unwrap();
        
        (Map { squares: map, width, robot_location }, dirs)
    }

    fn part_1(&self, (map, dirs): &mut Self::Input) -> u64 {
        let mut map = map.clone();
        solve(&mut map, dirs)
    }
    
    fn part_2(&self, (smol_map, dirs): &mut Self::Input) -> u64 {
        let mut map = embiggen_squares(smol_map);
        solve(&mut map, dirs)
    }

    fn expected(&self) -> (u64, u64) {
        (1442192, 1448458)
    }

    fn name(&self) -> &'static str {
        "Warehouse Woes"
    }
}

fn solve(map: &mut Map, dirs: &[char]) -> u64{
    let mut train: Vec<(usize, Square)> = Vec::new();

    'dir: for dir in dirs.iter() {
        train.clear();

        let is_vert = matches!(dir, '^' | 'v');
        
        let mut lanes: BTreeSet<usize> = BTreeSet::from([map.robot_location]);
        let mut next_robot_location = 0;

        loop {
            let mut end_of_the_line = true;
            let mut new_lanes: BTreeSet<usize> = BTreeSet::new();
            for lane in lanes.into_iter() {
                
                let next_idx = step(map, lane, *dir);
                let next = &map.squares[next_idx];
                
                if next_robot_location == 0 {
                    next_robot_location = next_idx;
                }

                match next {
                    Square::Empty => {
                    }
                    
                    Square::BoxLeft => {
                        end_of_the_line = false;
                        train.push((next_idx, Square::BoxLeft));
                        new_lanes.insert(next_idx);
                        
                        if is_vert {
                            new_lanes.insert(next_idx + 1);
                            train.push((next_idx + 1, Square::BoxRight));
                        }
                    }
                    
                    Square::BoxRight => {
                        end_of_the_line = false;
                        train.push((next_idx, Square::BoxRight));
                        new_lanes.insert(next_idx);
                        
                        if is_vert {
                            new_lanes.insert(next_idx - 1);
                            train.push((next_idx - 1, Square::BoxLeft));
                        }
                    }

                    Square::Box => {
                        end_of_the_line = false;
                        new_lanes.insert(next_idx);
                        train.push((next_idx, Square::Box));
                    }
                    Square::Wall => {
                        continue 'dir;
                    }
                }

            }

            if end_of_the_line {
                map.robot_location = next_robot_location;
                for (idx, square) in train.iter().rev() {

                    let old = map.squares.get_mut(*idx).unwrap();
                    *old = Square::Empty;

                    let new_idx = step(map, *idx, *dir);
                    let new = map.squares.get_mut(new_idx).unwrap();
                    *new = *square;

                }
                break;
            }

            lanes = new_lanes;
        }
    }
    map.sum_coords() as u64
}

fn step(map: &Map, idx: usize, dir: char) -> usize {
    let delta = match dir {
        '<' => -1,
        '>' => 1,
        '^' => -(map.width as isize),
        'v' => map.width as isize,
        _ => panic!("Unknown direction")
    };
    (idx as isize + delta) as usize
}

fn embiggen_squares(map: &Map) -> Map {
    let mut squares = Vec::with_capacity(map.squares.len() * 2);
    for square in map.squares.iter() {
        match square {
            Square::Box => {
                squares.push(Square::BoxLeft);
                squares.push(Square::BoxRight);
            }
            
            _ =>{
                squares.push(*square);
                squares.push(*square);
            } 
        }
    }
    
    Map {
        robot_location: map.robot_location * 2,
        squares,
        width: map.width * 2,
    }
}

#[derive(Debug, Clone, Copy)]
enum Square {
    BoxLeft,
    BoxRight,
    Box,
    Empty,
    Wall
}

#[derive(Debug, Clone)]
pub struct Map {
    robot_location: usize,
    squares: Vec<Square>,
    width: usize,
}

impl Map {
    fn sum_coords(&self) -> usize {
        self.squares
            .iter()
            .enumerate()
            .filter(|(_, s)| matches!(s, Square::Box) || matches!(s, Square::BoxLeft))
            .map(|(i, _)| {
                let x = i % self.width;
                let y = i / self.width;
                x + y * 100
            }).sum()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, square) in self.squares.iter().enumerate() {
            
            if i == self.robot_location {
                write!(f, "@")?;
            } else {
                match square {
                    Square::Box => write!(f, "O")?,
                    Square::Empty => write!(f, ".")?,
                    Square::Wall => write!(f, "#")?,
                    Square::BoxLeft => write!(f, "[")?,
                    Square::BoxRight => write!(f, "]")?,
                }
            }
            if i % self.width == self.width - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

