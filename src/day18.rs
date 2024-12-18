use crate::advent::Solver;
use std::collections::VecDeque;

pub struct Day18;

impl Solver for Day18 {
    type Input = Memory;

    fn parse(&self, input: &str, is_sample: bool) -> Self::Input {
        let dimension = if is_sample { 7 } else { 71 };
        let how_many = if is_sample { 12 } else { 1024 };

        let drops: Vec<usize> = input
            .lines()
            .map(|line| {
                let mut split = line.split(",");
                let x: usize = split.next().unwrap().parse().unwrap();
                let y: usize = split.next().unwrap().parse().unwrap();
                x + y * dimension
            })
            .collect();

        Memory {
            drops,
            dimension,
            how_many,
        }
    }

    fn part_1(&self, memory: &mut Self::Input) -> String {
        let mut space = vec![false; memory.dimension * memory.dimension];
        for i in memory.drops.iter().take(memory.how_many) {
            space[*i] = true;
        }

        memory.find_path_breadth_first(&mut space).unwrap().to_string()
    }

    fn part_2(&self, memory: &mut Self::Input) -> String {
        let mut low = 0;
        let mut high = memory.drops.len() - 1;

        while low < high {
            let mid = (low + high) / 2;
            let mut space = vec![false; memory.dimension * memory.dimension];
            for i in memory.drops.iter().take(mid) {
                space[*i] = true;
            }

            if memory.find_path_breadth_first(&mut space).is_some() {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        let v = memory.drops[high - 1];
        format!("{},{}", v % memory.dimension, v / memory.dimension)
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("272", "16,44")
    }

    fn name(&self) -> &'static str {
        "RAM Run"
    }
}

pub struct Memory {
    drops: Vec<usize>,
    dimension: usize,
    how_many: usize,
}

impl Memory {
    fn next_coord(&self, old_coord: usize) -> [Option<usize>; 4] {
        [
            if old_coord % self.dimension > 0 {
                Some(old_coord - 1)
            } else {
                None
            },
            if old_coord % self.dimension < self.dimension - 1 {
                Some(old_coord + 1)
            } else {
                None
            },
            if old_coord >= self.dimension {
                Some(old_coord - self.dimension)
            } else {
                None
            },
            if old_coord < self.dimension * (self.dimension - 1) {
                Some(old_coord + self.dimension)
            } else {
                None
            },
        ]
    }

    fn find_path_breadth_first(&self, space: &mut [bool]) -> Option<usize> {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::from(vec![(0, 0)]);
        let goal = self.dimension * self.dimension - 1;
        while let Some((coord, steps)) = queue.pop_back() {
            if coord == goal {
                return Some(steps);
            }

            let s = space.get_mut(coord).unwrap();
            if *s {
                continue;
            }
            *s = true;

            for new_coord in self.next_coord(coord).into_iter().flatten() {
                queue.push_front((new_coord, steps + 1));
            }
        }
        None
    }
}
