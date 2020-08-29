use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(test)]
mod tests {
    use crate::{SnowflakeWorker, Snowflake};

    #[test]
    fn test() {
        let mut worker = SnowflakeWorker::new(1, 2);
        // These might fail on an unreasonably slow computer
        assert_eq!(SnowflakeWorker::get_timestamp(), worker.make() >> 22);
        worker.increment = 42;
        let snowflake = Snowflake::from(worker.make());
        assert_eq!(snowflake.increment, 42);
        assert_eq!(snowflake.worker_id, 1);
        assert_eq!(snowflake.process_id, 2);
    }

    #[test]
    fn readme() {
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

    #[test]
    fn discord_doc() {
        let snowflake = Snowflake::from(175928847299117063);
        assert_eq!(snowflake.timestamp, 1462015105796);
        assert_eq!(snowflake.worker_id, 1);
        assert_eq!(snowflake.process_id, 0);
        assert_eq!(snowflake.increment, 7);
    }
}

pub struct SnowflakeWorker {
    worker_id: u64,
    process_id: u64,
    increment: u64,
}

impl SnowflakeWorker {
    pub fn new(worker_id: u64, process_id: u64) -> SnowflakeWorker {
        SnowflakeWorker { worker_id, process_id, increment: 0 }
    }

    fn get_timestamp() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).expect("").as_millis() as u64 - 1420070400000
    }

    pub fn make(&mut self) -> u64 {
        self.increment += 1;
        SnowflakeWorker::get_timestamp() << 22 | (self.worker_id & 31) << 17 | (self.process_id & 31) << 12 | ((self.increment - 1) & 0xFFF)
    }
}

pub struct Snowflake {
    /// Unix timestamp
    pub timestamp: u64,
    /// Worker that created this Snowflake, internal
    pub worker_id: u64,
    /// Process that created this Snowflake, internal
    pub process_id: u64,
    /// Increment of this Snowflake, internal
    pub increment: u64,
    /// Raw snowflake
    pub flake: u64,
}

impl Snowflake {
    pub fn from(snowflake: u64) -> Snowflake {
        Snowflake {
            timestamp: (snowflake >> 22) + 1420070400000,
            worker_id: (snowflake & 0x3E0000) >> 17,
            process_id: (snowflake & 0x1F000) >> 12,
            increment: snowflake & 0xFFF,
            flake: snowflake,
        }
    }
}
