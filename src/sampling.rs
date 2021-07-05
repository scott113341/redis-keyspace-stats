use redis::Connection;
use std::str::FromStr;

use crate::config::Config;
use crate::data::Data;

pub mod sample;
mod sample_all;
mod sample_random;

pub static SAMPLE_MODE_OPTIONS: [&str; 2] = ["all", "random"];

#[derive(Eq, PartialEq, Debug)]
pub enum SampleMode {
    All,
    Random,
}

impl FromStr for SampleMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SampleMode::*;
        match s {
            "all" => Ok(All),
            "random" => Ok(Random),
            _ => Err(format!("Unknown value: {}", s)),
        }
    }
}

pub fn collect_samples(config: &Config, conn: &mut Connection) -> Data {
    use SampleMode::*;

    match config.sample_mode {
        All => sample_all::sample_all(config, conn),
        Random => sample_random::sample_random(config, conn),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_mode_options() {
        for opt in &SAMPLE_MODE_OPTIONS {
            opt.parse::<SampleMode>()
                .unwrap_or_else(|_| panic!("Unsupported: {}", opt));
        }
    }
}
