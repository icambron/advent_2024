use crate::advent::Solver;
use hashbrown::HashSet;

pub struct Day12;

impl Solver for Day12 {
    fn run(&self, input: &str) -> (u64, u64) {
        let map = parse(input);
        let (p1, p2) = part_1(&map);

        (p1, p2)
    }

    fn expected(&self) -> (u64, u64) {
        (1319878, 784982)
    }
}

fn part_1(map: &Map) -> (u64, u64) {
    let mut global_visited: HashSet<usize> = HashSet::new();
    let mut local_visited: HashSet<usize> = HashSet::new();
    let mut total_with_perim = 0;
    let mut total_with_sides = 0;
    let mut search_stack: Vec<(usize, char)> = Vec::new();

    for i in 0..map.chars.len() {
        if global_visited.contains(&i) {
            continue;
        }

        let mut area = 0;
        let mut perim = 0;
        let mut sides = 0;

        let base_char = map.chars[i];

        search_stack.push((i, base_char));
        local_visited.clear();

        while let Some((j, _)) = search_stack.pop() {
            if global_visited.contains(&j) || local_visited.contains(&j) {
                continue;
            }

            area += 1;

            local_visited.insert(j);

            let n = map.navigate(j);
            let mut last_was_fence = false;
            let mut first_was_fence = false;

            for (k, next) in [n.up, n.right, n.down, n.left]
                .iter()
                .map(|o| conflate_missing(o, base_char))
                .enumerate()
            {
                if let Some(next) = next {
                    last_was_fence = false;
                    search_stack.push(next);
                } else {
                    if k == 0 {
                        first_was_fence = true;
                    }

                    if last_was_fence {
                        sides += 1;
                    }

                    perim += 1;
                    last_was_fence = true
                }
            }

            if first_was_fence && last_was_fence {
                sides += 1;
            }

            for (yes_1, yes_2, no) in [
                (n.up, n.right, n.up_right),
                (n.up, n.left, n.up_left),
                (n.down, n.left, n.down_left),
                (n.down, n.right, n.down_right),
            ]
            .iter()
            {
                if conflate_missing(&yes_1, base_char).is_some()
                    && conflate_missing(&yes_2, base_char).is_some()
                    && conflate_missing(&no, base_char).is_none()
                {
                    sides += 1;
                }
            }
        }

        global_visited.extend(local_visited.iter());
        total_with_perim += area * perim;
        total_with_sides += area * sides;
    }

    (total_with_perim, total_with_sides)
}

fn conflate_missing(op: &Option<(usize, char)>, expected_char: char) -> Option<(usize, char)> {
    op.and_then(|next| if next.1 == expected_char { Some(next) } else { None })
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
    fn navigate(&self, point: usize) -> Navigation {
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
