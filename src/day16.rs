use crate::advent::Solver;
use hashbrown::HashSet;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::hash::Hash;

pub struct Day16;

impl Solver for Day16 {
    type Input = Maze;

    fn parse(&self, input: &str, _: bool) -> Self::Input {
        let mut start = None;

        let width = input.find('\n').unwrap();
        let map = input
            .chars()
            .filter(|c| *c != '\n')
            .enumerate()
            .map(|(i, c)| {
                if c == 'S' {
                    start = Some(i);
                }
                c
            })
            .collect();

        Maze {
            start: start.unwrap(),
            map,
            width,
        }
    }

    fn part_1(&self, maze: &mut Self::Input) -> String {
        solve(maze, false).0.to_string()
    }

    fn part_2(&self, maze: &mut Self::Input) -> String {
        solve(maze, true).1.to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("85432", "465")
    }

    fn name(&self) -> &'static str {
        "Reindeer Maze"
    }
}

pub fn solve(maze: &Maze, multiple: bool) -> (u64, usize) {
    let initial_step = Step {
        coord: maze.start,
        dir: Dir::East,
        prev: None,
        cost: 0,
    };

    let mut queue: BinaryHeap<Step> = BinaryHeap::from([initial_step]);
    let mut visited: Vec<[Option<(usize, u64)>; 4]> = vec![[None; 4]; maze.map.len()];

    let mut found: Option<(usize, u64)> = None;

    while let Some(step) = queue.pop() {
        if let Some((_, found_cost)) = found {
            if step.cost >= found_cost {
                continue;
            }
        }

        let v = visited.get_mut(step.coord).unwrap();
        if v[step.dir.index()].is_some() {
            continue;
        } else if let Some(prev) = step.prev {
            v[step.dir.index()] = Some((prev, step.cost));
        }

        let c = maze.map.get(step.coord).unwrap();

        match c {
            'E' => {
                if multiple {
                    found = Some((step.coord, step.cost));
                } else {
                    return (step.cost, 0);
                }
            }

            '#' => {
                continue;
            }

            '.' | 'S' => {
                for dir in step.dir.left_and_right() {
                    queue.push(Step {
                        coord: maze.next(step.coord, &dir),
                        dir,
                        cost: step.cost + 1001,
                        prev: Some(step.coord),
                    });
                }

                queue.push(Step {
                    coord: maze.next(step.coord, &step.dir),
                    dir: step.dir,
                    cost: step.cost + 1,
                    prev: Some(step.coord),
                });
            }

            _ => panic!("Unexpected character: {}", c),
        }
    }

    if multiple {
        if let Some((found_coord, score)) = found {
            let mut set = HashSet::new();
            let mut queue = vec![(found_coord, score + 1, None)];
            let mut nodes = 0;
            while let Some((prev_coord, prev_score, prev_dir)) = queue.pop() {
                if set.insert(prev_coord) {
                    nodes += 1;
                    let v = visited.get(prev_coord).unwrap();
                    for (dir_index, val) in v.iter().enumerate() {
                        if let Some((p, s)) = val {
                            let diff_required = match prev_dir {
                                Some(prev_dir) => {
                                    if prev_dir == dir_index {
                                        0
                                    } else {
                                        1000
                                    }
                                }
                                None => 0,
                            };

                            if *s < prev_score - diff_required {
                                queue.push((*p, *s, Some(dir_index)));
                            }
                        }
                    }
                }
            }

            return (score, nodes);
        }

        (0, 0)
    } else {
        (0, 0)
    }
}

#[derive(Debug)]
pub struct Maze {
    start: usize,
    width: usize,
    map: Vec<char>,
}

impl Maze {
    fn next(&self, from: usize, dir: &Dir) -> usize {
        match dir {
            Dir::North => from - self.width,
            Dir::South => from + self.width,
            Dir::East => from + 1,
            Dir::West => from - 1,
        }
    }
}

impl Maze {
    #[allow(dead_code)]
    fn print(&self, marked: &HashSet<usize>) {
        for (i, c) in self.map.iter().enumerate() {
            if marked.contains(&i) {
                print!("O");
            } else {
                print!("{}", c);
            }
            if (i + 1) % self.width == 0 {
                println!();
            }
        }
    }
}
#[derive(Debug, Eq)]
struct Step {
    coord: usize,
    dir: Dir,
    cost: u64,
    prev: Option<usize>,
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord && self.dir == other.dir
    }
}

impl PartialOrd<Self> for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn left_and_right(&self) -> [Dir; 2] {
        match self {
            Dir::North => [Dir::West, Dir::East],
            Dir::South => [Dir::East, Dir::West],
            Dir::East => [Dir::North, Dir::South],
            Dir::West => [Dir::South, Dir::North],
        }
    }

    fn index(&self) -> usize {
        match self {
            Dir::North => 0,
            Dir::South => 1,
            Dir::East => 2,
            Dir::West => 3,
        }
    }
}
