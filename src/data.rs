pub mod get_data;

use glob;
use std::collections::HashMap;

use crate::parse_args::Config;
use crate::sample::Sample;

pub type Key = String;
pub type Samples = HashMap<Key, Sample>;
pub type KeyPatterns = Vec<glob::Pattern>;
pub type Keys = Vec<Key>;
pub type KeyBins = HashMap<glob::Pattern, Keys>;

#[derive(Debug)]
pub struct Data {
    samples: Samples,
    patterns: KeyPatterns,
    bins: KeyBins,
}

impl Data {
    pub fn new(config: &Config) -> Data {
        // Append a "*" pattern that matches any key for an "other" bin that matches anything
        let mut patterns = config.patterns.clone();
        patterns.push(glob::Pattern::new("*").unwrap());

        let bins = patterns.iter().map(|p| (p.clone(), vec![])).collect();

        Data {
            samples: HashMap::with_capacity(config.n_samples),
            patterns,
            bins,
        }
    }

    pub fn count(&self) -> usize {
        self.samples.len()
    }

    pub fn add_sample(&mut self, key: String, sample: Sample) {
        self.samples.insert(key.clone(), sample);

        if let Some(pattern) = self.patterns.iter().find(|p| p.matches(&key)) {
            let bin = self.bins.get_mut(pattern).unwrap();
            bin.push(key.clone());
        } else {
            panic!("Key: {} did not match any bins for some reason", key);
        }
    }

    pub fn patterns(&self) -> &KeyPatterns {
        &self.patterns
    }

    pub fn bins(&self) -> &KeyBins {
        &self.bins
    }

    pub fn has_sample(&self, key: &String) -> bool {
        self.samples.contains_key(key)
    }

    pub fn get_sample(&self, key: &String) -> Option<&Sample> {
        self.samples.get(key)
    }
}