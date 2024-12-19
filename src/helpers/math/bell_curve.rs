pub fn bell_curve(x: f32, center: f32, std_dev: f32, height: f32) -> f32 {
    let x = x - center;
    let x = x * x;
    let x = x / (std_dev * std_dev);
    let x = -x;
    let x = x.exp();
    let x = x * height;
    return x;
}

pub fn bell_curve_quad(x: f32, center: f32, std_dev: f32, height: f32) -> f32 {
    let x = x - center;
    let x = x.powi(4);
    let x = x / std_dev.powi(4);
    let x = -x;
    let x = x.exp();
    let x = x * height;
    return x;
}
