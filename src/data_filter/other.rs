//TODO: преобразовать к структурам, привести к общему методу filter или update

/// get average value from circular buffer 
#[allow(unused)]
pub fn moving_average_filter(buffer: &mut [f32; 10], new_value: f32) -> f32 {
    buffer.rotate_left(1);
    buffer[buffer.len() - 1] = new_value;
    buffer.iter().sum::<f32>() / buffer.len() as f32
}

/// get new value as percent of previous
#[allow(unused)]
pub fn low_pass_filter(previous: f32, new_value: f32, alpha: f32) -> f32 {
    alpha * new_value + (1.0 - alpha) * previous
}

/// not implemented
#[allow(unused)]
pub fn median_filter(buffer: &mut [f32; 10], new_value: f32) -> f32 {
    todo!();
    buffer.rotate_left(1);
    buffer[buffer.len() - 1] = new_value;

    // Create a mutable copy of the buffer for sorting
    let mut sorted = *buffer;
    // Sort the buffer
    // TODO: sorted.as_mut_slice().sort_by(|a, b| a.partial_cmp(b).unwrap());
    // Return the median (middle value)
    sorted[sorted.len() / 2]
}