//! Sequence and repeat-offset primitives ported from `ZSTD_storeSeqOnly()` and
//! `ZSTD_updateRep()`.

const REPCODE_COUNT: u32 = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum OffBase {
    Repeat(RepeatCode),
    Offset(u32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RepeatCode {
    First,
    Second,
    Third,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct StoredSequence {
    pub(crate) lit_len: u32,
    pub(crate) off_base: OffBase,
    pub(crate) match_len: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RepeatOffsets {
    offsets: [u32; REPCODE_COUNT as usize],
}

impl OffBase {
    pub(crate) fn from_c_value(value: u32) -> Option<Self> {
        match value {
            1 => Some(Self::Repeat(RepeatCode::First)),
            2 => Some(Self::Repeat(RepeatCode::Second)),
            3 => Some(Self::Repeat(RepeatCode::Third)),
            4.. => Some(Self::Offset(value - REPCODE_COUNT)),
            0 => None,
        }
    }

    pub(crate) fn from_offset(offset: u32) -> Option<Self> {
        (offset > 0).then_some(Self::Offset(offset))
    }

    pub(crate) fn to_c_value(self) -> u32 {
        match self {
            Self::Repeat(repeat) => repeat.to_c_value(),
            Self::Offset(offset) => offset + REPCODE_COUNT,
        }
    }
}

impl RepeatCode {
    fn to_c_value(self) -> u32 {
        match self {
            Self::First => 1,
            Self::Second => 2,
            Self::Third => 3,
        }
    }
}

impl StoredSequence {
    pub(crate) fn new(lit_len: u32, off_base: OffBase, match_len: u32) -> Self {
        Self {
            lit_len,
            off_base,
            match_len,
        }
    }
}

impl RepeatOffsets {
    pub(crate) const fn new() -> Self {
        Self { offsets: [1, 4, 8] }
    }

    pub(crate) const fn from_offsets(newest: u32, second: u32, third: u32) -> Self {
        Self {
            offsets: [newest, second, third],
        }
    }

    pub(crate) const fn as_offsets(self) -> [u32; REPCODE_COUNT as usize] {
        self.offsets
    }

    pub(crate) fn resolve(self, off_base: OffBase, lit_len: u32) -> u32 {
        match off_base {
            OffBase::Offset(offset) => offset,
            OffBase::Repeat(repeat) => {
                let rep_code = repeat.to_c_value() - 1 + u32::from(lit_len == 0);
                if rep_code == REPCODE_COUNT {
                    self.offsets[0] - 1
                } else {
                    self.offsets[rep_code as usize]
                }
            }
        }
    }

    pub(crate) fn update(&mut self, off_base: OffBase, lit_len: u32) {
        match off_base {
            OffBase::Offset(offset) => {
                self.offsets[2] = self.offsets[1];
                self.offsets[1] = self.offsets[0];
                self.offsets[0] = offset;
            }
            OffBase::Repeat(repeat) => {
                let rep_code = repeat.to_c_value() - 1 + u32::from(lit_len == 0);
                if rep_code == 0 {
                    return;
                }

                let current_offset = if rep_code == REPCODE_COUNT {
                    self.offsets[0] - 1
                } else {
                    self.offsets[rep_code as usize]
                };
                if rep_code >= 2 {
                    self.offsets[2] = self.offsets[1];
                }
                self.offsets[1] = self.offsets[0];
                self.offsets[0] = current_offset;
            }
        }
    }
}
