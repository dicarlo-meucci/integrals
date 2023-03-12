use meval::eval_str;

pub fn format_function(x: &str, y: f64) -> String {
    x.replace("x", &y.to_string())
}

pub fn f(function_to_eval: &String) -> f64 {
    eval_str(function_to_eval).unwrap()
}