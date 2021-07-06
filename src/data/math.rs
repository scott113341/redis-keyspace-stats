use crate::data::Data;
use crate::metadata::Metadata;

// Adapted from https://github.com/rust-lang/rust/blob/0f6f2d68/library/test/src/stats.rs#L259-L281
pub fn percentile_of_sorted(sorted_samples: &Vec<f64>, pct: f64) -> f64 {
    assert!(!sorted_samples.is_empty());
    if sorted_samples.len() == 1 {
        return sorted_samples[0];
    }
    let zero: f64 = 0.0;
    assert!(zero <= pct);
    let hundred = 100_f64;
    assert!(pct <= hundred);
    if pct == hundred {
        return sorted_samples[sorted_samples.len() - 1];
    }
    let length = (sorted_samples.len() - 1) as f64;
    let rank = (pct / hundred) * length;
    let lrank = rank.floor();
    let d = rank - lrank;
    let n = lrank as usize;
    let lo = sorted_samples[n];
    let hi = sorted_samples[n + 1];
    lo + (hi - lo) * d
}

pub fn pct_keyspace_sampled(metadata: &Metadata, data: &Data) -> f64 {
    let pct_of_keyspace_sampled = data.sample_count() as f64 / metadata.total_keys as f64;
    if pct_of_keyspace_sampled > 100_f64 {
        100_f64
    } else {
        pct_of_keyspace_sampled
    }
}
