use std::f64::consts::PI;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        let value = tan(60);
        println!("tan(60) = {}", value);
    }
}

pub fn _sin<T: Into<f64>>(_x: T) -> f64 {
    0.0
}

pub fn _cos<T: Into<f64>>(_x: T) -> f64 {
    0.0
}

pub fn tan<T: Into<f64>>(x: T) -> f64 {
    let y: f64 = x.into() * (PI / 180.0);
    let siny: f64 = y - (y.powi(3) / 6.0) + (y.powi(5) / 120.0) - (y.powi(7) / 5040.0);
    let cosy: f64 = 1.0 - (y.powi(2) / 2.0) + (y.powi(4) / 24.0) - (y.powi(6) / 720.0);
    let tanx: f64 =  siny / cosy;
    tanx
}