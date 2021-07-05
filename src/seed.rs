pub(crate) fn seed_fake_data(
    count: usize,
    conn: &mut redis::Connection,
) -> Result<(), redis::RedisError> {
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
