use crate::block_sequence::BlockSequence;
use crate::block_stack::BlockStack;
use crate::BlockCoordinates;
use rustc_hash::{FxHashMap, FxHasher};
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

pub(crate) fn merge_blocks(block_stack: BlockStack) -> Vec<BlockSequence> {
    let mut all_sequences_by_end_y = FxHashMap::default();

    for (y, plate) in block_stack.plates() {
        let mut plane_sequences = vec![];

        for (z, row) in plate.rows() {
            let row_sequences = merge_blocks_x_row(row);

            stretch_sequences_by_z(row_sequences, &mut plane_sequences, z);
        }

        stretch_sequences_by_y(&mut all_sequences_by_end_y, plane_sequences, y);
    }
    let mut all_sequences = vec![];
    for (.., seq) in all_sequences_by_end_y {
        all_sequences.extend(seq);
    }

    all_sequences
}

fn stretch_sequences_by_y(
    all_sequences_by_end_y: &mut HashMap<isize, Vec<BlockSequence>, BuildHasherDefault<FxHasher>>,
    mut current: Vec<BlockSequence>,
    y: isize,
) {
    let prev = all_sequences_by_end_y.get_mut(&(y - 1));

    if let Some(prev_sequences) = prev {
        prev_sequences.retain(|seq| {
            let same_new_seq = current
                .iter_mut()
                .find(|s| s.same_x_size(seq) && s.same_z_size(seq));

            if let Some(current_seq) = same_new_seq {
                current_seq.expand_start(seq.start.clone());
                false
            } else {
                true
            }
        });
    }

    all_sequences_by_end_y.insert(y, current);
}

fn stretch_sequences_by_z(
    row_sequences: Vec<BlockSequence>,
    plane_sequences: &mut Vec<BlockSequence>,
    z: i32,
) {
    let mut prev_row_sequences: Vec<&mut BlockSequence> = plane_sequences
        .iter_mut()
        .filter(|s: &&mut BlockSequence| s.has_z_end_on(z - 1))
        .collect();

    let unique_sequences: Vec<BlockSequence> = row_sequences
        .into_iter()
        .filter_map(|sequence| {
            let same_sequence = prev_row_sequences
                .iter_mut()
                .find(|s| s.same_x_size(&sequence));

            if let Some(same) = same_sequence {
                same.expand_z_end(sequence);
                None
            } else {
                Some(sequence)
            }
        })
        .collect();

    plane_sequences.extend(unique_sequences);
}

fn merge_blocks_x_row(mut row: Vec<BlockCoordinates>) -> Vec<BlockSequence> {
    row.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

    let mut x_sequences = vec![];
    let mut start_block_index = 0;

    for (index, block) in row.iter().enumerate().skip(1) {
        let prev_block = &row[index - 1];
        let stop_concatenation = block.x != prev_block.x + 1;

        if stop_concatenation {
            x_sequences.push(BlockSequence::new(
                row[start_block_index].clone(),
                row[index - 1].clone(),
            ));

            start_block_index = index;
        }
    }
    x_sequences.push(BlockSequence::new(
        row[start_block_index].clone(),
        row[row.len() - 1].clone(),
    ));

    x_sequences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_blocks_x_row_simple() {
        let b = |x| -> BlockCoordinates { BlockCoordinates::new(x, 0, 0) };
        let blocks = vec![b(0), b(1), b(2), b(3), b(4)];
        let result = merge_blocks_x_row(blocks);

        assert_eq!(
            result,
            vec![BlockSequence::new(
                BlockCoordinates::new(0, 0, 0),
                BlockCoordinates::new(4, 0, 0)
            )]
        );
    }
    #[test]
    fn merge_blocks_x_row_multiple() {
        let b = |x| -> BlockCoordinates { BlockCoordinates::new(x, 0, 0) };
        let blocks = vec![b(0), b(1), b(3), b(4)];
        let result = merge_blocks_x_row(blocks);

        assert_eq!(
            result,
            vec![
                BlockSequence::new(
                    BlockCoordinates::new(0, 0, 0),
                    BlockCoordinates::new(1, 0, 0)
                ),
                BlockSequence::new(
                    BlockCoordinates::new(3, 0, 0),
                    BlockCoordinates::new(4, 0, 0)
                )
            ]
        );
    }
}
