pub trait RateLimiter {
    fn allow(&mut self, request_time: i64) -> bool;
}
