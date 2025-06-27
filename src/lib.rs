use std::f64::consts::PI;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        let sin: f64 = sin(90);
        let cos: f64 = cos(0);
        let tan: f64 = tan(45);
        
        println!("sin(90) = {}", sin);
        println!("cos(0) = {}", cos);
        println!("tan(60) = {}", tan);
    }
}

