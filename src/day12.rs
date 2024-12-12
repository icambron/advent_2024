use crate::advent::Solver;

pub struct Day12;

impl Solver for Day12 {
    fn run(&self, input: &str) -> (u64, u64) {
        let map = parse(input);
        let (p1, p2) = solve(&map);

        (p1, p2)
    }

    fn expected(&self) -> (u64, u64) {
        (1319878, 784982)
    }
}

fn solve(map: &Map) -> (u64, u64) {
    let mut visited: Vec<bool> = vec![false; map.width * map.height];
    let mut search_stack: Vec<usize> = Vec::new();
    let mut total_with_perim = 0;
    let mut total_with_sides = 0;

    for i in 0..map.chars.len() {

        if visited[i] {
            continue;
        }

        let mut area = 0;
        let mut perim = 0;
        let mut sides = 0;

        let c = map.chars[i];
        search_stack.push(i);

        while let Some(j) = search_stack.pop() {
            let visit_status = visited.get_mut(j).unwrap();

            if *visit_status {
                continue;
            }

            *visit_status = true;
            area += 1;

            let n = map.navigate(j, c);

            // directions to explore
            for neighbor in [
                n.up, n.down, n.left, n.right
            ] {
                match neighbor {
                    Some(next) => search_stack.push(next.0),
                    None => perim += 1
                };
            }

            // convex corners
            for (no_1, no_2) in [
                (n.up, n.right),
                (n.up, n.left),
                (n.down, n.left),
                (n.down, n.right),
            ] {
                if no_1.is_none() && no_2.is_none() {
                    sides += 1;
                }
            }

            // concave corners
            for (yes_1, yes_2, no) in [
                (n.up, n.right, n.up_right),
                (n.up, n.left, n.up_left),
                (n.down, n.left, n.down_left),
                (n.down, n.right, n.down_right),
            ]
            {
                if yes_1.is_some() && yes_2.is_some() && no.is_none() {
                    sides += 1;
                }
            }
        }

        total_with_perim += area * perim;
        total_with_sides += area * sides;
    }

    (total_with_perim, total_with_sides)
}

fn parse(input: &str) -> Map {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines.first().map_or(0, |line| line.len());
    let chars: Vec<char> = lines.concat().chars().collect();
    Map { chars, width, height: lines.len() }
}

struct Map {
    chars: Vec<char>,
    width: usize,
    height: usize,
}

struct Navigation {
    up: Option<(usize, char)>,
    down: Option<(usize, char)>,
    left: Option<(usize, char)>,
    right: Option<(usize, char)>,
    up_left: Option<(usize, char)>,
    up_right: Option<(usize, char)>,
    down_left: Option<(usize, char)>,
    down_right: Option<(usize, char)>,
}

impl Map {
    fn navigate(&self, coord: usize, expected: char) -> Navigation {
        let height = self.chars.len() / self.width;
        let x = coord % self.width;
        let y = coord / self.width;

        let neighbor = |dx: isize, dy: isize| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < height as isize {
                let idx = (ny as usize) * self.width + (nx as usize);
                let c = self.chars[idx];
                if c == expected {
                    Some((idx, c))
                } else {
                    None
                }
            } else {
                None
            }
        };

        Navigation {
            up: neighbor(0, -1),
            down: neighbor(0, 1),
            left: neighbor(-1, 0),
            right: neighbor(1, 0),
            up_left: neighbor(-1, -1),
            up_right: neighbor(1, -1),
            down_left: neighbor(-1, 1),
            down_right: neighbor(1, 1),
        }
    }
}
