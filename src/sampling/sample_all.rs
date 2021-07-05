use redis::Connection;
use std::thread::sleep;
use std::time::Duration;

use crate::config::Config;
use crate::data::*;
use crate::sampling::sample::sample_key;

pub fn sample_all(config: &Config, mut conn: &mut Connection) -> Data {
    let mut data = Data::new(&config);

    let scan_keys: Vec<String> = redis::cmd("SCAN")
        .cursor_arg(0)
        .arg("COUNT")
        .arg(config.batch_size)
        .clone()
        .iter(conn)
        .unwrap()
        .collect();

    // This counter is used to sleep after each batch of n_samples
    let mut batch_count = 0;

    for key in scan_keys {
        if !data.has_sample(&key) {
            let sample = sample_key(&key, config, &mut conn);
            if let Ok(sample) = sample {
                data.add_sample(key, sample);
            }

            batch_count += 1;
            if batch_count == config.n_samples {
                sleep(Duration::from_millis(config.batch_sleep_ms.into()));
                batch_count = 0;
            }
        }
    }

    data
}
