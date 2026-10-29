use rand::distr::{Uniform, Distribution};

// 0.0 < probability < 1.0
pub fn attach_randomizer(probability: f32, run_on_trigger: impl Fn()) -> impl Fn() {
	let uniform = Uniform::new(0.0, 1.0).unwrap();
	move || {
		if uniform.sample(&mut rand::rng()) < probability {
			run_on_trigger();
		}
	}
}

pub trait Behavior {
	//attempt to start behavior. returns whether it actually started
	pub fn trigger(&self) -> bool;

	pub fn update<T> ()

	//check if behavior is running
	pub fn running(&self) -> bool;

	//immediately stop behavior
	pub fn interupt(&self);
}

pub struct pause {
	timer: f32
}