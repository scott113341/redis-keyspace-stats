use std::str::FromStr;

use crate::config::Config;
use crate::data::Data;
use crate::metadata::Metadata;

mod stdout_table;

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
pub fn output(config: &Config, metadata: &Metadata, data: &Data) {
    use OutputMode::*;

    match config.output_mode {
        StdoutTable => stdout_table::stdout_table(config, metadata, data),
    }
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
