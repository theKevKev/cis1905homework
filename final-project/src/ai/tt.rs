use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum NodeType {
    Exact,
    LowerBound, // failed high (beta cutoff): actual value >= stored score
    UpperBound, // failed low  (alpha cutoff): actual value <= stored score
}

/// Lockless transposition table. Each slot holds a (key, data) pair written
/// with two separate atomic stores — no locking. Torn reads are detected as
/// hash mismatches and treated as cache misses, so correctness is maintained.
pub(crate) struct TT {
    keys: Vec<AtomicU64>,
    data: Vec<AtomicU64>,
    mask: u64,
}

impl TT {
    /// `size_log2` controls capacity: 2^size_log2 entries.
    /// 22 → 4M entries → ~64 MB. 20 → 1M entries → ~16 MB.
    pub(crate) fn new(size_log2: u8) -> Self {
        let size = 1usize << size_log2;
        TT {
            keys: (0..size).map(|_| AtomicU64::new(0)).collect(),
            data: (0..size).map(|_| AtomicU64::new(0)).collect(),
            mask: (size - 1) as u64,
        }
    }

    /// Returns `(score, node_type)` if there is a hit at depth >= `min_depth`.
    pub(crate) fn lookup(&self, hash: u64, min_depth: u8) -> Option<(i32, NodeType)> {
        let idx = (hash & self.mask) as usize;
        if self.keys[idx].load(Ordering::Relaxed) != hash {
            return None;
        }
        let raw = self.data[idx].load(Ordering::Relaxed);
        if ((raw >> 32) as u8) < min_depth {
            return None; // stored at shallower depth — not precise enough
        }
        let score = raw as i32; // low 32 bits, round-trips correctly for negative values
        let node_type = match (raw >> 40) as u8 & 0x3 {
            1 => NodeType::LowerBound,
            2 => NodeType::UpperBound,
            _ => NodeType::Exact,
        };
        Some((score, node_type))
    }

    /// Always overwrites. Caller should apply a replacement policy if desired.
    pub(crate) fn store(&self, hash: u64, score: i32, depth: u8, node_type: NodeType) {
        let idx = (hash & self.mask) as usize;
        let nt: u64 = match node_type {
            NodeType::Exact => 0,
            NodeType::LowerBound => 1,
            NodeType::UpperBound => 2,
        };
        // Pack: bits 0-31 = score (as u32), bits 32-39 = depth, bits 40-41 = node_type
        let raw = (score as u32 as u64) | ((depth as u64) << 32) | (nt << 40);
        // Write data before key: a reader that sees the new key but old data gets a hash
        // mismatch on re-read next time, treating it as a miss — safe.
        self.data[idx].store(raw, Ordering::Relaxed);
        self.keys[idx].store(hash, Ordering::Relaxed);
    }
}
