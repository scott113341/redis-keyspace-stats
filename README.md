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
- ✅&nbsp;&nbsp;[Memory](https://redis.io/commands/memory-usage): total, total estimated, 50/90/99th percentiles
- ✅&nbsp;&nbsp;[TTL](https://redis.io/commands/ttl): percent with a TTL, 50/90/99th percentiles
- ✅&nbsp;&nbsp;[Data type](https://redis.io/commands/type) breakdown

Output formats:
- ✅&nbsp;&nbsp;Summary pretty-printed table to STDOUT
- 🚧&nbsp;&nbsp;Summary CSV/TSV
- 🚧&nbsp;&nbsp;Summary HTML
- 🚧&nbsp;&nbsp;Raw data CSV/TSV

Redis support:
- ✅&nbsp;&nbsp;Over [TLS](https://redis.io/topics/encryption) (`rediss://` connection strings)
- ✅&nbsp;&nbsp;[Logical databases](https://redis.io/commands/select), specified in the connection string (`redis://host:port/db`)
- 🚧&nbsp;&nbsp;[Clusters](https://redis.io/topics/cluster-tutorial)

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
Sampled 50 of 128 keys in db0
┌─────────┬────────────────┬────────────────────┬────────────────────────┬─────────────────┐
│ Pattern │ Keys           │ Example keys       │ Memory                 │ TTL             │
├─────────┼────────────────┼────────────────────┼────────────────────────┼─────────────────┤
│ *       │ 50 counted     │ company:121#memes  │ 360.77 kB (sum)        │ 54.00% have TTL │
│         │ 128 est. total │ user:79#memes      │ 923.57 kB (est. total) │ 2m 47s (p50)    │
│         │                │ user:8#memes       │ 7.75 kB (p50)          │ 3m 57s (p90)    │
│         │                │ user:92#friends    │ 11.9 kB (p90)          │ 4m 13s (p99)    │
│         │                │ company:84#friends │ 12.87 kB (p99)         │                 │
└─────────┴────────────────┴────────────────────┴────────────────────────┴─────────────────┘
```

Using what's showing in the "Example keys" column, let's write a few
[glob-style](https://docs.rs/glob/0.3.0/glob/struct.Pattern.html) patterns to bin keys together:

```
$ redis-keyspace-stats --url $REDIS_URL -n 50 'user:*#messages' 'user:?#*' 'company:*'
Sampled 50 of 128 keys in db0
┌─────────────────┬───────────────┬────────────────────┬────────────────────────┬──────────────────┐
│ Pattern         │ Keys          │ Example keys       │ Memory                 │ TTL              │
├─────────────────┼───────────────┼────────────────────┼────────────────────────┼──────────────────┤
│ user:*#messages │ 8 counted     │ user:27#messages   │ 30.27 kB (sum)         │ 37.50% have TTL  │
│                 │ 20 est. total │ user:34#messages   │ 77.5 kB (est. total)   │ 1m 30s (p50)     │
│                 │               │ user:13#messages   │ 3.4 kB (p50)           │ 3m 25s (p90)     │
│                 │               │ user:58#messages   │ 6.22 kB (p90)          │ 3m 51s (p99)     │
│                 │               │ user:60#messages   │ 6.22 kB (p99)          │                  │
├─────────────────┼───────────────┼────────────────────┼────────────────────────┼──────────────────┤
│ user:?#*        │ 2 counted     │ user:3#friends     │ 1.25 kB (sum)          │ 100.00% have TTL │
│                 │ 5 est. total  │ user:8#memes       │ 3.19 kB (est. total)   │ 1m 52s (p50)     │
│                 │               │                    │ 624 B (p50)            │ 2m 39s (p90)     │
│                 │               │                    │ 822.4 B (p90)          │ 2m 50s (p99)     │
│                 │               │                    │ 867.04 B (p99)         │                  │
├─────────────────┼───────────────┼────────────────────┼────────────────────────┼──────────────────┤
│ company:*       │ 24 counted    │ company:1#messages │ 162.13 kB (sum)        │ 45.83% have TTL  │
│                 │ 61 est. total │ company:75#memes   │ 415.05 kB (est. total) │ 2m 51s (p50)     │
│                 │               │ company:10#friends │ 7.24 kB (p50)          │ 3m 58s (p90)     │
│                 │               │ company:69#memes   │ 11.85 kB (p90)         │ 4m 2s (p99)      │
│                 │               │ company:6#memes    │ 12.64 kB (p99)         │                  │
├─────────────────┼───────────────┼────────────────────┼────────────────────────┼──────────────────┤
│ *               │ 16 counted    │ user:123#memes     │ 149.02 kB (sum)        │ 31.25% have TTL  │
│                 │ 41 est. total │ user:71#friends    │ 381.5 kB (est. total)  │ 2m 10s (p50)     │
│                 │               │ user:25#friends    │ 10.05 kB (p50)         │ 2m 59s (p90)     │
│                 │               │ user:86#memes      │ 12.86 kB (p90)         │ 3m 4s (p99)      │
│                 │               │ user:110#friends   │ 13.29 kB (p99)         │                  │
└─────────────────┴───────────────┴────────────────────┴────────────────────────┴──────────────────┘
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
