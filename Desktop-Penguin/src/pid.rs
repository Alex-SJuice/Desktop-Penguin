pub struct PidController {
    k_p: f32,
    k_i: f32,
    k_d: f32,

    previous_error: f32,
    accumulated_error: f32,
}

impl PidController {
    pub fn new(k_p: f32, k_i: f32, k_d: f32) -> Self {
        PidController {
            k_p,
            k_i,
            k_d,

            previous_error: 0.0,
            accumulated_error: 0.0,
        }
    }

    pub fn update(&mut self, error: f32) -> f32 {
        let difference = self.previous_error - error;
        self.previous_error = error;
        self.accumulated_error += error;

        self.k_p * error + self.k_i * self.accumulated_error - self.k_d * difference
    }
}
