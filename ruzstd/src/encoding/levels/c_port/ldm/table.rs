use alloc::{vec, vec::Vec};

use super::super::cctx_params::LdmParameters;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct LdmEntry {
    pub(crate) offset: u32,
    pub(crate) checksum: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LdmHashTable {
    hash_table: Vec<LdmEntry>,
    bucket_offsets: Vec<u8>,
    bucket_size_log: u32,
}

impl LdmHashTable {
    pub(crate) fn new(params: LdmParameters) -> Self {
        debug_assert!(params.hash_log >= params.bucket_size_log);
        let table_entries = 1_usize << params.hash_log;
        let bucket_count = 1_usize << (params.hash_log - params.bucket_size_log);

        Self {
            hash_table: vec![LdmEntry::default(); table_entries],
            bucket_offsets: vec![0; bucket_count],
            bucket_size_log: params.bucket_size_log,
        }
    }

    pub(crate) fn insert_entry(&mut self, hash: usize, entry: LdmEntry) {
        debug_assert!(hash < self.bucket_offsets.len());
        let bucket_size = 1_usize << self.bucket_size_log;
        let offset = self.bucket_offsets[hash] as usize;
        let table_index = (hash << self.bucket_size_log) + offset;

        self.hash_table[table_index] = entry;
        self.bucket_offsets[hash] = ((offset + 1) & (bucket_size - 1)) as u8;
    }

    pub(crate) fn bucket(&self, hash: usize) -> &[LdmEntry] {
        debug_assert!(hash < self.bucket_offsets.len());
        let bucket_size = 1_usize << self.bucket_size_log;
        let bucket_start = hash << self.bucket_size_log;
        &self.hash_table[bucket_start..bucket_start + bucket_size]
    }

    pub(crate) fn bucket_offset(&self, hash: usize) -> u8 {
        self.bucket_offsets[hash]
    }

    pub(crate) fn table_len(&self) -> usize {
        self.hash_table.len()
    }

    pub(crate) fn bucket_count(&self) -> usize {
        self.bucket_offsets.len()
    }
}
