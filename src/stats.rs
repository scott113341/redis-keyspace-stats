use Stats::*;

#[derive(clap::ValueEnum, Eq, PartialEq, Hash, Clone, Debug)]
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
