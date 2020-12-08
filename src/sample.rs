use redis::Connection;

use crate::parse_args::Config;

#[derive(Debug)]
pub struct Sample {
    pub memory: Option<isize>,
    pub ttl: Option<isize>,
}

pub fn sample_key(key: &String, config: &Config, conn: &mut Connection) -> Option<Sample> {
    // Instantiate an atomic pipeline that we'll use to gets stats about this key
    let mut pipe = redis::pipe();
    let mut pipe_ref = pipe.atomic();

    // Add commands to the pipeline for this key, depending on what stats we've requested
    {
        // Check whether this key exists (in case it's since expired)
        // https://redis.io/commands/exists
        pipe_ref = pipe_ref.cmd("EXISTS").arg(key.clone());

        // Get the memory usage of the key, sampling ALL values if this is a nested data type
        // https://redis.io/commands/memory-usage
        if config.stat_memory {
            pipe_ref = pipe_ref
                .cmd("MEMORY")
                .arg("USAGE")
                .arg(key.clone())
                .arg("SAMPLES")
                .arg("0");
        }

        // Get the TTL of the key in seconds
        // https://redis.io/commands/ttl
        if config.stat_ttl {
            pipe_ref = pipe_ref.cmd("TTL").arg(key.clone());
        }
    }

    // Run the pipeline
    let data: Vec<Option<isize>> = pipe_ref.query(conn).ok()?;

    // If EXISTS failed or returned 0, give up and return early. This can happen when a key gets
    // deleted from Redis between the time we got it from RANDOMKEY and now.
    if data[0].is_none() || data[0] == Some(0) {
        return None;
    }

    // This is kind of foolish, but it works? Wasn't sure how to better handle this, since the
    // "data" Vec is variable in length depending on which stats we're collecting. Also, right now
    // we can conveniently collect our data into a Vec<Option<isize>>, but that might not always be
    // the case if we add more stats in the future.
    let sample = match (config.stat_memory, config.stat_ttl) {
        (true, true) => Sample {
            memory: data[1],
            ttl: data[2],
        },
        (true, false) => Sample {
            memory: data[1],
            ttl: None,
        },
        (false, true) => Sample {
            memory: None,
            ttl: data[1],
        },
        (false, false) => Sample {
            memory: None,
            ttl: None,
        },
    };
    Some(sample)
}
