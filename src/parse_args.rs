use clap::Clap;
use glob;

use crate::output::OutputMode;

#[derive(Clap, Eq, PartialEq, Debug)]
#[clap(version = "0.1.0")]
pub struct Config {
    #[clap(short = 'n', long = "samples", default_value = "100")]
    pub n_samples: usize,

    #[clap(long = "batch-size", default_value = "100")]
    pub batch_size: usize,

    #[clap(long = "batch-sleep-ms", default_value = "100")]
    pub batch_sleep_ms: u64,

    #[clap(long = "mem", about = "Collect memory statistics")]
    pub stat_memory: bool,

    #[clap(long = "ttl", about = "Collect TTL statistics")]
    pub stat_ttl: bool,

    #[clap(short = 'o', long = "out", default_value = "table", possible_values = &["table"])]
    pub output_mode: OutputMode,

    #[clap(long = "url", default_value = "redis://127.0.0.1", validator = validate_url)]
    pub url: String,

    #[clap(about = "Glob-style patterns to group keys together")]
    pub patterns: Vec<glob::Pattern>,
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
    fn config_validate_url() {
        assert_eq!(
            Config::parse_from(&["test"]),
            Config {
                n_samples: 100,
                batch_size: 100,
                batch_sleep_ms: 100,
                stat_memory: false,
                stat_ttl: false,
                output_mode: OutputMode::StdoutTable,
                url: "redis://127.0.0.1".to_string(),
                patterns: vec![],
            }
        );
    }
}
