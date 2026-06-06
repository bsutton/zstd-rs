use super::sequence_store::{OffBase, RepeatCode, RepeatOffsets, StoredSequence};

#[test]
fn offbase_uses_c_numeric_representation() {
    assert_eq!(OffBase::from_c_value(0), None);
    assert_eq!(
        OffBase::from_c_value(1),
        Some(OffBase::Repeat(RepeatCode::First))
    );
    assert_eq!(
        OffBase::from_c_value(2),
        Some(OffBase::Repeat(RepeatCode::Second))
    );
    assert_eq!(
        OffBase::from_c_value(3),
        Some(OffBase::Repeat(RepeatCode::Third))
    );
    assert_eq!(OffBase::from_c_value(4), Some(OffBase::Offset(1)));
    assert_eq!(OffBase::from_offset(17).unwrap().to_c_value(), 20);
}

#[test]
fn stored_sequence_preserves_c_fields() {
    assert_eq!(
        StoredSequence::new(3, OffBase::Offset(9), 12),
        StoredSequence {
            lit_len: 3,
            off_base: OffBase::Offset(9),
            match_len: 12,
        }
    );
}

#[test]
fn offset_match_pushes_repeat_history() {
    let mut repeats = RepeatOffsets::new();

    repeats.update(OffBase::Offset(9), 4);

    assert_eq!(repeats.as_offsets(), [9, 1, 4]);
    assert_eq!(repeats.resolve(OffBase::Offset(9), 4), 9);
}

#[test]
fn repeat_codes_with_literals_match_c_update_rules() {
    let mut repeats = RepeatOffsets::from_offsets(11, 22, 33);

    repeats.update(OffBase::Repeat(RepeatCode::First), 5);
    assert_eq!(repeats.as_offsets(), [11, 22, 33]);
    assert_eq!(repeats.resolve(OffBase::Repeat(RepeatCode::First), 5), 11);

    repeats.update(OffBase::Repeat(RepeatCode::Second), 5);
    assert_eq!(repeats.as_offsets(), [22, 11, 33]);

    repeats.update(OffBase::Repeat(RepeatCode::Third), 5);
    assert_eq!(repeats.as_offsets(), [33, 22, 11]);
}

#[test]
fn repeat_codes_without_literals_shift_like_c() {
    let mut repeats = RepeatOffsets::from_offsets(11, 22, 33);

    repeats.update(OffBase::Repeat(RepeatCode::First), 0);
    assert_eq!(repeats.as_offsets(), [22, 11, 33]);

    repeats.update(OffBase::Repeat(RepeatCode::Second), 0);
    assert_eq!(repeats.as_offsets(), [33, 22, 11]);

    repeats.update(OffBase::Repeat(RepeatCode::Third), 0);
    assert_eq!(repeats.as_offsets(), [32, 33, 22]);
}
