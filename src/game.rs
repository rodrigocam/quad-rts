use crate::clock::Clock;

pub struct Game {
    clock: Clock,
}

impl Game {
    pub fn new() -> Self {
        Self {
            clock: Clock::start(),
        }
    }

    pub fn start(&mut self) {
        self.clock.set_alarm("spawn_units".to_owned(), 60.0, &|| {
            println!("spawning units")
        });
    }

    pub fn update(&mut self, delta: f32) -> Result<(), Box<dyn std::error::Error>> {
        // println!("updating");
        self.clock.update(delta)?;
        Ok(())
    }
}
