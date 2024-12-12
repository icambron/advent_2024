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
        let mut sides: u64 = 0;

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
            for neighbor in [n.up, n.down, n.left, n.right] {
                if let Some(next) = neighbor {
                    search_stack.push(next.0)
                } else {
                    perim += 1
                };
            }

            // convex corners
            sides += [(n.up, n.right), (n.up, n.left), (n.down, n.left), (n.down, n.right)]
                .iter()
                .filter(|&(no_1, no_2)| no_1.is_none() && no_2.is_none())
                .count() as u64;

            // concave corners
            sides += [
                (n.left, n.up, (-1, -1)),
                (n.right, n.up, (1, -1)),
                (n.left, n.down, (-1, 1)),
                (n.right, n.down, (1, 1)),
            ]
            .iter()
            .filter(|(yes_1, yes_2, no)| yes_1.is_some() && yes_2.is_some() && n.partial.neighbor(no.0, no.1).is_none())
            .count() as u64;
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
    Map {
        chars,
        width,
        height: lines.len(),
    }
}

struct Map {
    chars: Vec<char>,
    width: usize,
    height: usize,
}

struct NavPartial<'a> {
    expected: char,
    map: &'a Map,
    x: usize,
    y: usize,
    height: usize,
}

impl<'a> NavPartial<'a> {
    fn neighbor(&self, dx: isize, dy: isize) -> Option<(usize, char)> {
        let nx = self.x as isize + dx;
        let ny = self.y as isize + dy;
        if nx >= 0 && nx < self.map.width as isize && ny >= 0 && ny < self.height as isize {
            let idx = (ny as usize) * self.map.width + (nx as usize);
            let c = self.map.chars[idx];
            if c == self.expected {
                Some((idx, c))
            } else {
                None
            }
        } else {
            None
        }
    }
}

struct Navigation<'a> {
    up: Option<(usize, char)>,
    down: Option<(usize, char)>,
    left: Option<(usize, char)>,
    right: Option<(usize, char)>,
    partial: NavPartial<'a>,
}

impl Map {
    fn navigate(&self, coord: usize, expected: char) -> Navigation {
        let height = self.chars.len() / self.width;
        let x = coord % self.width;
        let y = coord / self.width;

        let partial = NavPartial {
            x,
            y,
            height,
            map: self,
            expected,
        };

        Navigation {
            up: partial.neighbor(0, -1),
            down: partial.neighbor(0, 1),
            left: partial.neighbor(-1, 0),
            right: partial.neighbor(1, 0),
            partial,
        }
    }
}
