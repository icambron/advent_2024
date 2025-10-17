use crate::advent::Solver;

pub struct Day12;

impl Solver for Day12 {
    type Input = Map;

    fn parse(&self, input: &str, _: bool) -> Self::Input {
        let lines: Vec<&str> = input.lines().collect();
        let width = lines.first().map_or(0, |line| line.len());
        let chars: Vec<char> = lines.concat().chars().collect();
        Map {
            chars,
            width,
            height: lines.len(),
        }
    }

    fn part_1(&self, input: &mut Self::Input) -> String {
        solve(input, false).to_string()
    }

    fn part_2(&self, input: &mut Self::Input) -> String {
        solve(input, true).to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("1319878", "784982")
    }

    fn name(&self) -> &'static str {
        "Garden Groups"
    }
}

fn solve(map: &Map, count_sides: bool) -> u64 {
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

            if count_sides {
                // convex corners
                sides += [(n.up, n.right), (n.up, n.left), (n.down, n.left), (n.down, n.right)]
                    .iter()
                    .filter(|&(no_1, no_2)| no_1.is_none() && no_2.is_none())
                    .count();

                // concave corners
                sides += [
                    (n.left, n.up, (-1, -1)),
                    (n.right, n.up, (1, -1)),
                    (n.left, n.down, (-1, 1)),
                    (n.right, n.down, (1, 1)),
                ]
                .iter()
                .filter(|(yes_1, yes_2, no)| yes_1.is_some() && yes_2.is_some() && n.partial.neighbor(no.0, no.1).is_none())
                .count();
            }

            // directions to explore
            for neighbor in [n.up, n.down, n.left, n.right] {
                if let Some(next) = neighbor {
                    search_stack.push(next)
                } else if !count_sides {
                    perim += 1
                };
            }
        }

        if count_sides {
            total_with_sides += area * sides;
        } else {
            total_with_perim += area * perim;
        }
    }

    if count_sides {
        total_with_sides as u64
    } else {
        total_with_perim as u64
    }
}

pub struct Map {
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
    fn neighbor(&self, dx: isize, dy: isize) -> Option<usize> {
        let nx = self.x as isize + dx;
        let ny = self.y as isize + dy;
        if nx >= 0 && nx < self.map.width as isize && ny >= 0 && ny < self.height as isize {
            let idx = (ny as usize) * self.map.width + (nx as usize);
            let c = self.map.chars[idx];
            if c == self.expected {
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

struct Navigation<'a> {
    up: Option<usize>,
    down: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    partial: NavPartial<'a>,
}

impl Map {
    fn navigate(&self, coord: usize, expected: char) -> Navigation<'_> {
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
