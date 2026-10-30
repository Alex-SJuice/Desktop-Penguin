use rand::distr::{Distribution, Uniform};
use rand::RngExt;

// 0.0 < probability < 1.0
pub fn build_randomizer(probability: f32) -> impl Fn() -> bool {
    let uniform = Uniform::new(0.0, 1.0).unwrap();
    move || -> bool {
        if uniform.sample(&mut rand::rng()) < probability {
            return true;
        }
        return false;
    }
}

pub struct Pause {
    timer: f32,
}

impl Pause {
    pub fn new() -> Self {
        Self {
            timer: 0.0,
        }
    }

    pub fn trigger(&mut self, randomizer: &impl Fn() -> bool) -> bool {
        if randomizer() {
            self.timer = rand::rng().random_range(1.0..3.0);
            return true;
        }
        return false;
    }

    pub fn update(&mut self, delta: f32) {
        self.timer -= delta;
        if !self.running() {
            self.stop();
        }
    }

    pub fn running(&self) -> bool {
        self.timer > 0.0
    }

    pub fn stop(&mut self) {
        self.timer = 0.0;
    }
}

pub struct DirectionChange {

}

impl DirectionChange {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn trigger(&mut self, randomizer: &impl Fn() -> bool) -> bool {
        randomizer()
    }

    pub fn update(&mut self) {

    }

    pub fn running(&self) -> bool {
        false
    }

    pub fn stop(&mut self) {

    }
}
