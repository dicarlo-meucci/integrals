use meval::eval_str;
mod io;

fn format_function(x: String, y: f64) -> String {
    x.replace("x", &y.to_string())
}

fn f(function_to_eval: &str) -> f64 {
    eval_str(function_to_eval).unwrap()
}

fn main() {
    let mut input_function = String::new();
    let mut stop_value: f64 = Default::default();
    let mut start_value: f64 = Default::default();
    let mut accuracy: f64 = 0.01;

    println!("Integration algorithm by iQuickDev");
    io::get_string("f(x) = ", &mut input_function);
    io::get_number("a = ", &mut start_value);
    io::get_number("b = ", &mut stop_value);
    io::get_number("accuracy = ", &mut accuracy);

    print!("\x1B[2J\x1B[1;1H");
    
    io::pretty_print_integral(&input_function, start_value, stop_value);

    // fix variable names

    let steps = (stop_value - start_value) / accuracy;
    let mut index = 0.0;
    let mut area = 0.0;

    while index <= steps {
        let mut input_fn_copy = input_function.clone();
        input_fn_copy = format_function(input_fn_copy, start_value + index * accuracy);

        area = area + accuracy * f(&input_fn_copy);
        index += 1.0;
    }

    io::pretty_print_result(area);
}
