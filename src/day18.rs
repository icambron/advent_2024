use std::collections::{HashSet, VecDeque};
use itertools::Itertools;
use crate::advent::Solver;

// sample input
// const WIDTH_AND_HEIGHT: usize = 7;
// const BYTES: usize = 12;

const WIDTH_AND_HEIGHT: usize = 71;
const BYTES: usize = 1024;

pub struct Day18;

impl Solver for Day18 {
    type Input = Vec<usize>;

    fn parse(&self, input: &str) -> Self::Input {
       input.lines().map(|line| {
            let mut split = line.split(",");
            let x: usize = split.next().unwrap().parse().unwrap();
            let y: usize = split.next().unwrap().parse().unwrap();
            x + y * WIDTH_AND_HEIGHT
        }).collect()
    }

    fn part_1(&self, input: &mut Self::Input) -> String {
        // oh hey, another bfs
        let mut space = vec![false; WIDTH_AND_HEIGHT * WIDTH_AND_HEIGHT];
        for i in input.iter().take(BYTES) {
            space[*i] = true;
        }
        
        let mut queue : VecDeque<(usize, usize)> = VecDeque::from(vec![(0, 0)]);
        
        while let Some((coord, steps)) = queue.pop_back() {
            
            let s = space.get_mut(coord).unwrap();
            if *s {
                continue;
            }
            *s = true;
            
            if coord == WIDTH_AND_HEIGHT * WIDTH_AND_HEIGHT - 1 {
                return steps.to_string()
            }
            
            for new_coord in next_coord(coord).into_iter().flatten() {
                queue.push_front((new_coord, steps + 1));
            }
        }
        
        "not found".to_string()
    }

    fn part_2(&self, input: &mut Self::Input) -> String {
        let mut space = vec![false; WIDTH_AND_HEIGHT * WIDTH_AND_HEIGHT];

        let mut path = find_path_depth_first(&mut space.clone(), 0).unwrap();
        
        for i in input.iter() {
            space[*i] = true;
            
            if !path.contains(i) {
                continue;
            } else if let Some(new_path) = find_path_depth_first(&mut space.clone(), 0) {
                path = new_path;
            } else {
                return format!("{},{}", i % WIDTH_AND_HEIGHT, i / WIDTH_AND_HEIGHT);
            }
        }
        
        "never found".to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("272", "16,44")
    }

    fn name(&self) -> &'static str {
        "RAM Run"
    }
}

#[allow(dead_code)]
fn print_space(space: &[bool], path: &HashSet<usize>) {
    for i in 0..WIDTH_AND_HEIGHT {
        for j in 0..WIDTH_AND_HEIGHT {
            if path.contains(&(i * WIDTH_AND_HEIGHT + j)) {
                print!("O");
            } else {
                print!("{}", if space[i * WIDTH_AND_HEIGHT + j] { "#" } else { " " });
            }
        }
        println!();
    }
}

// returns path we found
fn find_path_depth_first(squares: &mut Vec<bool>, coord: usize) -> Option<HashSet<usize>> {
    
    if coord == WIDTH_AND_HEIGHT * WIDTH_AND_HEIGHT - 1 {
        return Some(HashSet::new());
    }
    
    for new_coord in next_coord(coord).into_iter().flatten().sorted_by(|a, b| b.cmp(a)) {
        if !squares[new_coord] {
            squares[new_coord] = true;
            if let Some(mut path) = find_path_depth_first(squares, new_coord) {
                path.insert(new_coord);
                return Some(path);
            }
        }
    }
    None
}

fn next_coord(old_coord: usize) -> [Option<usize>; 4] {
    [
        if old_coord % WIDTH_AND_HEIGHT > 0 { Some(old_coord - 1) } else { None },
        if old_coord % WIDTH_AND_HEIGHT < WIDTH_AND_HEIGHT - 1 { Some(old_coord + 1) } else { None },
        if old_coord >= WIDTH_AND_HEIGHT { Some(old_coord - WIDTH_AND_HEIGHT) } else { None },
        if old_coord < WIDTH_AND_HEIGHT * (WIDTH_AND_HEIGHT - 1) { Some(old_coord + WIDTH_AND_HEIGHT) } else { None }
    ]
}