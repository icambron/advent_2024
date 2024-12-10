use crate::advent::Solver;

pub struct Day09;

impl Solver for Day09 {
    fn run(&self, input: &str) -> (u64, u64) {
        let (blocks, slots) = parse(input);
        let p1 = part_1(&slots);
        let p2 = part_2(&blocks, slots);
        (p1, p2)
    }

    fn expected(&self) -> (u64, u64) {
        (6344673854800, 6360363199987)
    }
}

fn part_1(slots: &[Slot]) -> u64 {
    let mut reversed = slots
        .iter()
        .rev()
        .enumerate()
        .filter(|(_, slot)| matches!(slot, Slot::File(_)));

    let mut checksum = 0;
    let mut high_j = 0;
    for (i, slot) in slots.iter().enumerate() {
        match slot {
            Slot::Empty => {
                if let Some((j, revered_block)) = reversed.next() {
                    high_j = j + 1;
                    if j >= slots.len() - i {
                        break;
                    }

                    if let Slot::File(id) = revered_block {
                        checksum += i * *id;
                    }
                }
            }
            Slot::File(id) => {
                if high_j >= slots.len() - i {
                    break;
                }
                checksum += i * *id;
            }
        }
    }

    checksum as u64
}

fn part_2(blocks: &[Block], mut slots: Vec<Slot>) -> u64 {
    let mut free: Vec<Block> = blocks
        .iter()
        .filter(|b| matches!(b.slot, Slot::Empty))
        .cloned()
        .collect();

    for block in blocks.iter().rev() {
        if let Slot::File(id) = block.slot {
            if let Some((free_index, free_block)) = free
                .iter_mut()
                .enumerate()
                .find(|(_, b)| b.size >= block.size)
            {
                // don't move blocks forward
                if free_block.start > block.start {
                    continue;
                }

                let new_slice = &mut slots[free_block.start..free_block.start + block.size];
                new_slice.fill(Slot::File(id));

                let old_slice = &mut slots[block.start..block.start + block.size];
                old_slice.fill(Slot::Empty);

                if free_block.size > block.size {
                    free_block.size -= block.size;
                    free_block.start += block.size;
                } else {
                    free.remove(free_index);
                }
            }
        }
    }

    slots
        .iter()
        .enumerate()
        .map(|(i, slot)| {
            if let Slot::File(id) = slot {
                (i * *id) as u64
            } else {
                0
            }
        })
        .sum()
}

fn parse(input: &str) -> (Vec<Block>, Vec<Slot>) {
    let mut chars = input.chars();
    let mut id = 0;
    let mut index = 0;
    let mut blocks = Vec::new();
    let mut slots = Vec::new();

    while let Some(filled) = chars.next().and_then(|c| c.to_digit(10)) {
        blocks.push(Block {
            start: index,
            size: filled as usize,
            slot: Slot::File(id),
        });
        for _ in 0..filled {
            slots.push(Slot::File(id));
        }

        index += filled as usize;

        if let Some(whitespace) = chars.next().and_then(|c| c.to_digit(10)) {
            blocks.push(Block {
                start: index,
                size: whitespace as usize,
                slot: Slot::Empty,
            });
            for _ in 0..whitespace {
                slots.push(Slot::Empty);
            }
            index += whitespace as usize;
        }

        id += 1;
    }

    (blocks, slots)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Slot {
    Empty,
    File(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub start: usize,
    pub size: usize,
    pub slot: Slot,
}
