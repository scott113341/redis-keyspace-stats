use redis::{Connection, Value};
use std::fmt::Debug;

use crate::parse_args::Config;
use crate::stats::Stats;

#[derive(Eq, PartialEq, Debug)]
pub struct Sample {
    pub exists: SampleValue<bool>,
    pub memory: SampleValue<i64>,
    pub ttl: SampleValue<i64>,
    pub type_: SampleValue<String>,
}

#[allow(dead_code)]
impl Sample {
    pub fn new(data: &Vec<Value>, config: &Config) -> Sample {
        use crate::sample::SampleValue::*;

        let mut sample = Sample {
            exists: Unsampled,
            memory: Unsampled,
            ttl: Unsampled,
            type_: Unsampled,
        };

        // Whether this key exists is always at the 0th index
        let exists = data.get(0);
        sample.exists = match exists {
            Some(&Value::Int(0)) => Sampled(false),
            Some(&Value::Int(1)) => Sampled(true),
            _ => NotFound,
        };

        // The rest of the stats are optional. To account for this, we use an offset for indexing
        // into the "data" vector. For each stat we read, it is incremented. Note that the order of
        // stats checked here MUST match the order they were collected in the sample_key function.
        let mut data_idx = 1;

        if config.has_stat(&Stats::Memory) {
            let memory = data.get(data_idx);
            data_idx += 1;
            sample.memory = match memory {
                Some(Value::Int(mem)) => Sampled(*mem),
                _ => NotFound,
            }
        }

        if config.has_stat(&Stats::TTL) {
            let ttl = data.get(data_idx);
            data_idx += 1;
            sample.ttl = match ttl {
                Some(Value::Int(ttl)) => Sampled(*ttl),
                _ => NotFound,
            }
        }

        #[allow(unused_assignments)]
        if config.has_stat(&Stats::Type) {
            let type_ = data.get(data_idx);
            data_idx += 1;
            sample.type_ = match type_ {
                Some(Value::Status(t)) => Sampled(t.clone()),
                _ => NotFound,
            }
        }

        sample
    }

    pub fn exists(&self) -> bool {
        self.exists.value().clone()
    }

    pub fn memory(&self) -> i64 {
        self.memory.value().clone()
    }

    pub fn ttl(&self) -> i64 {
        self.ttl.value().clone()
    }

    pub fn type_(&self) -> String {
        self.type_.value().clone()
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum SampleValue<T> {
    Unsampled,
    NotFound,
    Sampled(T),
}

impl<T> SampleValue<T> {
    pub fn value(&self) -> &T {
        use SampleValue::*;

        match self {
            Unsampled => panic!(),
            NotFound => panic!(),
            Sampled(val) => val,
        }
    }
}

pub fn sample_key(key: &String, config: &Config, conn: &mut Connection) -> Result<Sample, String> {
    use crate::sample::SampleValue::*;

    // Instantiate an atomic pipeline that we'll use to gets stats about this key
    let mut pipe = redis::pipe();
    let mut pipe_ref = pipe.atomic();

    // Add commands to the pipeline for this key, depending on what stats we've requested
    {
        // Always check whether this key exists (in case it's since expired)
        // https://redis.io/commands/exists
        pipe_ref = pipe_ref.cmd("EXISTS").arg(key);

        // Get the memory usage of the key, sampling ALL values if this is a nested data type
        // https://redis.io/commands/memory-usage
        if config.has_stat(&Stats::Memory) {
            pipe_ref = pipe_ref
                .cmd("MEMORY")
                .arg("USAGE")
                .arg(key)
                .arg("SAMPLES")
                .arg("0");
        }

        // Get the TTL of the key in seconds
        // https://redis.io/commands/ttl
        if config.has_stat(&Stats::TTL) {
            pipe_ref = pipe_ref.cmd("TTL").arg(key);
        }

        // Get the data type of the key
        // https://redis.io/commands/type
        if config.has_stat(&Stats::Type) {
            pipe_ref = pipe_ref.cmd("TYPE").arg(key);
        }
    }

    // Run the pipeline and build the Sample
    let data: Vec<Value> = pipe_ref
        .query(conn)
        .or_else(|e| Err(format!("Redis pipeline failed: {}", e)))?;
    let sample = Sample::new(&data, &config);

    // If EXISTS failed or returned 0, return an error. This can happen when a key gets deleted from
    // Redis between the time we got it from RANDOMKEY and now.
    if sample.exists == NotFound || sample.exists == Sampled(false) {
        Err(format!("Key '{}' not found", key))
    } else {
        Ok(sample)
    }
}

#[cfg(test)]
mod tests {
    use redis::Commands;

    use super::*;
    use crate::tests::test_config_and_conn;

    #[test]
    fn sample_key_works_1() {
        let (config, mut conn) = test_config_and_conn();
        let _: () = conn.set_ex("sample_key_works_1", "test_value", 10).unwrap();
        let sample = sample_key(&"sample_key_works_1".to_string(), &config, &mut conn).unwrap();

        assert_eq!(sample.exists(), true);
        assert!(sample.memory() > 16);
        assert_eq!(sample.ttl(), 10);
        assert_eq!(sample.type_(), "string".to_string());
    }

    #[test]
    fn sample_key_works_2() {
        let (config, mut conn) = test_config_and_conn();
        let _: () = conn.sadd("sample_key_works_2", "a").unwrap();
        let _: () = conn.sadd("sample_key_works_2", "b").unwrap();
        let _: () = conn.sadd("sample_key_works_2", "c").unwrap();
        let sample = sample_key(&"sample_key_works_2".to_string(), &config, &mut conn).unwrap();

        assert_eq!(sample.exists(), true);
        assert!(sample.memory() > 128);
        assert_eq!(sample.ttl(), -1);
        assert_eq!(sample.type_(), "set".to_string());
    }
}
