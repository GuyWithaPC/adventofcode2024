use indicatif::ProgressBar;
use itertools::{repeat_n, Itertools};

crate::day!("Disk Fragmenter" + bars => {
    part_1,
    part_2
});

fn part_1(data: &str, _: &ProgressBar) -> usize {
    let file_map: Vec<Option<usize>> = data
        .chars()
        .filter(|&c| c != '\n')
        .batching(|it| {
            if let Some(a) = it.next() {
                if let Some(b) = it.next() {
                    Some((a.to_digit(10).unwrap(), b.to_digit(10).unwrap()))
                } else {
                    Some((a.to_digit(10).unwrap(), 0))
                }
            } else {
                None
            }
        })
        .enumerate()
        .map(|(i, (file_len, empty_len))| {
            repeat_n(Some(i), file_len as usize).chain(repeat_n(None, empty_len as usize))
        })
        .flatten()
        .collect();
    let block_count = file_map.iter().filter(|&&x| x.is_some()).count();
    let rev_files = file_map.clone();
    let mut end_files = rev_files.iter().rev().filter_map(|&x| x);
    let mut reconstructed: Vec<usize> = Vec::with_capacity(block_count);
    for straight_id in file_map {
        if let Some(id) = straight_id {
            reconstructed.push(id);
        } else {
            reconstructed.push(end_files.next().unwrap())
        }
        if reconstructed.len() == block_count {
            break;
        }
    }
    reconstructed.iter().enumerate().map(|(i, &x)| i * x).sum()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Block {
    Full(usize, usize),
    Empty(usize),
}

fn part_2(data: &str, bar: &ProgressBar) -> usize {
    let mut blocks: Vec<Block> = data
        .chars()
        .filter(|&c| c != '\n')
        .batching(|it| {
            if let Some(a) = it.next() {
                if let Some(b) = it.next() {
                    Some((
                        a.to_digit(10).unwrap() as usize,
                        b.to_digit(10).unwrap() as usize,
                    ))
                } else {
                    Some((a.to_digit(10).unwrap() as usize, 0usize))
                }
            } else {
                None
            }
        })
        .enumerate()
        .map(|(i, (full, empty))| vec![Block::Full(i, full), Block::Empty(empty)])
        .flatten()
        .collect();
    let mut consolidate = blocks.clone();
    blocks.reverse();
    bar.set_length(blocks.len() as u64);
    for block in blocks {
        if let Block::Full(i, full_size) = block {
            let (found, full_block, empty_block, index) = {
                if let Some((index, Block::Empty(empty_size))) = consolidate
                    .iter()
                    .enumerate()
                    .take_while(|(_, &b)| match b {
                        Block::Full(index, _) => index != i,
                        Block::Empty(_) => true,
                    })
                    .find(|(_, &block)| match block {
                        Block::Full(_, _) => false,
                        Block::Empty(size) => size >= full_size,
                    })
                {
                    (
                        true,
                        Block::Full(i, full_size),
                        Block::Empty(*empty_size - full_size),
                        index,
                    )
                } else {
                    (false, Block::Empty(0), Block::Empty(0), 0)
                }
            };
            if found {
                let (moved_index, moved_size) = consolidate
                    .iter()
                    .enumerate()
                    .find(|(_, &b)| b == block)
                    .map(|(i, &b)| {
                        (
                            i,
                            match b {
                                Block::Full(_, s) => s,
                                Block::Empty(s) => s,
                            },
                        )
                    })
                    .unwrap();
                consolidate.remove(moved_index);
                consolidate.insert(moved_index, Block::Empty(moved_size));
                consolidate.remove(index);
                consolidate.insert(index, full_block);
                consolidate.insert(index + 1, empty_block);
            }
        }
        bar.inc(1);
    }
    consolidate
        .iter()
        .map(|&b| match b {
            Block::Full(index, size) => repeat_n(index, size),
            Block::Empty(size) => repeat_n(0, size),
        })
        .flatten()
        .enumerate()
        .map(|(i, x)| i * x)
        .sum()
}

crate::test_day!(
"
2333133121414131402
" + bars,
{
    part_1 => 1928,
    part_2 => 2858
}
);
