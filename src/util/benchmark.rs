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

    pub fn get_bench_time(&self) -> String {
        let end = self.start_time.elapsed();
        format!(
            "Process {}.{:03} msec",
            end.as_micros() / 1000,
            end.as_micros() % 1000,
        )
    }
}
