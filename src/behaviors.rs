use rand::distr::{Distribution, Uniform};
use rand::RngExt;
use raylib::prelude::*;
use std::ops::Range;

// 0.0 < probability < 1.0
// frequency in seconds
pub fn build_randomizer(probability: f64, frequency: f64) -> impl Fn(f64) -> bool {
    let uniform = Uniform::new(0.0, 1.0).unwrap();
    move |time: f64| -> bool {
        if time % frequency < 0.01 && uniform.sample(&mut rand::rng()) < probability {
            return true;
        }
        return false;
    }
}

//Basic template of behavior, DON'T ACTUALLY IMPLEMENT
pub trait Behavior {
    fn update(&mut self, should_run: bool) {
        if !self.running() && should_run {
            self.trigger();
            return;
        }
        if self.running() && should_run {
            self.run();
            return;
        }
        if self.running() && !should_run {
            self.stop();
            return;
        }
        if !self.running() && !should_run {
            self.stopped();
            return;
        }
    }

    fn trigger(&mut self);

    fn run(&mut self);

    fn stop(&mut self);

    fn stopped(&mut self);

    fn running(&self) -> bool;
}

pub struct Pause {
    timer: f32,
    wait: Range<f32>,
}

impl Pause {
    pub fn new(wait: Range<f32>) -> Self {
        Self {
            timer: 0.0,
            wait,
        }
    }

    pub fn update(&mut self, should_run: bool, delta: f32) {
        if !self.running() && should_run {
            self.trigger();
            return;
        }
        if self.running() {
            self.run(delta);
            return;
        }
        if !self.running() && !should_run {
            self.stopped();
            return;
        }
    }

    fn trigger(&mut self) {
        self.timer = rand::rng().random_range(self.wait.clone());
    }

    fn run(&mut self, delta: f32) {
        self.timer -= delta;
    }

    fn stop(&mut self) {}

    fn stopped(&mut self) {
        self.timer = 0.0;
    }


    pub fn running(&self) -> bool {
        self.timer > 0.0
    }
}

pub struct DirectionChange {
    change: bool,
}

impl DirectionChange {
    pub fn new() -> Self {
        Self { change: false }
    }

    pub fn update(&mut self, should_run: bool) {
        if !self.running() && should_run {
            self.trigger();
            return;
        }
        if self.running() && should_run {
            self.run();
            return;
        }
        if self.running() && !should_run{
            self.stop();
            return;
        }
        if !self.running() && !should_run {
            self.stopped();
            return;
        }
    }

    fn trigger(&mut self) {
        self.change = true;
    }

    fn run(&mut self) {
        self.change = false;
    }

    fn stop(&mut self) {
        self.change = false;
    }

    fn stopped(&mut self) {
        self.change = false;
    }

    pub fn running(&self) -> bool {
        self.change
    }
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

    pub fn update(&mut self, should_run: bool, mouse_position: Vector2) -> Option<Vector2> {
        if !self.running() && should_run {
            self.trigger(mouse_position);
            return None;
        }
        if self.running() && should_run {
            return Some(self.run(mouse_position));
        }
        if self.running() && !should_run {
            return Some(self.stop());
        }
        if !self.running() && !should_run {
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

pub struct Slide {
    sliding: bool,
    waddle_speed: f32,
    slide_speed: f32,
}

impl Slide {
    pub fn new(waddle_speed: f32, slide_speed: f32) -> Self {
        Self {
            sliding: false,
            waddle_speed,
            slide_speed, 
        }
    }

    pub fn update(&mut self, should_toggle: bool) -> f32 {
        if !self.running() && should_toggle {
            return self.trigger();
        }
        if self.running() && should_toggle {
            return self.stop();
        }
        if self.running() && !should_toggle {
            return self.run();
        }
        if !self.running() && !should_toggle {
            return self.stopped();
        }
        0.0
    }

    fn trigger(&mut self) -> f32 {
        self.sliding = true;
        self.slide_speed
    }

    fn run(&mut self) -> f32 {
        self.slide_speed
    }

    fn stop(&mut self) -> f32 {
        self.sliding = false;
        self.waddle_speed
    }

    fn stopped(&mut self) -> f32 {
        self.waddle_speed
    }

    pub fn running(&self) -> bool {
        self.sliding
    }
}

pub struct Investigate {
    interest: f32,
    interest_threshold: f32,
    interest_rate: f32,
    disinterest_rate: f32,
    investigating: bool,
}

impl Investigate {
    pub fn new(interest_threshold: f32, interest_rate: f32, disinterest_rate: f32) -> Self {
        Self {
            interest: 0.0,
            interest_threshold,
            interest_rate,
            disinterest_rate,
            investigating: false,
        }
    }

    pub fn update(&mut self, distance: f32) {
        if !self.running() && self.interest > self.interest_threshold {
            self.trigger();
            return;
        }
        if self.running() && self.interest > 0.0 {
            self.run(distance);
            return;
        }
        if self.running() && self.interest <= 0.0 {
            self.stop();
            return;
        }
        if !self.running() && self.interest <= self.interest_threshold {
            self.stopped(distance);
            return;
        }
    }

    fn trigger(&mut self) {
        self.investigating = true;
    }

    fn run(&mut self, distance: f32) {
        self.interest -= self.disinterest_rate * (1.0 / distance).clamp(0.0, 500.0);
    }

    fn stop(&mut self) {
        self.investigating = false;
        self.interest = 0.0;
    }

    fn stopped(&mut self, distance: f32) {
        self.interest += distance * self.interest_rate;
    }

    pub fn running(&self) -> bool {
        self.investigating
    }
}