use clap::Parser;
use std::collections::HashSet;

use crate::output::OutputMode;
use crate::sampling::SampleMode;
use crate::stats::Stats;

#[derive(Parser, Eq, PartialEq, Debug)]
#[clap(version)]
pub struct Config {
    #[clap(long = "sample", default_value = "random", value_enum, value_parser)]
    pub sample_mode: SampleMode,

    #[clap(
        short = 'n',
        long = "samples",
        default_value = "100",
        help = "Ignored when --sample=all is specified"
    )]
    pub n_samples: usize,

    #[clap(long = "batch-size", default_value = "100")]
    pub batch_size: usize,

    #[clap(long = "batch-sleep-ms", default_value = "100")]
    pub batch_sleep_ms: u64,

    #[clap(
        long = "stats",
        use_value_delimiter = true,
        default_value = "memory,ttl",
        value_enum,
        value_parser
    )]
    pub stats: Vec<Stats>,

    #[clap(
        short = 'o',
        long = "out",
        default_value = "table",
        value_enum,
        value_parser
    )]
    pub output_mode: OutputMode,

    #[clap(long = "url", default_value = "redis://127.0.0.1", value_parser = parse_url)]
    pub url: String,

    #[clap(help = "Glob-style patterns to group keys together")]
    pub patterns: Vec<glob::Pattern>,
}

impl Config {
    // This is kind of a catch-all method for doing some additional munging of the Config struct
    // that doesn't nicely fit into what Clap gives us.
    pub fn normalize(&mut self) {
        // Deduplicate "stats"
        let mut unique_stats = HashSet::new();
        self.stats.retain(|s| unique_stats.insert(s.clone()));

        // Append a "*" pattern, then deduplicate
        self.patterns.push(glob::Pattern::new("*").unwrap());
        let mut unique_patterns = HashSet::new();
        self.patterns.retain(|s| unique_patterns.insert(s.clone()));
    }

    pub fn has_stat(&self, stat: &Stats) -> bool {
        self.stats.iter().any(|s| s == stat)
    }
}

// Connects to the given Redis instance and executes a PING command. Returns whatever error message
// if any part fails, or given URL verbatim if the PING succeeds.
fn parse_url(url: &str) -> Result<String, String> {
    let client = redis::Client::open(url).map_err(|e| e.to_string())?;
    let mut conn = client.get_connection().map_err(|e| e.to_string())?;
    redis::cmd("PING")
        .query(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(url.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn config_parse_url() {
        assert_eq!(
            Config::parse_from(&["test"]),
            Config {
                sample_mode: SampleMode::Random,
                n_samples: 100,
                batch_size: 100,
                batch_sleep_ms: 100,
                stats: vec![Stats::Memory, Stats::Ttl],
                output_mode: OutputMode::Table,
                url: "redis://127.0.0.1".to_string(),
                patterns: vec![],
            }
        );
    }

    #[test]
    fn verify_clap() {
        Config::command().debug_assert();
    }

    #[test]
    fn readme_help_text_is_up_to_date() {
        let help_text = Config::command().render_help().to_string();
        assert!(
            include_str!("../README.md").contains(&help_text),
            "README help text is outdated"
        );
    }
}
