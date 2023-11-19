pub fn peoples(activity: String, quantity: i64) -> f64 {
    let power = match activity.as_str() {
        "low" => 80.0,
        "medium" => 150.0,
        "high" => 500.0,
        _ => 0.0,
    };

    power * 0.86 * quantity as f64
}
