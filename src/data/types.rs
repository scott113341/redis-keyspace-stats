use std::cmp::Reverse;
use std::collections::HashMap;

use crate::data::{Data, Keys};

pub fn type_pcts(data: &Data, keys: &Keys) -> Vec<(String, f64)> {
    let type_counts = type_counts(data, keys);
    let total_count = keys.len();

    let mut type_pcts = Vec::new();

    for (type_, count) in type_counts.into_iter() {
        let pct = (count as f64) / (total_count as f64) * 100.0;
        type_pcts.push((type_, pct));
    }

    type_pcts
}

fn type_counts(data: &Data, keys: &Keys) -> Vec<(String, usize)> {
    let mut counts = HashMap::new();

    for t in keys.iter().map(|k| data.get_sample(k).unwrap().type_()) {
        let count = counts.entry(t).or_insert(0);
        *count += 1;
    }

    let mut sorted_counts: Vec<_> = counts.into_iter().collect();
    sorted_counts.sort_by_key(|(_type, count)| Reverse(*count));
    sorted_counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_pcts_works() {
        use crate::sample::Sample;
        use crate::sample::SampleValue::*;

        let mut samples = HashMap::new();
        let mut keys = Vec::new();

        for (idx, t) in ["set", "string", "string", "string"].iter().enumerate() {
            let key = format!("key_{}", idx);
            keys.push(key.clone());
            samples.insert(
                key,
                Sample {
                    exists: Unsampled,
                    memory: Unsampled,
                    ttl: Unsampled,
                    type_: Sampled(t.to_string()),
                },
            );
        }

        let data = Data {
            samples,
            bins: Default::default(),
            patterns: vec![],
        };

        assert_eq!(
            type_pcts(&data, &keys),
            vec![("string".to_string(), 75.0), ("set".to_string(), 25.0),]
        );
    }
}
