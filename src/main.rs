use clap::Clap;

mod config;
mod data;
mod metadata;
mod output;
mod sampling;
mod seed;
mod stats;

fn main() {
    // Parse CLI args into a Config struct
    let mut config = config::Config::parse();
    config.normalize();

    // Connect to Redis
    let mut conn = redis_connection(config.url.clone()).unwrap();

    // Optionally seed fake data
    if let Ok(seed_env) = std::env::var("RKS_SEED_FAKE_DATA") {
        if seed_env == "true" {
            seed::seed_fake_data(128, &mut conn).unwrap();
        }
    }

    // Get metadata and sample data from Redis
    let metadata = metadata::get_metadata(&mut conn);
    let data = sampling::collect_samples(&config, &mut conn);

    // Display stats
    eprintln!(
        "Sampled {} of {} keys in db{}",
        data.sample_count(),
        metadata.total_keys,
        metadata.redis_db,
    );
    output::output(&config, &metadata, &data);
}

fn redis_connection(url: String) -> redis::RedisResult<redis::Connection> {
    let client = redis::Client::open(url)?;
    client.get_connection()
}

#[cfg(test)]
mod tests {
    // This doesn't test anything, it's just a helper function that returns a basic config and
    // Redis connection for use in other tests.
    pub fn test_config_and_conn() -> (crate::config::Config, redis::Connection) {
        let config = crate::config::Config {
            sample_mode: crate::sampling::SampleMode::Random,
            n_samples: 1,
            batch_size: 1,
            batch_sleep_ms: 0,
            stats: crate::stats::Stats::all(),
            output_mode: crate::output::OutputMode::StdoutTable,
            url: "redis://127.0.0.1".to_string(),
            patterns: vec![],
        };

        let conn = crate::redis_connection(config.url.clone()).unwrap();

        (config, conn)
    }
}
