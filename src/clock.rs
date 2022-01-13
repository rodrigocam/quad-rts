use std::fmt::Formatter;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct ClockUninitializedError;

impl std::fmt::Display for ClockUninitializedError {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}
impl std::error::Error for ClockUninitializedError {}

pub struct Clock {
    start: Option<Instant>,
    current: Option<Instant>,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            start: None,
            current: None,
        }
    }

    pub fn start(&mut self) {
        if self.start.is_none() && self.current.is_none() {
            self.start = Some(Instant::now());
            self.current = Some(self.start.unwrap());
        }
    }

    pub fn update(&mut self, seconds: f32) -> Result<(), ClockUninitializedError> {
        if let Some(current) = self.current.as_mut() {
            *current += Duration::from_secs(seconds as u64);
            return Ok(());
        }
        Err(ClockUninitializedError)
    }
}
