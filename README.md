# rustflakes
An implementation of Discord's version of Twitter's snowflakes in Rust.

## Usage
```rust
use rustflakes::SnowflakeWorker;

fn main() {
    let worker_id = 1;
    let process_id = 2;
    let mut worker = SnowflakeWorker::new(worker_id, process_id);
    let snowflake = Snowflake::from(worker.make());
    println!("Created snowflake {}", snowflake.flake);
    println!("Snowflake created at: {}", snowflake.timestamp);
    println!("Snowflake created by worker {}", snowflake.worker_id);
    println!("Snowflake created by process {}", snowflake.process_id);
    println!("Snowflakes increment is {}", snowflake.increment);
}
```
## Structure
Snowflakes are structured as defined in the [Discord API documentation](https://discord.com/developers/docs/reference#snowflakes-snowflake-id-format-structure-left-to-right).

Field               | Bits     | Number of Bits | Description
--------------------|----------|----------------|------------
Timestamp           | 63 to 22 | 42 bits        | Milliseconds since Discord Epoch, the first second of 2015 or 1420070400000.
Internal worker ID  | 21 to 17 | 5 bits         |
Internal process ID | 16 to 12 | 5 bits         |
Increment           | 11 to 0  | 12 bits        | For every ID that is generated on that process, this number is incremented
