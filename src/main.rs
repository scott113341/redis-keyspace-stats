mod data;
mod output;
mod parse_args;
mod sample;

use clap::Clap;
use redis::{Client, Connection};

fn main() {
    // Parse CLI args into a Config struct
    let mut config = parse_args::Config::parse();
    config.normalize();

    // Connect to Redis
    let client = Client::open(config.url.clone()).unwrap();
    let mut conn = client.get_connection().unwrap();

    // (Optionally) seed fake data
    // seed_fake_data(128, &mut conn).unwrap();

    // Get sample data from Redis
    let data = data::get_data::get_data(&config, &mut conn);

    // Display stats
    output::output(&config, &data);
}

#[allow(dead_code)]
fn seed_fake_data(count: usize, conn: &mut Connection) -> Result<(), redis::RedisError> {
    use rand::random;

    let fake_resources = vec!["user", "company"];
    let fake_attributes = vec!["friends", "messages", "memes"];

    let mut pipe = redis::pipe();
    let mut pipe_ref = pipe.atomic();

    for i in 1..=count {
        let resource = fake_resources[random::<u8>() as usize % fake_resources.len()];
        let attribute = fake_attributes[random::<u8>() as usize % fake_attributes.len()];
        let key = format!("{}:{}#{}", resource, i, attribute);

        // This will be like "some_value________", but with (i * 100) trailing underscores
        let value = format!("some_value{:_<1$}", "", i * 100);
        pipe_ref = pipe_ref.set(&key, value);

        // Set a TTL for ~half of keys
        if random::<bool>() {
            pipe_ref = pipe_ref.expire(&key, random::<u8>() as usize);
        }
    }

    pipe_ref.query(conn)?;
    Ok(())
}
