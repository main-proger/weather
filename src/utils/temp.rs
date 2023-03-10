pub fn ctof(t: f64) -> f64 {
    (t * 9f64 / 5f64) + 32f64
}
pub fn ctok(t: f64) -> f64 {
    t + 273.15f64
}

pub fn ftoc(t: f64) -> f64 {
    (t - 32f64) * 5f64 / 9f64
}
pub fn ftok(t: f64) -> f64 {
    (t - 32f64) * 5f64 / 9f64 + 273.15f64
}

pub fn ktof(t: f64) -> f64 {
    (t - 273.15f64) * 9f64 / 5f64 + 32f64
}
pub fn ktoc(t: f64) -> f64 {
    t - 273.15f64
}
