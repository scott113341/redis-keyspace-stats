use redis::{Connection, ConnectionLike};
use regex::Regex;

pub struct Metadata {
    pub redis_db: i64,
    pub total_keys: u64,
}

pub fn get_metadata(conn: &mut Connection) -> Metadata {
    let redis_db = conn.get_db();
    let total_keys = get_total_keys(conn).unwrap();

    Metadata {
        redis_db,
        total_keys,
    }
}

fn get_total_keys(conn: &mut Connection) -> Result<u64, String> {
    // The current logical Redis database
    let db = conn.get_db();
    let db_key = format!("db{}", db);

    // See the "keyspace" section within: https://redis.io/commands/info
    let res: redis::InfoDict = redis::cmd("INFO")
        .arg("keyspace")
        .query(conn)
        .or_else(|e| Err(e.to_string()))?;

    // This will be a String like: "keys=321,expires=123,avg_ttl=456"
    let db_info: String = res.get(&db_key).ok_or(format!("{} not found", db_key))?;

    // Extract and parse the "keys" value
    Regex::new(r"keys=(?P<keys>\d+)")
        .unwrap()
        .captures(&db_info)
        .and_then(|caps| caps["keys"].parse().ok())
        .ok_or(format!("Key count failed for {}", db))
}
