use std::str::FromStr;

use Stats::*;

pub static STATS_OPTIONS: [&str; 2] = ["memory", "ttl"];

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Stats {
    Memory,
    TTL,
}

impl Stats {
    #[allow(dead_code)]
    pub fn all() -> Vec<Stats> {
        vec![Memory, TTL]
    }
}

impl FromStr for Stats {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "memory" => Ok(Memory),
            "ttl" => Ok(TTL),
            _ => Err(format!("Unknown value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stats_options() {
        for opt in &STATS_OPTIONS {
            opt.parse::<Stats>()
                .unwrap_or_else(|_| panic!("Unsupported: {}", opt));
        }
    }
}
