mod stdout_table;

use std::str::FromStr;

use crate::data::{Data, Keys};
use crate::parse_args::Config;

pub static OUTPUT_MODE_OPTIONS: [&str; 1] = ["table"];

#[derive(Eq, PartialEq, Debug)]
pub enum OutputMode {
    StdoutTable,
}

impl FromStr for OutputMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutputMode::*;
        match s {
            "table" => Ok(StdoutTable),
            _ => Err(format!("Unknown value: {}", s)),
        }
    }
}

// Eventually, there will be more OutputMode options, and this will make more sense
pub fn output(config: &Config, data: &Data) {
    use OutputMode::*;

    match config.output_mode {
        StdoutTable => stdout_table::stdout_table(config, data),
    }
}

mod memory {
    use super::*;

    pub fn total(data: &Data, keys: &Keys) -> i64 {
        memory_values(data, keys).iter().sum()
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

    fn memory_values(data: &Data, keys: &Keys) -> Vec<i64> {
        keys.iter()
            .map(|k| data.get_sample(k).unwrap().memory())
            .collect()
    }
}

mod ttl {
    use super::*;

    pub fn pct_with_ttl(data: &Data, keys: &Keys) -> f64 {
        let values = ttl_values(data, keys);

        if values.len() == 0 {
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
}

fn example_keys(keys: &Keys) -> Keys {
    keys.clone().into_iter().take(5).collect()
}

// Adapted from https://github.com/rust-lang/rust/blob/0f6f2d68/library/test/src/stats.rs#L259-L281
fn percentile_of_sorted(sorted_samples: &Vec<f64>, pct: f64) -> f64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_mode_options() {
        for opt in &OUTPUT_MODE_OPTIONS {
            opt.parse::<OutputMode>()
                .unwrap_or_else(|_| panic!("Unsupported: {}", opt));
        }
    }
}
