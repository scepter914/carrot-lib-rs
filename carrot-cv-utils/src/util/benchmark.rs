use std::time::Instant;

pub struct Benchmark {
    start_time: Instant,
}

impl Benchmark {
    pub fn new() -> Benchmark {
        Benchmark {
            start_time: Instant::now(),
        }
    }

    pub fn start(&mut self) {
        self.start_time = Instant::now();
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
