My [Advent of Code](https://adventofcode.com/2024/) solutions for 2024.

# Goals

Just having fun. Not competitive on any metric. Aiming for:

1. Correct
2. Elegant and idiomatic
3. Reasonably performant, aiming for sub 200ms total on my Macbook.

# Running

```
# Run the given day
cargo run -- -d <day>

# Run the given day with sample input
cargo run -- -d <day> -s

# Run all days
cargo run -- -a

# The -c flag checks the outputs, works on all or individual days
cargo run -- -a -c
```

# Performance

To measure its speed, build it with the release flag:

```
cargo build --release
```

Then run it:

```
❯ ./target/release/advent_2024 -a -c
+-----+--------------------------------------+------------+-------------+-------------+------------+
| Day | Name                                 | Parse (µs) | Part 1 (µs) | Part 2 (µs) | Total (µs) |
+-----+--------------------------------------+------------+-------------+-------------+------------+
|   1 | Historian Hysteria (list similarity) |        281 |           0 |          63 |        344 |
|   2 | Red-Nosed Reports                    |         97 |          10 |          41 |        148 |
|   3 | Mull It Over                         |        852 |           1 |           1 |        854 |
|   4 | Ceres Search (finding XMAS)          |         39 |         507 |         287 |        833 |
|   5 | Print Queue                          |        336 |        3775 |        3635 |       7746 |
|   6 | Guard Gallivant                      |        136 |         712 |       14525 |      15373 |
|   7 | Bridge Repair (equations)            |        425 |         285 |        1389 |       2099 |
|   8 | Resonant Collinearity                |         17 |          19 |         460 |        496 |
|   9 | Disk Fragmenter                      |        476 |         245 |         171 |        892 |
|  10 | Hoof It (topographic map)            |        139 |         672 |         639 |       1450 |
|  11 | Plutonian Pebbles                    |          0 |         671 |        3848 |       4519 |
|  12 | Garden Groups                        |         31 |         407 |         532 |        970 |
|  13 | Claw Contraption                     |        139 |           3 |           3 |        145 |
|  14 | Restroom Rebound                     |        394 |           2 |        7645 |       8041 |
|  15 | Warehouse Woes                       |         50 |        1268 |        1352 |       2670 |
|  16 | Reindeer Maze                        |         36 |        2900 |        2765 |       5701 |
|  17 | Chronospacial Computer               |          1 |           1 |          79 |         81 |
|  18 | RAM Run                              |        156 |         100 |         211 |        467 |
|  19 | Linen Layout                         |         54 |       21199 |       29676 |      50929 |
|  20 | Race Condition                       |         95 |         263 |       16078 |      16436 |
|  21 | Keypad Conundrum                     |         55 |          11 |          80 |        146 |
|  22 | Monkey Market                        |         33 |        5743 |       19575 |      25351 |
|  23 | LAN Party                            |        292 |        1266 |        9206 |      10764 |
|  24 | Crossed Wires                        |       1934 |          32 |          29 |       1995 |
|  25 | Code Chronicle                       |         84 |         228 |           0 |        312 |
+-----+--------------------------------------+------------+-------------+-------------+------------+
Total: 158.794341ms
```

Not winning any speed competitions, but pretty good!
