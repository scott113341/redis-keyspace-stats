use crate::data::math::percentile_of_sorted;
use crate::data::{Data, Keys};

pub fn pct_with_ttl(data: &Data, keys: &Keys) -> f64 {
    let values = ttl_values(data, keys);

    if values.is_empty() {
        0.0
    } else {
        let with_ttl_count = values.iter().filter(|&&v| v >= 0).count();
        (with_ttl_count as f64) / (values.len() as f64) * 100.0
    }
}

pub fn percentile(data: &Data, keys: &Keys, pct: f64) -> f64 {
    let mut values = ttl_values(data, keys)
        .iter()
        .filter(|&&v| v >= 0)
        .map(|&v| v as f64)
        .collect::<Vec<f64>>();

    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if values.is_empty() {
        0.0
    } else {
        percentile_of_sorted(&values, pct).round()
    }
}

fn ttl_values(data: &Data, keys: &Keys) -> Vec<i64> {
    keys.iter()
        .map(|k| data.get_sample(k).unwrap().ttl())
        .collect()
}
