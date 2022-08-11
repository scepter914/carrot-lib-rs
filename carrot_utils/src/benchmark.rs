use std::time::Instant;
use std::{thread, time};

pub struct Benchmark {
    start_time: Instant,
}

impl Default for Benchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark {
    pub fn new() -> Self {
        Benchmark {
            start_time: Instant::now(),
        }
    }

    pub fn start(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn stop(&self) -> String {
        let end: time::Duration = self.start_time.elapsed();
        if end.as_secs_f32() > 1.0 {
            format!(
                "{}.{:03} sec",
                end.as_millis() / 1000,
                end.as_millis() % 1000
            )
        } else {
            format!(
                "{}.{:03} ms",
                end.as_micros() / 1000,
                end.as_micros() % 1000,
            )
        }
    }
}

#[test]
fn test_work_benchmark() {
    let mut benchmark: Benchmark = Benchmark::new();
    benchmark.start();
    thread::sleep(time::Duration::from_millis(100));
    println!("{}", benchmark.stop());
    benchmark.start();
    thread::sleep(time::Duration::from_millis(1500));
    println!("{}", benchmark.stop());
}
