# [redis-keyspace-stats](https://crates.io/crates/redis-keyspace-stats)

This program analyzes your Redis keyspace and returns statistics about it. It's somewhat flexible. Usually, you'll
provide [glob-style](https://docs.rs/glob/0.3.0/glob/struct.Pattern.html) patterns to group keys into bins.

This tool is under development, and some obviously useful features are not yet implemented.

Sampling modes:
- ✅&nbsp;&nbsp;[Random](https://redis.io/commands/randomkey) sampling
- 🚧&nbsp;&nbsp;[Random](https://redis.io/commands/randomkey) sampling of keys matching a pattern
- ✅&nbsp;&nbsp;[Scan](https://redis.io/commands/scan) all keys
- 🚧&nbsp;&nbsp;[Scan](https://redis.io/commands/scan) all keys matching a pattern

Statistics:
- ✅&nbsp;&nbsp;[Memory](https://redis.io/commands/memory-usage): total, 50/90/99th percentiles
- ✅&nbsp;&nbsp;[TTL](https://redis.io/commands/ttl): percent with a TTL, 50/90/99th percentiles
- ✅&nbsp;&nbsp;[Data type](https://redis.io/commands/type) breakdown

Output formats:
- ✅&nbsp;&nbsp;Summary pretty-printed table to STDOUT
- 🚧&nbsp;&nbsp;Summary CSV/TSV
- 🚧&nbsp;&nbsp;Summary HTML
- 🚧&nbsp;&nbsp;Raw data CSV/TSV

Redis support:
- ✅&nbsp;&nbsp;Over [TLS](https://redis.io/topics/encryption) (`rediss://` connection strings)
- 🚧&nbsp;&nbsp;[Clusters](https://redis.io/topics/cluster-tutorial)
- 🚧&nbsp;&nbsp;[Logical databases](https://redis.io/commands/select)

## ⚠️ Warnings

- This is **NOT** production-grade software (yet?). Use at your own risk.
- Redis is single-threaded, so be careful about running this against busy production systems. This tool sends commands
  to Redis in batches (and sleeps between them) to reduce its impact, but caution should still be exercised. Start with
  a small sample size, like the default `-n 100` before running more comprehensive analysis.
- Use the `--sample=all` mode with care; the `-n` option is ignored and ALL keys will be enumerated with
  [`SCAN`](https://redis.io/commands/scan)
- This tool fetches keys, and includes a handful of them in its output. While it's rare to store sensitive information
  in Redis keys, it's worth noting. Values are never fetched or included in any output.
- The flags/options/organization/etc of the CLI **will** change as more sampling modes and other features are added

## Installation

For now, there are no published binaries of this tool.

1. Make sure you have Rust installed, or get it via [rustup](https://rustup.rs)
2. Run `cargo install redis-keyspace-stats` to download + compile the binary

## Usage examples

Invoking the CLI with `-h` or `--help` will print documentation:

```
$ redis-keyspace-stats -h
redis-keyspace-stats 0.4.0

USAGE:
    redis-keyspace-stats [OPTIONS] [--] [patterns]...

ARGS:
    <patterns>...    Glob-style patterns to group keys together

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --batch-size <batch-size>            [default: 100]
        --batch-sleep-ms <batch-sleep-ms>    [default: 100]
    -n, --samples <n-samples>                Ignored when --sample=all is specified [default: 100]
    -o, --out <output-mode>                  [default: table] [possible values: table]
        --sample <sample-mode>               [default: random] [possible values: all, random]
        --stats <stats>...
            [default: memory,ttl] [possible values: memory, ttl, type]

        --url <url>                          [default: redis://127.0.0.1]
```

Let's get some quick memory + TTL stats, sampling 50 keys:

```
$ redis-keyspace-stats --url $REDIS_URL -n 50
+---------+-----------+---------------------+--------------+--------------------+--------------+-----------------+
| Pattern | Key count | Example keys        | Memory (sum) | Memory (p50/90/99) | TTL (% with) | TTL (p50/90/99) |
+---------+-----------+---------------------+--------------+--------------------+--------------+-----------------+
| *       | 50        | company:72#messages | 347.03 kB    | 7.18 kB            | 48.00%       | 1m 51s          |
|         |           | company:92#friends  |              | 11.64 kB           |              | 3m 14s          |
|         |           | user:46#friends     |              | 12.68 kB           |              | 3m 39s          |
|         |           | user:78#messages    |              |                    |              |                 |
|         |           | company:3#memes     |              |                    |              |                 |
+---------+-----------+---------------------+--------------+--------------------+--------------+-----------------+
```

Using what's showing in the "Example keys" column, let's write a few
[glob-style](https://docs.rs/glob/0.3.0/glob/struct.Pattern.html) patterns to bin keys together:

```
$ redis-keyspace-stats --url $REDIS_URL -n 50 'user:*#messages' 'user:?#*' 'company:*'
+-----------------+-----------+---------------------+--------------+--------------------+--------------+-----------------+
| Pattern         | Key count | Example keys        | Memory (sum) | Memory (p50/90/99) | TTL (% with) | TTL (p50/90/99) |
+-----------------+-----------+---------------------+--------------+--------------------+--------------+-----------------+
| user:*#messages | 8         | user:43#messages    | 64.39 kB     | 8.57 kB            | 62.50%       | 2m 16s          |
|                 |           | user:113#messages   |              | 11.71 kB           |              | 3m 12s          |
|                 |           | user:110#messages   |              | 12.4 kB            |              | 3m 16s          |
|                 |           | user:64#messages    |              |                    |              |                 |
|                 |           | user:124#messages   |              |                    |              |                 |
+-----------------+-----------+---------------------+--------------+--------------------+--------------+-----------------+
| user:?#*        | 2         | user:8#friends      | 1.84 kB      | 922 B              | 0.00%        | 0s              |
|                 |           | user:9#friends      |              | 962 B              |              | 0s              |
|                 |           |                     |              | 971 B              |              | 0s              |
+-----------------+-----------+---------------------+--------------+--------------------+--------------+-----------------+
| company:*       | 25        | company:11#memes    | 146.49 kB    | 5.18 kB            | 32.00%       | 1m 56s          |
|                 |           | company:75#messages |              | 10.56 kB           |              | 3m              |
|                 |           | company:110#friends |              | 11.91 kB           |              | 3m 3s           |
|                 |           | company:32#memes    |              |                    |              |                 |
|                 |           | company:30#memes    |              |                    |              |                 |
+-----------------+-----------+---------------------+--------------+--------------------+--------------+-----------------+
| *               | 15        | user:117#memes      | 106.29 kB    | 7.17 kB            | 33.33%       | 56s             |
|                 |           | user:125#friends    |              | 11.73 kB           |              | 1m 16s          |
|                 |           | user:116#friends    |              | 12.46 kB           |              | 1m 16s          |
|                 |           | user:42#memes       |              |                    |              |                 |
|                 |           | user:92#friends     |              |                    |              |                 |
+-----------------+-----------+---------------------+--------------+--------------------+--------------+-----------------+
```

Note that the **first** pattern that matches a key will determine the group.

## Development

### Testing locally

- Seed some fake test data via environment variable: `RKS_SEED_FAKE_DATA=true cargo run -- --sample=all 'company:*'`
- Starting the `redis-cli` binary and running `monitor` can be useful for debugging

### Releasing

1. Bump the `version` in `Cargo.toml`
2. Update the usage example in the `README.md`
3. Commit, add a git tag for the [release](https://github.com/scott113341/redis-keyspace-stats/releases), and push
4. Run `cargo publish`
