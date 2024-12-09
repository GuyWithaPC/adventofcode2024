use itertools::{repeat_n, Itertools};

crate::day!("Disk Fragmenter" => {
    part_1
});

fn part_1(data: &str) -> usize {
    let file_map: Vec<Option<usize>> = data.chars().filter(|&c| c != '\n').batching(|it| {
        if let Some(a) = it.next() {
            if let Some(b) = it.next() {
                Some((a.to_digit(10).unwrap(), b.to_digit(10).unwrap()))
            } else {
                Some((a.to_digit(10).unwrap(), 0))
            }
        } else {
            None
        }
    }).enumerate()
        .map(|(i, (file_len, empty_len))| {
            repeat_n(Some(i), file_len as usize).chain(repeat_n(None, empty_len as usize))
        })
        .flatten()
        .collect();
    let block_count = file_map.iter().filter(|&&x| x.is_some()).count();
    let rev_files = file_map.clone();
    let mut end_files = rev_files.iter().rev()
        .filter_map(|&x| x);
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

crate::test_day!(
"
2333133121414131402
",
{
    part_1 => 1928
}
);