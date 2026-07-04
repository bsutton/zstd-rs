use alloc::{vec, vec::Vec};

use crate::{
    blocks::sequence_section::Sequence,
    fse::fse_encoder::{
        build_table_from_probabilities, ncount_cost_from_probabilities,
        normalize_counts_with_table_log, normalized_probabilities_from_counts, optimal_table_log,
        FSETable,
    },
};

use super::{FseTableMode, SequenceModeSearchConfig};
use crate::encoding::blocks::compressed::{
    encode_literal_length, encode_match_len, encode_offset,
    sequence_cost::{cross_entropy_cost, entropy_cost, repeat_table_cost, CodeCounts},
    EXACT_SEQUENCE_TABLE_MIN_LOG,
};

#[derive(Clone, Copy)]
struct TableModeCandidateConfig {
    max_log: u8,
    repeat_table_max_sequences: usize,
    predefined_max_sequences: usize,
    exact_sequence_mode_search: bool,
    selection_policy: TableSelectionPolicy,
}

#[derive(Clone, Copy)]
enum TableSelectionPolicy {
    Legacy,
    CFast { default_norm_log: u8 },
    CCost,
}

#[cfg(test)]
#[allow(clippy::too_many_arguments)]
pub(in crate::encoding::blocks::compressed) fn choose_table<'a>(
    previous: Option<&'a FSETable>,
    default_table: &'a FSETable,
    sequences: &[Sequence],
    code: impl Fn(&Sequence) -> u8 + Copy,
    max_log: u8,
    repeat_table_max_sequences: usize,
    predefined_max_sequences: usize,
) -> FseTableMode<'a> {
    choose_table_with_policy(
        previous,
        default_table,
        sequences,
        code,
        max_log,
        repeat_table_max_sequences,
        predefined_max_sequences,
        TableSelectionPolicy::Legacy,
    )
}

#[allow(clippy::too_many_arguments)]
fn choose_table_with_policy<'a>(
    previous: Option<&'a FSETable>,
    default_table: &'a FSETable,
    sequences: &[Sequence],
    code: impl Fn(&Sequence) -> u8 + Copy,
    max_log: u8,
    repeat_table_max_sequences: usize,
    predefined_max_sequences: usize,
    selection_policy: TableSelectionPolicy,
) -> FseTableMode<'a> {
    match selection_policy {
        TableSelectionPolicy::CFast { default_norm_log } => {
            let counts = CodeCounts::from_codes(sequences.iter().map(code));
            return choose_c_fast_table(
                previous,
                default_table,
                sequences,
                code,
                &counts,
                max_log,
                repeat_table_max_sequences,
                predefined_max_sequences,
                default_norm_log,
            );
        }
        TableSelectionPolicy::CCost => {
            let counts = CodeCounts::from_codes(sequences.iter().map(code));
            return choose_c_cost_table(previous, default_table, sequences, code, &counts, max_log);
        }
        TableSelectionPolicy::Legacy => {}
    }

    let first_code = code(&sequences[0]);
    let all_same_code = sequences
        .iter()
        .skip(1)
        .all(|sequence| code(sequence) == first_code);

    if all_same_code && sequences.len() > 2 {
        return FseTableMode::Rle(first_code);
    }

    if sequences.len() <= predefined_max_sequences
        && sequences
            .iter()
            .all(|sequence| default_table.can_encode_symbol(code(sequence)))
    {
        return FseTableMode::Predefined(default_table);
    }

    if all_same_code {
        return FseTableMode::Rle(first_code);
    }

    if let Some(previous) = previous {
        if sequences.len() < repeat_table_max_sequences
            && sequences
                .iter()
                .all(|sequence| previous.can_encode_symbol(code(sequence)))
        {
            return FseTableMode::RepeatLast(previous);
        }
    }

    FseTableMode::Encoded(build_sequence_table(sequences, code, max_log))
}

#[allow(clippy::too_many_arguments)]
fn choose_c_fast_table<'a>(
    previous: Option<&'a FSETable>,
    default_table: &'a FSETable,
    sequences: &[Sequence],
    code: impl Fn(&Sequence) -> u8 + Copy,
    counts: &CodeCounts,
    max_log: u8,
    repeat_table_max_sequences: usize,
    predefined_max_sequences: usize,
    default_norm_log: u8,
) -> FseTableMode<'a> {
    if counts.most_frequent() == counts.total() {
        let first_code = counts.max_symbol() as u8;
        let default_allowed = default_table.can_encode_symbol(first_code);
        return if default_allowed && sequences.len() <= 2 {
            FseTableMode::Predefined(default_table)
        } else {
            FseTableMode::Rle(first_code)
        };
    }

    let default_allowed = counts.default_allowed(default_table);

    if default_allowed {
        if let Some(previous) = previous {
            if sequences.len() < repeat_table_max_sequences && counts.default_allowed(previous) {
                return FseTableMode::RepeatLast(previous);
            }
        }

        if sequences.len() < predefined_max_sequences
            || counts.most_frequent() < (sequences.len() >> (usize::from(default_norm_log) - 1))
        {
            return FseTableMode::Predefined(default_table);
        }
    }

    FseTableMode::Encoded(build_sequence_table_from_counts(
        counts,
        code(&sequences[sequences.len() - 1]),
        max_log,
    ))
}

fn choose_c_cost_table<'a>(
    previous: Option<&'a FSETable>,
    default_table: &'a FSETable,
    sequences: &[Sequence],
    code: impl Fn(&Sequence) -> u8 + Copy,
    counts: &CodeCounts,
    max_log: u8,
) -> FseTableMode<'a> {
    let default_allowed = counts.default_allowed(default_table);

    if counts.most_frequent() == counts.total() {
        let first_code = counts.max_symbol() as u8;
        return if default_allowed && sequences.len() <= 2 {
            FseTableMode::Predefined(default_table)
        } else {
            FseTableMode::Rle(first_code)
        };
    }

    let max_symbol = counts.max_symbol();
    let (encoded_probs, encoded_acc_log) =
        normalized_probabilities_from_counts(&counts.counts()[..=max_symbol], max_log, true);
    let compressed_cost =
        ncount_cost_from_probabilities(&encoded_probs, encoded_acc_log) + entropy_cost(counts);
    let basic_cost = default_allowed
        .then(|| cross_entropy_cost(default_table, counts))
        .flatten();
    let repeat_cost = previous.and_then(|previous| repeat_table_cost(previous, counts));

    if basic_cost.is_some_and(|basic_cost| {
        basic_cost <= repeat_cost.unwrap_or(usize::MAX) && basic_cost <= compressed_cost
    }) {
        return FseTableMode::Predefined(default_table);
    }

    if let Some(previous) = previous {
        if repeat_cost.is_some_and(|repeat_cost| repeat_cost <= compressed_cost) {
            return FseTableMode::RepeatLast(previous);
        }
    }

    let last_code = code(&sequences[sequences.len() - 1]);
    let encoded_table = if counts.counts()[usize::from(last_code)] > 1 {
        build_sequence_table_from_counts(counts, last_code, max_log)
    } else {
        build_table_from_probabilities(&encoded_probs, encoded_acc_log)
    };
    FseTableMode::Encoded(encoded_table)
}

fn build_sequence_table(
    sequences: &[Sequence],
    code: impl Fn(&Sequence) -> u8 + Copy,
    max_log: u8,
) -> FSETable {
    let mut counts = [0usize; 256];
    for sequence in sequences {
        counts[usize::from(code(sequence))] += 1;
    }

    let last_code = usize::from(code(&sequences[sequences.len() - 1]));
    build_sequence_table_from_raw_counts(&mut counts, last_code, max_log)
}

fn build_sequence_table_from_counts(counts: &CodeCounts, last_code: u8, max_log: u8) -> FSETable {
    let mut counts = *counts.counts();
    build_sequence_table_from_raw_counts(&mut counts, usize::from(last_code), max_log)
}

fn build_sequence_table_from_raw_counts(
    counts: &mut [usize; 256],
    last_code: usize,
    max_log: u8,
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
    build_table_from_probabilities(&probs, acc_log)
}

fn candidate_table_modes<'a>(
    previous: Option<&'a FSETable>,
    default_table: &'a FSETable,
    sequences: &[Sequence],
    code: impl Fn(&Sequence) -> u8 + Copy,
    config: TableModeCandidateConfig,
) -> Vec<FseTableMode<'a>> {
    let heuristic = choose_table_with_policy(
        previous,
        default_table,
        sequences,
        code,
        config.max_log,
        config.repeat_table_max_sequences,
        config.predefined_max_sequences,
        config.selection_policy,
    );

    let mut candidates = vec![heuristic];
    let first_code = code(&sequences[0]);
    let all_same_code = sequences
        .iter()
        .skip(1)
        .all(|sequence| code(sequence) == first_code);

    if sequences.len() <= config.predefined_max_sequences
        && sequences
            .iter()
            .all(|sequence| default_table.can_encode_symbol(code(sequence)))
    {
        candidates.push(FseTableMode::Predefined(default_table));
    }

    if let Some(previous) = previous {
        if sequences.len() < config.repeat_table_max_sequences
            && sequences
                .iter()
                .all(|sequence| previous.can_encode_symbol(code(sequence)))
        {
            candidates.push(FseTableMode::RepeatLast(previous));
        }
    }

    if all_same_code {
        if sequences.len() > 2 {
            candidates.push(FseTableMode::Rle(first_code));
        }
    } else {
        let exact_min_log = if config.exact_sequence_mode_search {
            EXACT_SEQUENCE_TABLE_MIN_LOG.min(config.max_log)
        } else {
            config.max_log
        };
        for candidate_max_log in exact_min_log..=config.max_log {
            candidates.push(FseTableMode::Encoded(build_sequence_table(
                sequences,
                code,
                candidate_max_log,
            )));
        }
    }

    candidates
}

pub(in crate::encoding::blocks::compressed) fn choose_sequence_table_modes<'a>(
    sequences: &[Sequence],
    config: SequenceModeSearchConfig<'a>,
) -> (FseTableMode<'a>, FseTableMode<'a>, FseTableMode<'a>) {
    let llml_policy =
        sequence_table_selection_policy(config.c_fast_heuristics, config.c_cost_model, 6);
    let offset_policy =
        sequence_table_selection_policy(config.c_fast_heuristics, config.c_cost_model, 5);

    if !config.exact_sequence_mode_search {
        return (
            choose_table_with_policy(
                config.ll_previous,
                config.ll_default,
                sequences,
                |seq| encode_literal_length(seq.ll).0,
                9,
                config.repeat_table_max_sequences,
                config.llml_predefined_max_sequences,
                llml_policy,
            ),
            choose_table_with_policy(
                config.ml_previous,
                config.ml_default,
                sequences,
                |seq| encode_match_len(seq.ml).0,
                9,
                config.repeat_table_max_sequences,
                config.llml_predefined_max_sequences,
                llml_policy,
            ),
            choose_table_with_policy(
                config.of_previous,
                config.of_default,
                sequences,
                |seq| encode_offset(seq.of).0,
                config.of_max_log,
                config.repeat_table_max_sequences,
                config.of_predefined_max_sequences,
                offset_policy,
            ),
        );
    }

    let ll_candidates = candidate_table_modes(
        config.ll_previous,
        config.ll_default,
        sequences,
        |seq| encode_literal_length(seq.ll).0,
        TableModeCandidateConfig {
            max_log: 9,
            repeat_table_max_sequences: config.repeat_table_max_sequences,
            predefined_max_sequences: config.llml_predefined_max_sequences,
            exact_sequence_mode_search: config.exact_sequence_mode_search,
            selection_policy: llml_policy,
        },
    );
    let ml_candidates = candidate_table_modes(
        config.ml_previous,
        config.ml_default,
        sequences,
        |seq| encode_match_len(seq.ml).0,
        TableModeCandidateConfig {
            max_log: 9,
            repeat_table_max_sequences: config.repeat_table_max_sequences,
            predefined_max_sequences: config.llml_predefined_max_sequences,
            exact_sequence_mode_search: config.exact_sequence_mode_search,
            selection_policy: llml_policy,
        },
    );
    let of_candidates = candidate_table_modes(
        config.of_previous,
        config.of_default,
        sequences,
        |seq| encode_offset(seq.of).0,
        TableModeCandidateConfig {
            max_log: config.of_max_log,
            repeat_table_max_sequences: config.repeat_table_max_sequences,
            predefined_max_sequences: config.of_predefined_max_sequences,
            exact_sequence_mode_search: config.exact_sequence_mode_search,
            selection_policy: offset_policy,
        },
    );

    let mut ll_candidates = ll_candidates;
    let mut ml_candidates = ml_candidates;
    let mut of_candidates = of_candidates;
    let mut best_ll = 0usize;
    let mut best_ml = 0usize;
    let mut best_of = 0usize;
    let mut best_size = super::exact_sequence_section_size(
        sequences,
        &ll_candidates[0],
        &ml_candidates[0],
        &of_candidates[0],
    );

    for (ll_idx, ll_mode) in ll_candidates.iter().enumerate() {
        for (ml_idx, ml_mode) in ml_candidates.iter().enumerate() {
            for (of_idx, of_mode) in of_candidates.iter().enumerate() {
                let size = super::exact_sequence_section_size(sequences, ll_mode, ml_mode, of_mode);
                if size < best_size {
                    best_ll = ll_idx;
                    best_ml = ml_idx;
                    best_of = of_idx;
                    best_size = size;
                }
            }
        }
    }

    (
        ll_candidates.swap_remove(best_ll),
        ml_candidates.swap_remove(best_ml),
        of_candidates.swap_remove(best_of),
    )
}

fn sequence_table_selection_policy(
    c_fast_heuristics: bool,
    c_cost_model: bool,
    default_norm_log: u8,
) -> TableSelectionPolicy {
    if c_fast_heuristics {
        TableSelectionPolicy::CFast { default_norm_log }
    } else if c_cost_model {
        TableSelectionPolicy::CCost
    } else {
        TableSelectionPolicy::Legacy
    }
}
