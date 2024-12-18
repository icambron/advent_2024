use regex::Regex;
use crate::advent::Solver;

// sample sizes
// const WIDTH: i32 = 11;
// const HEIGHT: i32 = 7;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

pub struct Day14;

impl Solver for Day14 {
    type Input = Vec<Robot>;

    fn parse(&self, input: &str) -> Self::Input {
        let regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        input.lines().map(|l| {
            let caps = regex.captures(l).unwrap();
            Robot {
                pos: (caps[1].parse().unwrap(), caps[2].parse().unwrap()),
                velocity: (caps[3].parse().unwrap(), caps[4].parse().unwrap()),
            }
        }).collect()
    }

    fn part_1(&self, input: &mut Self::Input) -> String {
        
        let mut quadrants: [u16; 4] = [0, 0, 0, 0];
        
        for robot in input {
            
            let (x, y) = tick_robot(robot, 100);

            if x == WIDTH / 2 || y == HEIGHT / 2 {
                continue;
            }
            
            let is_left = x < WIDTH / 2;
            let is_top = y < HEIGHT / 2;
            let quadrant = (is_top as usize) * 2 + is_left as usize;
            
            quadrants[quadrant] += 1;
        }

        quadrants.iter().fold(1, |acc, &q| acc * q).to_string()
    }
    
    fn part_2(&self, input: &mut Self::Input) -> String {
        let mut visited = [0; WIDTH as usize * HEIGHT as usize];
        
        for i in 1..= WIDTH * HEIGHT {
            let mut found_dupe = false;
            for robot in input.iter_mut() {
                let (x, y) = robot.tick(1);
                
                if !found_dupe {
                    let j = visited.get_mut((y * WIDTH + x) as usize).unwrap();
                    if *j == i {
                        found_dupe = true;
                    }
                    *j = i;
                }
            }

            if !found_dupe && hard_check(&visited, i) {
                return i.to_string()
            }
        }
        "0".to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("214109808", "7687")
    }

    fn name(&self) -> &'static str {
        "Restroom Rebound"
    }
}

fn hard_check(counts: &[i32], expected: i32) -> bool {
    counts.windows(12).any(|window| window.iter().all(|&b| b == expected))
}

fn tick_robot(robot: &Robot, seconds: i32) -> (i32, i32) {
    let mut x = (robot.pos.0 + robot.velocity.0 * seconds) % WIDTH;
    let mut y = (robot.pos.1 + robot.velocity.1 * seconds) % HEIGHT;

    if x < 0 {
        x += WIDTH;
    }

    if y < 0 {
        y += HEIGHT;
    }

    (x, y)
}

#[allow(dead_code)]
fn print_frame(i: u64, robots: &[Robot]) {
    println!("SECONDS: {}", i);
    println!("{}", stringify_robots(robots));
    println!();
    println!();
}

fn stringify_robots(robots: &[Robot]) -> String {
    let mut grid: Vec<Vec<u8>> = vec![vec![0; WIDTH as usize]; HEIGHT as usize];
    for robot in robots {
        grid[robot.pos.1 as usize][robot.pos.0 as usize] += 1;
    }
    
    let mut s = String::new();
    for row in grid {
        for cell in row {
            if cell > 0 {
                s.push((cell + b'0') as char)
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    
    s
}

#[derive(Debug)]
pub struct Robot {
    pos: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn tick(&mut self, seconds: i32) -> (i32, i32){
        let (x, y) = tick_robot(self, seconds);
        self.pos = (x, y);
        (x, y)
    }
}