use crate::{
    blocks::sequence_section::Sequence,
    fse::fse_encoder::{
        build_probability_table_for_estimate, build_table_from_probabilities,
        normalize_counts_with_table_log, optimal_table_log, FSETable,
    },
};

use crate::encoding::blocks::compressed::sequence_cost::CodeCounts;

#[derive(Clone, Copy)]
pub(in crate::encoding::blocks::compressed) enum TableBuilder {
    Full,
    ProbabilityOnly,
}

pub(in crate::encoding::blocks::compressed) fn build_sequence_table(
    sequences: &[Sequence],
    code: impl Fn(&Sequence) -> u8 + Copy,
    max_log: u8,
    table_builder: TableBuilder,
) -> FSETable {
    let mut counts = [0usize; 256];
    for sequence in sequences {
        counts[usize::from(code(sequence))] += 1;
    }

    let last_code = usize::from(code(&sequences[sequences.len() - 1]));
    build_sequence_table_from_raw_counts(&mut counts, last_code, max_log, table_builder)
}

pub(in crate::encoding::blocks::compressed) fn build_sequence_table_from_counts(
    counts: &CodeCounts,
    last_code: u8,
    max_log: u8,
    table_builder: TableBuilder,
) -> FSETable {
    let mut counts = *counts.counts();
    build_sequence_table_from_raw_counts(
        &mut counts,
        usize::from(last_code),
        max_log,
        table_builder,
    )
}

fn build_sequence_table_from_raw_counts(
    counts: &mut [usize; 256],
    last_code: usize,
    max_log: u8,
    table_builder: TableBuilder,
) -> FSETable {
    let original_total = counts.iter().sum::<usize>();
    let original_max_symbol = counts
        .iter()
        .rposition(|count| *count != 0)
        .unwrap_or(last_code);
    let table_log = optimal_table_log(max_log, original_total, original_max_symbol);

    // The final sequence initializes the FSE state. C zstd removes one
    // duplicated final code before normalizing compressed sequence tables, but
    // computes the FSE table log from the original sequence count first.
    if counts[last_code] > 1 {
        counts[last_code] -= 1;
    }

    let max_symbol = counts
        .iter()
        .rposition(|count| *count != 0)
        .unwrap_or(last_code);
    let adjusted_total = counts[..=max_symbol].iter().sum::<usize>();
    let low_prob_count = if adjusted_total >= 2048 { -1 } else { 1 };
    let (probs, acc_log) = normalize_counts_with_table_log(
        &counts[..=max_symbol],
        table_log,
        max_log,
        low_prob_count,
        true,
    );
    build_table_from_probabilities_with_builder(&probs, acc_log, table_builder)
}

pub(in crate::encoding::blocks::compressed) fn build_table_from_probabilities_with_builder(
    probs: &[i32],
    acc_log: u8,
    table_builder: TableBuilder,
) -> FSETable {
    match table_builder {
        TableBuilder::Full => build_table_from_probabilities(probs, acc_log),
        TableBuilder::ProbabilityOnly => build_probability_table_for_estimate(probs, acc_log),
    }
}
