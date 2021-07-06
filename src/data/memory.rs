use crate::data::math::{pct_keyspace_sampled, percentile_of_sorted};
use crate::data::{Data, Keys};
use crate::metadata::Metadata;

pub fn total(data: &Data, keys: &Keys) -> u64 {
    memory_values(data, keys).iter().sum()
}

pub fn total_estimate(metadata: &Metadata, data: &Data, keys: &Keys) -> u64 {
    let pct_of_keyspace_sampled = pct_keyspace_sampled(metadata, data);
    let sampled_total = total(data, keys);
    (sampled_total as f64 / pct_of_keyspace_sampled).round() as u64
}

pub fn percentile(data: &Data, keys: &Keys, pct: f64) -> f64 {
    let mut values = memory_values(data, keys)
        .iter()
        .map(|&v| v as f64)
        .collect::<Vec<f64>>();

    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if values.is_empty() {
        0_f64
    } else {
        percentile_of_sorted(&values, pct)
    }
}

fn memory_values(data: &Data, keys: &Keys) -> Vec<u64> {
    keys.iter()
        .map(|k| data.get_sample(k).unwrap().memory())
        .collect()
}
