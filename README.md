# rustflakes
An implementation of Discords version of Twitter snowflakes in Rust.

## Usage
```rust
use rustflakes::SnowflakeWorker;

fn main() {
    let worker_id = 0;
    let process_id = 0;
    let worker = SnowflakeWorker::new(worker_id, process_id);
    println!("{}", &worker.make());
}
```
