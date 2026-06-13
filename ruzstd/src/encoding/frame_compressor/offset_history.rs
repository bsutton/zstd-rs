#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct OffsetHistory {
    pub(crate) newest: u32,
    pub(crate) second: u32,
    pub(crate) third: u32,
}

impl OffsetHistory {
    pub(crate) const fn new() -> Self {
        Self {
            newest: 1,
            second: 4,
            third: 8,
        }
    }

    pub(crate) const fn from_offsets(newest: u32, second: u32, third: u32) -> Self {
        Self {
            newest,
            second,
            third,
        }
    }

    pub(crate) fn as_offsets(self) -> (u32, u32, u32) {
        (self.newest, self.second, self.third)
    }

    pub(crate) fn encode_offset_value(&mut self, offset: u32, lit_len: u32) -> u32 {
        let offset_value = if lit_len > 0 {
            if offset == self.newest {
                1
            } else if offset == self.second {
                2
            } else if offset == self.third {
                3
            } else {
                offset + 3
            }
        } else if offset == self.second {
            1
        } else if offset == self.third {
            2
        } else if self.newest.checked_sub(1) == Some(offset) {
            3
        } else {
            offset + 3
        };

        self.update_from_offset_value(offset_value, lit_len, offset);
        offset_value
    }

    #[inline(always)]
    pub(crate) fn update_after_match(&mut self, offset: u32, has_literals: bool) {
        if has_literals {
            if offset == self.newest {
                return;
            }
            if offset == self.second {
                self.second = self.newest;
                self.newest = offset;
                return;
            }
        } else if offset == self.second {
            self.second = self.newest;
            self.newest = offset;
            return;
        }

        self.third = self.second;
        self.second = self.newest;
        self.newest = offset;
    }

    fn update_from_offset_value(&mut self, offset_value: u32, lit_len: u32, actual_offset: u32) {
        if lit_len > 0 {
            match offset_value {
                1 => {}
                2 => {
                    self.second = self.newest;
                    self.newest = actual_offset;
                }
                _ => {
                    self.third = self.second;
                    self.second = self.newest;
                    self.newest = actual_offset;
                }
            }
        } else {
            match offset_value {
                1 => {
                    self.second = self.newest;
                    self.newest = actual_offset;
                }
                _ => {
                    self.third = self.second;
                    self.second = self.newest;
                    self.newest = actual_offset;
                }
            }
        }
    }
}
