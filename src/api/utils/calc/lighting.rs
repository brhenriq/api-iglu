pub fn lighting_calc(area: f64) -> f64 {
    0.86 * 100.00 + (((area - 6.00) / 4.00) * 60.00) * 1.2
}
