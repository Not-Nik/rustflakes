# rustflakes
An implementation of Discords version of Twitter snowflakes in Rust.

## Usage
```rust
use rustflakes::SnowflakeWorker;

fn main() {
    let worker_id = 0;
    let process_id = 0;
    let mut worker = SnowflakeWorker::new(worker_id, process_id);
    println!("{}", &worker.make());
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
