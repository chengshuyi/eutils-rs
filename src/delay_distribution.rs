pub struct DelayDistribution {
    number: usize,
    dis: Vec<usize>,
}

impl DelayDistribution {
    pub fn new() -> DelayDistribution {
        DelayDistribution {
            number: 0,
            dis: Vec::default(),
        }
    }

    /// Delay should be measured in milliseconds, because millisecond jitter
    /// is probably what users care about most. But the delay you enter is
    /// fine in microseconds or nanoseconds.
    pub fn insert(&mut self, delay: usize) {
        if delay > 1_000_000 {
            panic!("delay out of range, max support 1_000_000")
        }

        if delay >= self.dis.len() {
            self.dis.resize(delay + 1, 0);
        }

        self.dis[delay] += 1;
        self.number += 1;
    }

    pub fn show(&self, kinds: usize) {
        // Rounded up
        let width = (self.dis.len() + kinds - 1) / kinds;

        for i in 0..kinds {
            let start = i * width;
            let end = std::cmp::min(start + width, self.dis.len());

            let mut sum = 0;
            for j in start..end {
                sum += self.dis[j];
            }

            print!("{:<6}: ", end);
            for _ in 0..(sum * 100 / self.number) {
                print!("*");
            }

            println!();
        }
    }
}
