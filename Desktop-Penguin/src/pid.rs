pub struct pid_controller {
	kP: f32,
	kI: f32,
	kD: f32,

	previous_error: f32,
	accumulated_error: f32
}

impl pid_controller {
	pub fn new(kP: f32, kI: f32, kD: f32) -> Self {
		pid_controller {
			kP,
			kI,
			kD,

			previous_error: 0.0,
			accumulated_error: 0.0,
		}
	}

	pub fn update(&mut self, error: f32) -> f32 {
		let difference = self.previous_error - error;
		self.previous_error = error;
		self.accumulated_error += error;

		self.kP * error + self.kI * self.accumulated_error - self.kD * difference
	}
}