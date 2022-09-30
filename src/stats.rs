use Stats::*;

#[derive(clap::ValueEnum, Eq, PartialEq, Hash, Clone, Debug)]
pub enum Stats {
    Memory,
    Ttl,
    Type,
}

impl Stats {
    #[allow(dead_code)]
    pub fn all() -> Vec<Stats> {
        vec![Memory, Ttl, Type]
    }
}
