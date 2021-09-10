

/// Returns a Vector of values corresponding to [start : end : step]
pub fn range_step(start: f32, end: f32, step: f32) -> Vec<f32> {
    // TODO: assert non-negative, step is a factor of end or something like that

    let mut result: Vec<f32> = Vec::new();
    let mut count = start;
    while count < end {
        result.push(count);
        count += step;
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_step() {
        let range = range_step(0.0, 1.0, 0.1);

        assert_eq!(range.len(), 10);
        assert_eq!(range.iter().sum::<f32>(), 4.5);
    }
}
