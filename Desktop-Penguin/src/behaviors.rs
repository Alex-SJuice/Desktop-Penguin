use rand::distr::{Distribution, Uniform};
use rand::RngExt;
use raylib::prelude::*;

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

    pub fn trigger(&mut self) {
        self.timer = rand::rng().random_range(1.0..3.0);
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
        Self {}
    }

    pub fn trigger(&mut self) {}

    pub fn update(&mut self) {}

    pub fn running(&self) -> bool {
        false
    }

    pub fn stop(&mut self) {}
}

pub struct Grab {
    grabbed: bool,
    grab_offset: Vector2,
    last_moving_mouse_position: Vector2,
}

impl Grab {
    pub fn new() -> Self {
        Self {
            grabbed: false,
            grab_offset: Vector2::ZERO,
            last_moving_mouse_position: Vector2::ZERO,
        }
    }

    pub fn update(&mut self, mouse_pressed: bool, mouse_position: Vector2) -> Option<Vector2> {
        if !self.running() && mouse_pressed {
            self.trigger(mouse_position);
            return None;
        }
        if self.running() && mouse_pressed {
            return Some(self.run(mouse_position));
        }
        if self.running() && !mouse_pressed {
            return Some(self.stop());
        }
        if !self.running() && !mouse_pressed {
            self.stopped();
            return None;
        }
        return None;
    }

    fn trigger(&mut self, mouse_position: Vector2) {
        self.grabbed = true;
        self.grab_offset = mouse_position;
    }

    fn run(&mut self, mouse_position: Vector2) -> Vector2 {
        if (mouse_position - self.last_moving_mouse_position).length() > 0.1 {
            self.last_moving_mouse_position = mouse_position;
        }
        return mouse_position - self.grab_offset;
    }

    fn stop(&mut self) -> Vector2 {
        self.grabbed = false;
        (self.last_moving_mouse_position - self.grab_offset)
            .normalize()
            .scale(800.0)
    }

    fn stopped(&self) {

    }

    pub fn running(&self) -> bool {
        self.grabbed
    }
}
