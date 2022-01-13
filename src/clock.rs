use std::collections::HashMap;
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

pub struct Alarm {
    last_rang: Option<Instant>,
    interval: f32,
}

impl Alarm {
    pub fn new(interval: f32) -> Self {
        Alarm {
            last_rang: Some(Instant::now()),
            interval,
        }
    }

    pub fn update(&mut self, current_time: Instant) -> bool {
        if let Some(last_rang) = self.last_rang {
            if last_rang + Duration::from_secs_f32(self.interval) <= current_time {
                self.last_rang = Some(current_time);
                return true;
            }
        }
        false
    }
}

pub struct Clock {
    start: Instant,
    current: Instant,

    alarms: HashMap<String, (Alarm, Box<dyn Fn() -> ()>)>,
}

impl Clock {
    pub fn start() -> Self {
        Self {
            start: Instant::now(),
            current: Instant::now(),
            alarms: HashMap::new(),
        }
    }

    pub fn set_alarm(&mut self, id: String, interval: f32, action: &'static dyn Fn() -> ()) {
        self.alarms
            .insert(id, (Alarm::new(interval), Box::new(action)));
    }

    pub fn update(&mut self, seconds: f32) -> Result<(), ClockUninitializedError> {
        self.current += Duration::from_secs_f32(seconds);

        for (_id, (alarm, action)) in self.alarms.iter_mut() {
            if alarm.update(self.current) {
                action();
            }
        }
        Ok(())
    }
}
