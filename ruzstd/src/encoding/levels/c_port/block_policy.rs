//! Block emission policy for C frame-level edge cases.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct BlockEncodingPolicy {
    allow_rle: bool,
}

impl BlockEncodingPolicy {
    pub(super) const fn normal() -> Self {
        Self { allow_rle: true }
    }

    pub(super) const fn frame_first_block() -> Self {
        Self { allow_rle: false }
    }

    pub(super) const fn allows_rle(self) -> bool {
        self.allow_rle
    }
}
