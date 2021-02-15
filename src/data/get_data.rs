use redis::{Connection, RedisResult};
use std::borrow::BorrowMut;
use std::thread::sleep;
use std::time::Duration;

use crate::data::*;
use crate::parse_args::Config;
use crate::sample::sample_key;

// This could be more efficient by pipelining more commands. Right now, the order of operations
// looks like this:
//
//     keys = PIPELINE(["RANDOMKEY", "RANDOMKEY", ..., "RANDOMKEY"])
//     for key in keys {
//         stats = PIPELINE(["EXISTS $key", "MEMORY USAGE $key", "TTL $key", ...])
//     }
//
// That's fine, but we could get more aggressive by doing something like pipelining the stats for
// multiple keys at the same, time, or even using Lua to get random keys AND their stats in one
// operation.
//
// For now, we'll keep this kind of slow implementation that uses N+1 pipelines for N keys. At least
// we won't risk blocking Redis with a massive pipelined command.
//
// Unrelated note: if we don't find found any new keys for 10 batches in a row, this function will
// exit before n_samples has been collected. This guards against sampling indefinitely if Redis has
// fewer than n_samples keys total.
pub fn get_data(config: &Config, mut conn: &mut Connection) -> Data {
    let mut data = Data::new(&config);
    let mut no_new_keys_streak = 0;

    loop {
        // Get a batch of random keys
        let batch_size = this_batch_size(config, &data);
        let keys = get_random_keys(batch_size, &mut conn).unwrap();

        // Initialized as true, but set to false if any new keys are sampled this batch
        let mut no_new_keys = true;

        // Sample each key, and add it to our Data struct if successful. Skip keys that have already
        // been sampled.
        for key in keys {
            if !data.has_sample(&key) {
                let sample = sample_key(&key, config, &mut conn);
                if let Ok(sample) = sample {
                    data.add_sample(key, sample);
                    no_new_keys = false;
                    no_new_keys_streak = 0;
                }
            }
        }

        // Increment, then check our current streak of not finding new keys
        if no_new_keys {
            no_new_keys_streak += 1;
        }
        if no_new_keys_streak == 10 {
            eprintln!(
                "Could only reasonably sample {} keys (of {} requested)",
                data.samples.len(),
                config.n_samples,
            );
            break;
        }

        // Continue sampling until we've surpassed `n_samples`, sleeping after each batch so we
        // don't hammer Redis too hard.
        if data.count() < config.n_samples {
            sleep(Duration::from_millis(config.batch_sleep_ms.into()));
        } else {
            break;
        }
    }

    data
}

// By default, use the batch size in the Config object. If we have just a few samples left to
// collect, simply collect that number directly.
fn this_batch_size(config: &Config, data: &Data) -> usize {
    let n_samples = config.n_samples;
    let default_batch_size = config.batch_size;

    if data.count() + default_batch_size > n_samples {
        n_samples - data.count()
    } else {
        default_batch_size
    }
}

// This uses a single pipelined command of multiple "RANDOMKEY" commands to get the requested number
// of random keys. Note that duplicate keys might be returned by this function.
fn get_random_keys(n_keys: usize, conn: &mut Connection) -> RedisResult<Vec<String>> {
    if n_keys == 0 {
        Ok(Vec::new())
    } else {
        let mut pipe = redis::pipe();
        let mut pipe_ref = pipe.borrow_mut();
        for _ in 0..n_keys {
            pipe_ref = pipe_ref.cmd("RANDOMKEY");
        }
        pipe_ref.query(conn)
    }
}
