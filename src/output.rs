use crate::config::Config;
use crate::data::Data;
use crate::metadata::Metadata;

mod table;

#[derive(clap::ValueEnum, Eq, PartialEq, Clone, Debug)]
pub enum OutputMode {
    Table,
}

// Eventually, there will be more OutputMode options, and this will make more sense
pub fn output(config: &Config, metadata: &Metadata, data: &Data) {
    use OutputMode::*;

    match config.output_mode {
        Table => table::table(config, metadata, data),
    }
}
