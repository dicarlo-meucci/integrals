#[cfg(test)]
mod tests {
    use crate::math;
    
    #[test]
    fn math_format_function() {
        let input_function = String::from("x^2 + 2*x + 1");
        let x_value = 2.0;
        let formatted_function = math::format_function(&input_function, x_value);
        assert_eq!(formatted_function, String::from("2^2 + 2*2 + 1"));
    }

    #[test]
    fn integration_algorithm() {
        let input_function = String::from("x^3");
        let start_value = 0.0;
        let stop_value = 1.0;
        let accuracy = 0.001;

        let mut index = 0.0;
        let mut area = 0.0;

        while index <= (stop_value - start_value) / accuracy {
            let input_fn_copy = math::format_function(&input_function, start_value + index * accuracy);
            area = area + accuracy * math::f(&input_fn_copy);
            index += 1.0;
        }
        
        let area = format!("{:.2}", area).parse::<f64>().unwrap();
        
        assert_eq!(area, 0.25);
    }
}