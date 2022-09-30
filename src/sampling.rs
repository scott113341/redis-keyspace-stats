use redis::Connection;

use crate::config::Config;
use crate::data::Data;

pub mod sample;
mod sample_all;
mod sample_random;

#[derive(clap::ValueEnum, Eq, PartialEq, Clone, Debug)]
pub enum SampleMode {
    All,
    Random,
}

pub fn collect_samples(config: &Config, conn: &mut Connection) -> Data {
    use SampleMode::*;
    match config.sample_mode {
        All => sample_all::sample_all(config, conn),
        Random => sample_random::sample_random(config, conn),
    }
}
