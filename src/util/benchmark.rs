use log::debug;
use std::time::Instant;

pub struct Benchmark {
    start_time: Instant,
}

impl Benchmark {
    pub fn set_start_time() -> Benchmark {
        let now_time = Instant::now();
        Benchmark {
            start_time: now_time,
        }
    }

    pub fn print_bench_time(&self) {
        let end = self.start_time.elapsed();
        debug!(
            "Process {}.{:03} msec",
            end.as_micros() / 1000,
            end.as_micros() % 1000,
        );
    }
}
