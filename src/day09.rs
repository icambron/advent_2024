use crate::advent::Solver;

pub struct Day09;

impl Solver for Day09 {
    fn part_1(&self, input: &str) -> u64 {
        let (_, slots) = parse(input);
        part_1(&slots)
    }

    fn part_2(&self, input: &str) -> u64 {
        let (blocks, _) = parse(input);
        part_2(&blocks)
    }

    fn expected(&self) -> (u64, u64) {
        (6344673854800, 6360363199987)
    }
}

fn part_1(slots: &[Slot]) -> u64 {
    let mut reversed = slots.iter().rev().enumerate().filter(|(_, slot)| matches!(slot, Slot::File(_)));

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

fn part_2(blocks: &[Block]) -> u64 {
    let mut free: Vec<Block> = blocks.iter().filter(|b| matches!(b.slot, Slot::Empty)).cloned().collect();
    let mut offsets: [usize; 9] = [0; 9];

    let mut total: usize = 0;

    for block in blocks.iter().filter(|b| matches!(b.slot, Slot::File(_))).rev() {
        if let Slot::File(id) = block.slot {
            let offset = offsets[block.size - 1];

            if offset > block.start {
                total += sum(block.start, block.size, id);
                continue;
            }

            if let Some((free_index, free_block)) = free.iter_mut().enumerate().skip(offset).find(|(_, b)| b.size >= block.size) {
                // don't move blocks forward
                if free_block.start > block.start {
                    offsets[block.size - 1] = usize::MAX;
                    total += sum(block.start, block.size, id);
                    continue;
                }

                offsets[block.size - 1] = free_index;

                total += sum(free_block.start, block.size, id);

                // huge optimization -- we DON'T remove completely used blocks from the free list, because
                // that turns out to be expensive. Just zero them out and depend on the offsets to usually skip them
                free_block.size -= block.size;
                free_block.start += block.size;
            } else {
                total += sum(block.start, block.size, id);
                offsets[block.size - 1] = usize::MAX;
            }
        }
    }

    total as u64
}

fn sum(start: usize, count: usize, id: usize) -> usize {
    let m = start + count - 1;
    count * (start + m) / 2 * id
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
