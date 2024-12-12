use crate::advent::Solver;
use hashbrown::HashSet;

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
    let mut visited: HashSet<usize> = HashSet::new();
    let mut total_with_perim = 0;
    let mut total_with_sides = 0;
    let mut search_stack: Vec<usize> = Vec::new();

    for i in 0..map.chars.len() {
        if visited.contains(&i) {
            continue;
        }

        let mut area = 0;
        let mut perim = 0;
        let mut sides = 0;

        let c = map.chars[i];

        search_stack.push(i);

        while let Some(j) = search_stack.pop() {
            if visited.contains(&j) {
                continue;
            }

            visited.insert(j);
            area += 1;

            let n = map.navigate(j, c);

            for neighbor in [
                n.up, n.down, n.left, n.right
            ] {
                match neighbor {
                    Some(next) => search_stack.push(next.0),
                    None => perim += 1
                };
            }

            // concave corners
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

            // convex corners
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

        // global_visited.extend(local_visited.iter());
        total_with_perim += area * perim;
        total_with_sides += area * sides;
    }

    (total_with_perim, total_with_sides)
}

fn parse(input: &str) -> Map {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines.first().map_or(0, |line| line.len());
    let chars: Vec<char> = lines.concat().chars().collect();
    Map { chars, width }
}

struct Map {
    chars: Vec<char>,
    width: usize,
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

    fn navigate(&self, point: usize, expected: char) -> Navigation {
        let height = self.chars.len() / self.width;
        let x = point % self.width;
        let y = point / self.width;

        let neighbor = |dx: isize, dy: isize| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < height as isize {
                let idx = (ny as usize) * self.width + (nx as usize);
                Some((idx, self.chars[idx]))
            } else {
                None
            }
        };

        Navigation {
            up: smash(neighbor(0, -1), expected),
            down: smash(neighbor(0, 1), expected),
            left: smash(neighbor(-1, 0), expected),
            right: smash(neighbor(1, 0), expected),
            up_left: smash(neighbor(-1, -1), expected),
            up_right: smash(neighbor(1, -1), expected),
            down_left: smash(neighbor(-1, 1), expected),
            down_right: smash(neighbor(1, 1), expected),
        }
    }
}

fn smash(op: Option<(usize, char)>, expected_char: char) -> Option<(usize, char)> {
    op.and_then(|next| if next.1 == expected_char { Some(next) } else { None })
}
