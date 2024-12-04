/// Basic implementation of Kalman filter
pub struct Kalman {
    estimate: f32,
    error_covariance: f32,
    process_noise: f32,
    measurement_noise: f32,
}

impl Kalman {
    /// If your ADC measurements are very noisy:
    /// Use a larger R (e.g., 0.1 to 1.0) and a smaller Q (e.g., 0.01).
    ///
    /// If your ADC measurements are stable:
    /// Use a smaller R (e.g., 0.001 to 0.01) and a larger Q (e.g., 0.1).
    #[allow(unused)]
    pub fn new(process_noise: f32, measurement_noise: f32) -> Self {
        Self {
            estimate: 0.0,
            error_covariance: 1.0,
            process_noise,
            measurement_noise,
        }
    }

    /// Call this method in loop for get calibrated value
    #[allow(unused)]
    pub fn update(&mut self, measurement: f32) -> f32 {
        let kalman_gain = self.error_covariance / (self.error_covariance + self.measurement_noise);
        self.estimate += kalman_gain * (measurement - self.estimate);
        self.error_covariance = (1.0 - kalman_gain) * self.error_covariance + self.process_noise;
        self.estimate
    }
}
