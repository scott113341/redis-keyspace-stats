use crate::data::math::pct_keyspace_sampled;
use crate::data::{Data, Keys};
use crate::metadata::Metadata;

pub fn total_estimate(metadata: &Metadata, data: &Data, keys: &Keys) -> u64 {
    let pct_of_keyspace_sampled = pct_keyspace_sampled(metadata, data);
    (keys.len() as f64 / pct_of_keyspace_sampled).round() as u64
}
