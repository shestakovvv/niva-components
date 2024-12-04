use num::traits::NumOps;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct QuadCalibrationData<T> {
    pub quad_gain: T,
    pub gain: T,
    pub bias: T
}

impl<T> QuadCalibrationData<T> 
where 
    T: NumOps + Copy
{
    pub fn calibrate(&self, value: T) -> T {
        self.quad_gain * value * value + self.gain * value + self.bias
    }
}

impl Default for QuadCalibrationData<f32> {
    fn default() -> Self {
        Self { quad_gain: 0.0, gain: 1.0, bias: 0.0 }
    }
}


pub struct LinearCalibrationData<T> {
    pub gain: T,
    pub bias: T
}

impl<T> LinearCalibrationData<T> 
where 
    T: NumOps + Copy
{
    pub fn calibrate(&self, value: T) -> T {
        self.gain * value + self.bias
    }
}

impl Default for LinearCalibrationData<f32> {
    fn default() -> Self {
        Self { gain: 1.0, bias: 0.0 }
    }
}

