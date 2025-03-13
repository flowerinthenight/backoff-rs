//! `backoff` provides jittered backoff values (nanoseconds) for
//! operations that needs to do sleeps with jittered backoff
//! between retries. The implementation is based on
//! [https://www.awsarchitectureblog.com/2015/03/backoff.html](https://www.awsarchitectureblog.com/2015/03/backoff.html).

use rand::Rng;
use std::cmp;

/// `Backoff` provides jittered backoff values (nanoseconds) for
/// operations that needs to do sleeps with jittered backoff
/// between retries.
pub struct Backoff {
    /// The initial value of the retry period in ns, defaults to 1s.
    pub initial_ns: u64,

    /// The max value of the retry period in ns, defaults to 30s.
    pub max_ns: u64,

    /// The factor by which the retry period increases. Should be > 1, defaults to 2.
    pub multiplier: f64,

    last: u64,
    iter: u64,
}

impl Backoff {
    pub fn builder() -> BackoffBuilder {
        BackoffBuilder::default()
    }

    /// Returns the next nanosecond duration that the caller should use to backoff.
    pub fn pause(&mut self) -> u64 {
        self.iter += 1;
        if self.initial_ns == 0 {
            self.initial_ns = 1_000_000_000;
        }

        if self.max_ns == 0 {
            self.max_ns = 30 * 1_000_000_000;
        }

        if self.multiplier == 0.0 {
            self.multiplier = 2.0;
        }

        if self.iter == 1 {
            return self.initial_ns;
        }

        let upper = self.last as f64 * self.multiplier;
        let mut rng = rand::rng();
        let rval = 1 + rng.random_range(0..=upper as u64);
        self.last = cmp::min(self.max_ns, rval);
        return self.last;
    }
}

/// `BackoffBuilder` builds an instance of Backoff with default values.
#[derive(Default)]
pub struct BackoffBuilder {
    initial_ns: u64,
    max_ns: u64,
    multiplier: f64,
}

impl BackoffBuilder {
    pub fn new() -> BackoffBuilder {
        BackoffBuilder::default()
    }

    pub fn initial(mut self, v: u64) -> BackoffBuilder {
        self.initial_ns = v;
        self
    }

    pub fn max(mut self, v: u64) -> BackoffBuilder {
        self.max_ns = v;
        self
    }

    pub fn multiplier(mut self, v: f64) -> BackoffBuilder {
        self.multiplier = v;
        self
    }

    pub fn build(self) -> Backoff {
        Backoff {
            initial_ns: self.initial_ns,
            max_ns: self.max_ns,
            multiplier: self.multiplier,
            last: 1_000_000_000,
            iter: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut bo = BackoffBuilder::new().build();
        let def = bo.pause();
        assert_eq!(def, 1_000_000_000);
        let val = bo.pause();
        assert!(val > 0 && val <= bo.max_ns);
    }
}
