use crate::advent::Solver;
use regex::Regex;

pub struct Day14;

impl Solver for Day14 {
    type Input = Grid;

    fn parse(&self, input: &str, is_sample: bool) -> Self::Input {
        let regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        let robots = input
            .lines()
            .map(|l| {
                let caps = regex.captures(l).unwrap();
                Robot {
                    pos: (caps[1].parse().unwrap(), caps[2].parse().unwrap()),
                    velocity: (caps[3].parse().unwrap(), caps[4].parse().unwrap()),
                }
            })
            .collect();

        Grid {
            width: if is_sample { 11 } else { 101 },
            height: if is_sample { 7 } else { 103 },
            robots,
        }
    }

    fn part_1(&self, grid: &mut Self::Input) -> String {
        let mut quadrants: [u64; 4] = [0, 0, 0, 0];

        for robot in &grid.robots {
            let (x, y) = tick_robot(robot, 100, grid.width, grid.height);

            if x == grid.width / 2 || y == grid.height / 2 {
                continue;
            }

            let is_left = x < grid.width / 2;
            let is_top = y < grid.height / 2;
            let quadrant = (is_top as usize) * 2 + is_left as usize;

            quadrants[quadrant] += 1;
        }

        quadrants.iter().product::<u64>().to_string()
    }

    fn part_2(&self, grid: &mut Self::Input) -> String {
        let mut visited = vec![0; grid.width as usize * grid.height as usize];

        for i in 1..=grid.width * grid.height {
            let mut found_dupe = false;
            for robot in grid.robots.iter_mut() {
                let (x, y) = robot.tick(1, grid.width, grid.height);

                if !found_dupe {
                    let j = visited.get_mut((y * grid.width + x) as usize).unwrap();
                    if *j == i {
                        found_dupe = true;
                    }
                    *j = i;
                }
            }

            if !found_dupe && hard_check(&visited, i) {
                return i.to_string();
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

fn tick_robot(robot: &Robot, seconds: i32, width: i32, height: i32) -> (i32, i32) {
    let mut x = (robot.pos.0 + robot.velocity.0 * seconds) % width;
    let mut y = (robot.pos.1 + robot.velocity.1 * seconds) % height;

    if x < 0 {
        x += width;
    }

    if y < 0 {
        y += height;
    }

    (x, y)
}

#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn tick(&mut self, seconds: i32, width: i32, height: i32) -> (i32, i32) {
        let (x, y) = tick_robot(self, seconds, width, height);
        self.pos = (x, y);
        (x, y)
    }
}

pub struct Grid {
    width: i32,
    height: i32,
    robots: Vec<Robot>,
}
