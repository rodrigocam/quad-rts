use crate::clock::Clock;

pub struct Game {
    pub clock: Clock,
}

impl Game {
    pub fn new() -> Self {
        Self {
            clock: Clock::new(),
        }
    }

    pub fn start(&mut self) {
        self.clock.start()
    }

    pub fn update(&mut self, delta: f32) -> Result<(), Box<dyn std::error::Error>> {
        println!("updating");
        self.clock.update(delta)?;
        Ok(())
    }
}
