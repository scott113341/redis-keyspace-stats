use std::str::FromStr;

use Stats::*;

pub static STATS_OPTIONS: [&str; 3] = ["memory", "ttl", "type"];

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Stats {
    Memory,
    TTL,
    Type,
}

impl Stats {
    #[allow(dead_code)]
    pub fn all() -> Vec<Stats> {
        vec![Memory, TTL, Type]
    }
}

impl FromStr for Stats {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "memory" => Ok(Memory),
            "ttl" => Ok(TTL),
            "type" => Ok(Type),
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
