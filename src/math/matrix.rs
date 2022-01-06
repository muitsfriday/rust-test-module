pub fn dot(v1: Vec<f32>, v2: Vec<f32>) -> Result<f32, String> {
    if v1.len() != v2.len() {
        return Result::Err("Invalid Dimension".to_string());
    }

    Result::Ok(0 as f32)
}
