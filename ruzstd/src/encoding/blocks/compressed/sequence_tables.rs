use alloc::{vec, vec::Vec};

use crate::{
    bit_io::BitWriter,
    fse::fse_encoder::{build_table_from_data, FSETable},
};

use super::{
    encode_literal_length, encode_match_len, encode_offset, encode_sequences,
    EXACT_SEQUENCE_TABLE_MIN_LOG,
};

#[derive(Clone)]
#[allow(clippy::large_enum_variant)]
pub(super) enum FseTableMode<'a> {
    Predefined(&'a FSETable),
    Rle(u8),
    Encoded(FSETable),
    RepeatLast(&'a FSETable),
}

impl FseTableMode<'_> {
    pub(super) fn table(&self) -> Option<&FSETable> {
        match self {
            Self::Predefined(t) => Some(t),
            Self::RepeatLast(t) => Some(t),
            Self::Encoded(t) => Some(t),
            Self::Rle(_) => None,
        }
    }
}

pub(super) struct SequenceModeSearchConfig<'a> {
    pub(super) ll_previous: Option<&'a FSETable>,
    pub(super) ll_default: &'a FSETable,
    pub(super) ml_previous: Option<&'a FSETable>,
    pub(super) ml_default: &'a FSETable,
    pub(super) of_previous: Option<&'a FSETable>,
    pub(super) of_default: &'a FSETable,
    pub(super) repeat_table_max_sequences: usize,
    pub(super) llml_predefined_max_sequences: usize,
    pub(super) of_predefined_max_sequences: usize,
    pub(super) of_max_log: u8,
    pub(super) exact_sequence_mode_search: bool,
    pub(super) c_fast_heuristics: bool,
}

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
}

#[cfg(test)]
pub(super) fn choose_table<'a>(
    previous: Option<&'a FSETable>,
    default_table: &'a FSETable,
    sequences: &[crate::blocks::sequence_section::Sequence],
    code: impl Fn(&crate::blocks::sequence_section::Sequence) -> u8 + Copy,
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
    sequences: &[crate::blocks::sequence_section::Sequence],
    code: impl Fn(&crate::blocks::sequence_section::Sequence) -> u8 + Copy,
    max_log: u8,
    repeat_table_max_sequences: usize,
    predefined_max_sequences: usize,
    selection_policy: TableSelectionPolicy,
) -> FseTableMode<'a> {
    let first_code = code(&sequences[0]);
    let all_same_code = sequences
        .iter()
        .skip(1)
        .all(|sequence| code(sequence) == first_code);

    if let TableSelectionPolicy::CFast { default_norm_log } = selection_policy {
        return choose_c_fast_table(
            previous,
            default_table,
            sequences,
            code,
            max_log,
            repeat_table_max_sequences,
            predefined_max_sequences,
            default_norm_log,
            first_code,
            all_same_code,
        );
    }

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

    FseTableMode::Encoded(build_table_from_data(
        sequences.iter().map(code),
        max_log,
        true,
    ))
}

#[allow(clippy::too_many_arguments)]
fn choose_c_fast_table<'a>(
    previous: Option<&'a FSETable>,
    default_table: &'a FSETable,
    sequences: &[crate::blocks::sequence_section::Sequence],
    code: impl Fn(&crate::blocks::sequence_section::Sequence) -> u8 + Copy,
    max_log: u8,
    repeat_table_max_sequences: usize,
    predefined_max_sequences: usize,
    default_norm_log: u8,
    first_code: u8,
    all_same_code: bool,
) -> FseTableMode<'a> {
    let default_allowed = sequences
        .iter()
        .all(|sequence| default_table.can_encode_symbol(code(sequence)));

    if all_same_code {
        return if default_allowed && sequences.len() <= 2 {
            FseTableMode::Predefined(default_table)
        } else {
            FseTableMode::Rle(first_code)
        };
    }

    if default_allowed {
        if let Some(previous) = previous {
            if sequences.len() < repeat_table_max_sequences
                && sequences
                    .iter()
                    .all(|sequence| previous.can_encode_symbol(code(sequence)))
            {
                return FseTableMode::RepeatLast(previous);
            }
        }

        if sequences.len() < predefined_max_sequences
            || most_frequent_code_count(sequences, code)
                < (sequences.len() >> (usize::from(default_norm_log) - 1))
        {
            return FseTableMode::Predefined(default_table);
        }
    }

    FseTableMode::Encoded(build_table_from_data(
        sequences.iter().map(code),
        max_log,
        true,
    ))
}

fn most_frequent_code_count(
    sequences: &[crate::blocks::sequence_section::Sequence],
    code: impl Fn(&crate::blocks::sequence_section::Sequence) -> u8 + Copy,
) -> usize {
    let mut counts = [0usize; 256];
    for sequence in sequences {
        counts[usize::from(code(sequence))] += 1;
    }
    counts.iter().copied().max().unwrap_or(0)
}

fn candidate_table_modes<'a>(
    previous: Option<&'a FSETable>,
    default_table: &'a FSETable,
    sequences: &[crate::blocks::sequence_section::Sequence],
    code: impl Fn(&crate::blocks::sequence_section::Sequence) -> u8 + Copy,
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
            candidates.push(FseTableMode::Encoded(build_table_from_data(
                sequences.iter().map(code),
                candidate_max_log,
                true,
            )));
        }
    }

    candidates
}

pub(super) fn choose_sequence_table_modes<'a>(
    sequences: &[crate::blocks::sequence_section::Sequence],
    config: SequenceModeSearchConfig<'a>,
) -> (FseTableMode<'a>, FseTableMode<'a>, FseTableMode<'a>) {
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
            selection_policy: if config.c_fast_heuristics {
                TableSelectionPolicy::CFast {
                    default_norm_log: 6,
                }
            } else {
                TableSelectionPolicy::Legacy
            },
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
            selection_policy: if config.c_fast_heuristics {
                TableSelectionPolicy::CFast {
                    default_norm_log: 6,
                }
            } else {
                TableSelectionPolicy::Legacy
            },
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
            selection_policy: if config.c_fast_heuristics {
                TableSelectionPolicy::CFast {
                    default_norm_log: 5,
                }
            } else {
                TableSelectionPolicy::Legacy
            },
        },
    );

    if !config.exact_sequence_mode_search {
        return (
            ll_candidates.into_iter().next().unwrap(),
            ml_candidates.into_iter().next().unwrap(),
            of_candidates.into_iter().next().unwrap(),
        );
    }

    let mut ll_candidates = ll_candidates;
    let mut ml_candidates = ml_candidates;
    let mut of_candidates = of_candidates;
    let mut best_ll = 0usize;
    let mut best_ml = 0usize;
    let mut best_of = 0usize;
    let mut best_size = exact_sequence_section_size(
        sequences,
        &ll_candidates[0],
        &ml_candidates[0],
        &of_candidates[0],
    );

    for (ll_idx, ll_mode) in ll_candidates.iter().enumerate() {
        for (ml_idx, ml_mode) in ml_candidates.iter().enumerate() {
            for (of_idx, of_mode) in of_candidates.iter().enumerate() {
                let size = exact_sequence_section_size(sequences, ll_mode, ml_mode, of_mode);
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

pub(super) fn exact_sequence_section_size(
    sequences: &[crate::blocks::sequence_section::Sequence],
    ll_mode: &FseTableMode<'_>,
    ml_mode: &FseTableMode<'_>,
    of_mode: &FseTableMode<'_>,
) -> usize {
    let mut encoded = Vec::new();
    let mut writer = BitWriter::from(&mut encoded);
    writer.write_bits(encode_fse_table_modes(ll_mode, ml_mode, of_mode), 8);
    encode_table(ll_mode, &mut writer);
    encode_table(of_mode, &mut writer);
    encode_table(ml_mode, &mut writer);
    encode_sequences(sequences, &mut writer, ll_mode, ml_mode, of_mode);
    writer.flush();
    encoded.len()
}

pub(super) fn encode_table(mode: &FseTableMode<'_>, writer: &mut BitWriter<&mut Vec<u8>>) {
    match mode {
        FseTableMode::Predefined(_) => {}
        FseTableMode::Rle(symbol) => writer.write_bits(*symbol, 8),
        FseTableMode::RepeatLast(_) => {}
        FseTableMode::Encoded(table) => table.write_table(writer),
    }
}

pub(super) fn encode_fse_table_modes(
    ll_mode: &FseTableMode<'_>,
    ml_mode: &FseTableMode<'_>,
    of_mode: &FseTableMode<'_>,
) -> u8 {
    fn mode_to_bits(mode: &FseTableMode<'_>) -> u8 {
        match mode {
            FseTableMode::Predefined(_) => 0,
            FseTableMode::Rle(_) => 1,
            FseTableMode::Encoded(_) => 2,
            FseTableMode::RepeatLast(_) => 3,
        }
    }
    mode_to_bits(ll_mode) << 6 | mode_to_bits(of_mode) << 4 | mode_to_bits(ml_mode) << 2
}
