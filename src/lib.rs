use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(test)]
mod tests {
    use crate::{SnowflakeWorker};

    #[test]
    fn test() {
        let mut worker = SnowflakeWorker::new(0, 0);
        // These might fail on an unreasonably slow computer
        assert_eq!(SnowflakeWorker::get_timestamp(), worker.make() >> 22);
        worker.increment = 42;
        assert_eq!(42, /*Clear the left-most 52 bits*/ worker.make() << 52 >> 52);
    }

    #[test]
    fn readme() {
        let worker_id = 0;
        let process_id = 0;
        let worker = SnowflakeWorker::new(worker_id, process_id);
        println!("{}", &worker.make());
    }
}

pub struct SnowflakeWorker
{
    worker_id: i64,
    process_id: i64,
    increment: i64
}

impl SnowflakeWorker
{
    pub fn new(worker_id: i64, process_id: i64) -> SnowflakeWorker
    {
        SnowflakeWorker { worker_id, process_id, increment: 0 }
    }

    fn get_timestamp() -> i64 {
        SystemTime::now().duration_since(UNIX_EPOCH)
            .expect("")
            .as_millis() as i64 - 1420070400000
    }

    pub fn make(&self) -> i64 {
        SnowflakeWorker::get_timestamp() << 22 | self.worker_id << 17 | self.process_id << 12 | self.increment
    }
}
