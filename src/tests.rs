#[cfg(test)]
mod tests {
    use crate::math;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn math_function_evaluation() {
        let input_function = String::from("x^3");
        let x = 3.0;
        assert_eq!(math::f(&input_function, x), 27.0);
    }

    #[test]
    fn math_rectangles_integration() {
        let input_function = String::from("x^3");
        let start_value = 0.0;
        let stop_value = 1.0;
        let accuracy = 0.001;
        let result =
            math::rectangles_integration(&input_function, start_value, stop_value, accuracy);
        assert_approx_eq!(result, 0.250, 1e-3);
    }

    #[test]
    fn math_trapezoid_integration() {
        let input_function = String::from("x^3");
        let start_value = 0.0;
        let stop_value = 1.0;
        let accuracy = 0.001;
        let result =
            math::trapezoid_integration(&input_function, start_value, stop_value, accuracy);
        assert_approx_eq!(result, 0.250, 1e-3);
    }

    #[test]
    fn math_parabola_integration() {
        let input_function = String::from("x^3");
        let start_value = 0.0;
        let stop_value = 1.0;
        let accuracy = 0.001;
        let result = math::parabola_integration(&input_function, start_value, stop_value, accuracy);
        assert_approx_eq!(result, 0.250, 1e-3);
    }
}
