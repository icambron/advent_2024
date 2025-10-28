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
+-----+------------------------+------------+-------------+-------------+------------+
| Day | Name                   | Parse (µs) | Part 1 (µs) | Part 2 (µs) | Total (µs) |
+-----+------------------------+------------+-------------+-------------+------------+
|   1 | Historian Hysteria     |        141 |           0 |          63 |        204 |
|   2 | Red-Nosed Reports      |         94 |           9 |          39 |        142 |
|   3 | Mull It Over           |       1582 |           2 |           3 |       1587 |
|   4 | Ceres Search           |        111 |         672 |          96 |        879 |
|   5 | Print Queue            |        111 |        1459 |        3367 |       4937 |
|   6 | Guard Gallivant        |        164 |         779 |       15479 |      16422 |
|   7 | Bridge Repair          |        424 |         281 |        1385 |       2090 |
|   8 | Resonant Collinearity  |         17 |          19 |         471 |        507 |
|   9 | Disk Fragmenter        |        483 |         245 |         171 |        899 |
|  10 | Hoof It                |        139 |         657 |         640 |       1436 |
|  11 | Plutonian Pebbles      |          0 |         775 |        4049 |       4824 |
|  12 | Garden Groups          |         31 |         411 |         534 |        976 |
|  13 | Claw Contraption       |        139 |           3 |           3 |        145 |
|  14 | Restroom Rebound       |        391 |           2 |        7716 |       8109 |
|  15 | Warehouse Woes         |         38 |         969 |        1022 |       2029 |
|  16 | Reindeer Maze          |         27 |        2395 |        2959 |       5381 |
|  17 | Chronospacial Computer |          1 |           1 |          79 |         81 |
|  18 | RAM Run                |        158 |          98 |         217 |        473 |
|  19 | Linen Layout           |         53 |       21396 |       29765 |      51214 |
|  20 | Race Condition         |         92 |         261 |       15868 |      16221 |
|  21 | Keypad Conundrum       |         32 |           9 |          79 |        120 |
|  22 | Monkey Market          |         31 |        5711 |       19289 |      25031 |
|  23 | LAN Party              |        307 |        1244 |        9087 |      10638 |
|  24 | Crossed Wires          |       1920 |          37 |          30 |       1987 |
|  25 | Code Chronicle         |         84 |         228 |           0 |        312 |
+-----+------------------------+------------+-------------+-------------+------------+
Total: 156.678996ms
```

Not winning any speed competitions, but pretty good!
