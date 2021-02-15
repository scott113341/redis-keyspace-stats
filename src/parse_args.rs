use clap::crate_version;
use clap::Clap;
use glob;
use std::collections::HashSet;
use std::str::FromStr;

static STATS_OPTIONS: [&str; 2] = ["memory", "ttl"];
use crate::output::{OutputMode, OUTPUT_MODE_OPTIONS};

#[derive(Clap, Eq, PartialEq, Debug)]
#[clap(version = crate_version!())]
pub struct Config {
    #[clap(short = 'n', long = "samples", default_value = "100")]
    pub n_samples: usize,

    #[clap(long = "batch-size", default_value = "100")]
    pub batch_size: usize,

    #[clap(long = "batch-sleep-ms", default_value = "100")]
    pub batch_sleep_ms: u64,

    #[clap(long = "stats", use_delimiter = true, default_value = "memory,ttl", possible_values = &STATS_OPTIONS)]
    pub stats: Vec<Stats>,

    #[clap(short = 'o', long = "out", default_value = "table", possible_values = &OUTPUT_MODE_OPTIONS)]
    pub output_mode: OutputMode,

    #[clap(long = "url", default_value = "redis://127.0.0.1", validator = validate_url)]
    pub url: String,

    #[clap(about = "Glob-style patterns to group keys together")]
    pub patterns: Vec<glob::Pattern>,
}

impl Config {
    // This is kind of a catch-all method for doing some additional munging of the Config struct
    // that doesn't nicely fit into what Clap gives us.
    pub fn normalize(&mut self) {
        // Deduplicate "stats"
        let mut uniques = HashSet::new();
        self.stats.retain(|s| uniques.insert(s.clone()));

        // Append a "*" pattern, then deduplicate
        self.patterns.push(glob::Pattern::new("*").unwrap());
        let mut uniques = HashSet::new();
        self.stats.retain(|s| uniques.insert(s.clone()));
    }

    pub fn has_stat(&self, stat: Stats) -> bool {
        self.stats.iter().any(|s| *s == stat)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Stats {
    Memory,
    TTL,
}

impl FromStr for Stats {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Stats::*;
        match s {
            "memory" => Ok(Memory),
            "ttl" => Ok(TTL),
            _ => Err(format!("Unknown value: {}", s)),
        }
    }
}

// Connects to the given Redis instance and executes a PING command. Returns whatever error message
// if any part fails.
fn validate_url(url: &str) -> Result<(), String> {
    let client = redis::Client::open(url).or_else(|e| Err(e.to_string()))?;
    let mut conn = client.get_connection().or_else(|e| Err(e.to_string()))?;
    let _res = redis::cmd("PING")
        .query(&mut conn)
        .or_else(|e| Err(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_stats_options() {
        for opt in &STATS_OPTIONS {
            opt.parse::<Stats>()
                .unwrap_or_else(|_| panic!("Unsupported: {}", opt));
        }
    }

    #[test]
    fn config_validate_url() {
        assert_eq!(
            Config::parse_from(&["test"]),
            Config {
                n_samples: 100,
                batch_size: 100,
                batch_sleep_ms: 100,
                stats: vec![Stats::Memory, Stats::TTL],
                output_mode: OutputMode::StdoutTable,
                url: "redis://127.0.0.1".to_string(),
                patterns: vec![],
            }
        );
    }
}
