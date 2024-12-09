pub fn part1(contents: &str) -> u64 {
    let mut memory: Vec<Option<u32>> = Vec::new();
    for (i, value) in contents.chars().map(|c| { c.to_digit(10).unwrap() }).enumerate() {
        if i % 2 == 0 {
            for _ in 0..value {
                memory.push(Some((i / 2) as u32));
            }
        } else {
            for _ in 0..value {
                memory.push(None);
            }
        }
    }
    let mut front: usize = 0;
    let mut back: usize = memory.len() - 1;
    while front < back {
        if memory[front].is_some() {
            front += 1;
            continue;
        }

        memory[front] = memory[back];
        memory[back] = None;
        while memory[back].is_none() {
            back -= 1;
        }
        front += 1;
    }

    memory.iter()
        .enumerate()
        .filter_map(|(i, x)| x.map(|v| v as u64 * i as u64))
        .sum()
}

pub fn part2(contents: &str) -> u64 {
    let mut occupied_blocks: Vec<Block> = Vec::new();
    let mut empty_blocks: Vec<Block> = Vec::new();
    let mut memory: Vec<Option<u32>> = Vec::new();
    for (i, value) in contents.chars().map(|c| { c.to_digit(10).unwrap() }).enumerate() {
        if i % 2 == 0 {
            occupied_blocks.push(Block { start: memory.len(), size: value as usize });
            for _ in 0..value {
                memory.push(Some((i / 2) as u32));
            }
        } else {
            empty_blocks.push(Block { start: memory.len(), size: value as usize });
            for _ in 0..value {
                memory.push(None);
            }
        }
    }

    for occupied_block in occupied_blocks.iter().rev() {
        for empty_block in empty_blocks.iter_mut().filter(|eb| { eb.size > 0 && eb.start < occupied_block.start }) {
            if empty_block.size >= occupied_block.size {
                let mut i = 0;
                while i < occupied_block.size {
                    memory[empty_block.start + i] = memory[occupied_block.start + i];
                    memory[occupied_block.start + i] = None;
                    i += 1;
                }

                // adjust empty block if necessary
                if empty_block.size > occupied_block.size {
                    empty_block.size -= occupied_block.size;
                    empty_block.start += occupied_block.size;
                } else {
                    empty_block.size = 0;
                }

                break;
            }
        }
    }
    
    memory.iter()
        .enumerate()
        .filter_map(|(i, x)| x.map(|v| v as u64 * i as u64))
        .sum()
}

struct Block {
    start: usize,
    size: usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "2333133121414131402";
        assert_eq!(part1(contents), 1928);
    }

    #[test]
    fn test_part2() {
        let contents = "2333133121414131402";
        assert_eq!(part2(contents), 2858);
    }
}
