use std::time::{Duration, Instant};

pub struct Benchmark {
    start_time: Instant,
    end_time: Duration,
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
            end_time: Duration::new(0, 0),
        }
    }

    pub fn start(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn stop(&mut self) {
        self.end_time = self.start_time.elapsed();
    }

    pub fn to_string(&self) -> String {
        if self.end_time.as_secs_f32() > 1.0 {
            format!(
                "{}.{:03} sec",
                self.end_time.as_millis() / 1000,
                self.end_time.as_millis() % 1000
            )
        } else {
            format!(
                "{}.{:03} ms",
                self.end_time.as_micros() / 1000,
                self.end_time.as_micros() % 1000,
            )
        }
    }

    pub fn print(&self) {
        println!("time: {}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_work_benchmark() {
        let mut benchmark: Benchmark = Benchmark::new();
        benchmark.start();
        sleep(Duration::from_millis(100));
        println!("{:?}", benchmark.stop());
        benchmark.start();
        sleep(Duration::from_millis(1500));
        println!("{:?}", benchmark.stop());
    }
}
