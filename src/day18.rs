use std::collections::{HashSet, VecDeque};
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
        let mut space = vec![false; WIDTH_AND_HEIGHT * WIDTH_AND_HEIGHT];
        for i in input.iter().take(BYTES) {
            space[*i] = true;
        }
        
        find_path_breadth_first(&mut space).unwrap().to_string()
        
    }

    fn part_2(&self, input: &mut Self::Input) -> String {
        let mut low = 0;
        let mut high = input.len() - 1;
        
        while low < high {
            let mid = (low + high) / 2;
            let mut space = vec![false; WIDTH_AND_HEIGHT * WIDTH_AND_HEIGHT];
            for i in input.iter().take(mid) {
                space[*i] = true;
            }
            
            
            if find_path_breadth_first(&mut space).is_some() {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        
        let v = input[high - 1];
        format!("{},{}", v % WIDTH_AND_HEIGHT, v / WIDTH_AND_HEIGHT)
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


fn find_path_breadth_first(space: &mut [bool]) -> Option<usize> {

    let mut queue : VecDeque<(usize, usize)> = VecDeque::from(vec![(0, 0)]);
    while let Some((coord, steps)) = queue.pop_back() {
        let s = space.get_mut(coord).unwrap();
        if *s {
            continue;
        }
        *s = true;

        if coord == WIDTH_AND_HEIGHT * WIDTH_AND_HEIGHT - 1 {
            return Some(steps)
        }

        for new_coord in next_coord(coord).into_iter().flatten() {
            queue.push_front((new_coord, steps + 1));
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