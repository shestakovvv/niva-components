mod kalman;
pub use kalman::Kalman;

mod other;
pub use other::{
    moving_average_filter,
    low_pass_filter,
    median_filter
};