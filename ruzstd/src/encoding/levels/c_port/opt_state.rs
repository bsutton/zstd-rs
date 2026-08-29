//! Optimal-parser state shared by the no-dictionary C optimal strategies.

use alloc::{boxed::Box, vec::Vec};

use super::{
    greedy::GreedyMatchState,
    opt_match::OptMatchTable,
    opt_price::{OptPriceState, ZSTD_MAX_PRICE},
    params::CompressionParameters,
    sequence_store::{RepeatOffsets, StoredSequence},
};
use crate::encoding::blocks::{EstimateScratch, PreparedBlock};

pub(super) const HASH_READ_SIZE: usize = 8;
pub(super) const ZSTD_OPT_NUM: usize = 1 << 12;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Optimal {
    pub(super) price: i32,
    pub(super) off: u32,
    pub(super) mlen: u32,
    pub(super) litlen: u32,
    pub(super) rep: [u32; 3],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ForwardResult {
    pub(super) last_pos: usize,
    pub(super) last_stretch: Option<Optimal>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum OptParserStrategy {
    BtOpt,
    BtUltra,
}

#[derive(Clone, Debug)]
pub(crate) struct OptBlockState {
    pub(super) match_state: GreedyMatchState,
    pub(super) attached_match_state: Option<GreedyMatchState>,
    pub(super) price_state: OptPriceState,
    pub(super) literal_price_cache: LiteralPriceCache,
    pub(super) matches: OptMatchTable,
    pub(super) opt: Box<[Optimal; ZSTD_OPT_NUM + 4]>,
    pub(super) path: Vec<Optimal>,
    pub(super) post_split_estimate_scratch: EstimateScratch,
    sequences: Vec<StoredSequence>,
    prepared: PreparedBlock,
    block_bytes: Vec<u8>,
}

impl Default for Optimal {
    fn default() -> Self {
        Self {
            price: ZSTD_MAX_PRICE,
            off: 0,
            mlen: 0,
            litlen: 0,
            rep: RepeatOffsets::new().as_offsets(),
        }
    }
}

impl OptBlockState {
    pub(crate) fn new() -> Self {
        Self {
            match_state: GreedyMatchState::new(),
            attached_match_state: None,
            price_state: OptPriceState::new(),
            literal_price_cache: LiteralPriceCache::new(),
            matches: OptMatchTable::new(),
            opt: Box::new([Optimal::default(); ZSTD_OPT_NUM + 4]),
            path: Vec::with_capacity(16),
            post_split_estimate_scratch: EstimateScratch::new(),
            sequences: Vec::new(),
            prepared: empty_prepared_block(),
            block_bytes: Vec::new(),
        }
    }

    pub(crate) fn reset_for_frame(&mut self, params: CompressionParameters) {
        self.match_state.reset_for_frame(params);
        self.attached_match_state = None;
        self.price_state.reset_for_frame();
    }

    pub(crate) fn take_sequences(&mut self, min_capacity: usize) -> Vec<StoredSequence> {
        let mut sequences = core::mem::take(&mut self.sequences);
        sequences.clear();
        if sequences.capacity() < min_capacity {
            sequences.reserve(min_capacity - sequences.capacity());
        }
        sequences
    }

    pub(crate) fn recycle_sequences(&mut self, mut sequences: Vec<StoredSequence>) {
        sequences.clear();
        if self.sequences.capacity() < sequences.capacity() {
            self.sequences = sequences;
        }
    }

    pub(crate) fn take_prepared_block(&mut self) -> PreparedBlock {
        core::mem::replace(&mut self.prepared, empty_prepared_block())
    }

    pub(crate) fn recycle_prepared_block(&mut self, mut prepared: PreparedBlock) {
        prepared.literals.clear();
        prepared.sequences.clear();
        if self.prepared.literals.capacity() < prepared.literals.capacity()
            || self.prepared.sequences.capacity() < prepared.sequences.capacity()
        {
            self.prepared = prepared;
        }
    }

    pub(crate) fn take_block_bytes(&mut self, min_capacity: usize) -> Vec<u8> {
        let mut bytes = core::mem::take(&mut self.block_bytes);
        bytes.clear();
        if bytes.capacity() < min_capacity {
            bytes.reserve(min_capacity - bytes.capacity());
        }
        bytes
    }

    pub(crate) fn recycle_block_bytes(&mut self, mut bytes: Vec<u8>) {
        bytes.clear();
        if self.block_bytes.capacity() < bytes.capacity() {
            self.block_bytes = bytes;
        }
    }
}

fn empty_prepared_block() -> PreparedBlock {
    PreparedBlock {
        literals: Vec::new(),
        sequences: Vec::new(),
    }
}

#[derive(Clone, Debug)]
pub(super) struct LiteralPriceCache {
    prices: [u32; 256],
    generations: [u16; 256],
    generation: u16,
}

impl LiteralPriceCache {
    fn new() -> Self {
        Self {
            prices: [0; 256],
            generations: [0; 256],
            generation: 1,
        }
    }

    pub(super) fn begin_pass(&mut self) {
        self.generation = self.generation.wrapping_add(1);
        if self.generation == 0 {
            self.generations = [0; 256];
            self.generation = 1;
        }
    }

    pub(super) fn lookup(&self, literal: u8) -> Option<u32> {
        let idx = usize::from(literal);
        (self.generations[idx] == self.generation).then_some(self.prices[idx])
    }

    pub(super) fn insert(&mut self, literal: u8, price: u32) {
        let idx = usize::from(literal);
        self.prices[idx] = price;
        self.generations[idx] = self.generation;
    }
}
