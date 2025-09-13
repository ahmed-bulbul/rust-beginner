pub fn calculate(num1: f64, num2: f64, op: &str) -> Option<f64> {
    match op {
        "+" => Some(num1 + num2),
        "-" => Some(num1 - num2),
        "*" => Some(num1 * num2),
        "/" => {
            if num2 != 0.0 {
                Some(num1 / num2)
            } else {
                None
            }
        }
        _ => None,
    }
}
