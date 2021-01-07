use super::prelude::*;

// Implementation of Token Bucket Algorithm
// specified in https://en.wikipedia.org/wiki/Token_bucket
// Works better with single database.
#[derive(Debug, PartialEq)]
pub struct TokenBucket {
    // Timestamp of user's last access in seconds.
    pub last_access: i64,
    pub capacity: i64,
    // Refers to tokens left for consumption.
    pub tokens: i64,
    // Number of tokens to add per min.
    pub refill_rate: i64,
}
impl TokenBucket {
    fn new(last_access: i64, capacity: i64, tokens: i64, refill_rate: i64) -> Self {
        Self {
            last_access,
            capacity,
            tokens,
            refill_rate,
        }
    }
    fn refill(&mut self, request_time: i64) {
        use std::cmp::min;
        let time_elasped_in_sec = request_time - self.last_access;
        let to_add = self.refill_rate * (time_elasped_in_sec / 60);
        self.tokens = min(self.capacity, self.tokens + to_add);
    }
}
impl RateLimiter for TokenBucket {
    fn allow(&mut self, request_time: i64) -> bool {
        // Refill token since last access.
        self.refill(request_time);

        self.last_access = request_time;
        // Consume tokens if possible
        if self.tokens == 0 {
            return false;
        }
        self.tokens = self.tokens - 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::{super::prelude::*, TokenBucket};
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_refill_buket() {
        let last_access = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
        let mut bucket = TokenBucket::new(last_access.timestamp(), 10, 1, 2);

        // No refill should happen as < 1 min has elasped.
        let request_time = Utc.ymd(2021, 1, 1).and_hms(0, 0, 2);
        bucket.refill(request_time.timestamp());
        assert_eq!(bucket.tokens, 1);

        // Should refill
        let after_2_mins = Utc.ymd(2021, 1, 1).and_hms(0, 2, 0);
        bucket.refill(after_2_mins.timestamp());
        assert_eq!(bucket.tokens, 5);

        // Should only refill up to capacity.
        let after_10_mins = Utc.ymd(2021, 1, 1).and_hms(0, 10, 0);
        bucket.refill(after_10_mins.timestamp());
        assert_eq!(bucket.tokens, bucket.capacity);
    }

    #[test]
    fn test_rate_limit() {
        let late_access = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
        let mut bucket = TokenBucket::new(late_access.timestamp(), 10, 1, 2);

        // No refill
        let after_2_seconds = Utc.ymd(2021, 1, 1).and_hms(0, 0, 2);
        let res = bucket.allow(after_2_seconds.timestamp());
        assert_eq!(res, true);
        assert_eq!(
            bucket,
            TokenBucket {
                last_access: after_2_seconds.timestamp(),
                capacity: 10,
                tokens: 0,
                refill_rate: 2
            }
        );

        // No tokens left
        let after_3_seconds = Utc.ymd(2021, 1, 1).and_hms(0, 0, 3);
        let res = bucket.allow(after_3_seconds.timestamp());
        assert_eq!(res, false);
        assert_eq!(
            bucket,
            TokenBucket {
                last_access: after_3_seconds.timestamp(),
                capacity: 10,
                tokens: 0,
                refill_rate: 2
            }
        );

        // Refilled 4 tokens, consumed 1.
        let req_time = Utc.ymd(2021, 1, 1).and_hms(0, 2, 30);
        let res = bucket.allow(req_time.timestamp());
        assert_eq!(res, true);
        assert_eq!(
            bucket,
            TokenBucket {
                last_access: req_time.timestamp(),
                capacity: 10,
                tokens: 3,
                refill_rate: 2
            }
        );

        // Refilled 6 tokens, consumed 1.
        let req_time = Utc.ymd(2021, 1, 1).and_hms(0, 6, 0);
        let res = bucket.allow(req_time.timestamp());
        assert_eq!(res, true);
        assert_eq!(
            bucket,
            TokenBucket {
                last_access: req_time.timestamp(),
                capacity: 10,
                tokens: 8,
                refill_rate: 2
            }
        );
    }
}
