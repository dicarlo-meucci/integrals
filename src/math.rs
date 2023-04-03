use meval::eval_str;
//use rayon::prelude::*;

pub fn f(function: &str, x: f64) -> f64 {
    let function_to_eval = function.replace("x", &x.to_string());
    eval_str(&function_to_eval).unwrap()
}

// pub fn multithreaded_rectangles_integration<F>(
//     input_function: F,
//     lower_limit: f64,
//     upper_limit: f64,
//     precision: f64,
// ) -> f64
// where
//     F: Fn(f64) -> f64 + Sync,
// {
//     let steps = ((upper_limit - lower_limit) / precision).ceil() as usize;
//     let x_values: Vec<f64> = (0..steps)
//         .into_par_iter()
//         .map(|i| lower_limit + i as f64 * precision)
//         .collect();

//     let area = x_values
//         .into_par_iter()
//         .map(|x| input_function(x) * precision)
//         .sum::<f64>();

//     area
// }

pub fn rectangles_integration(
    input_function: &String,
    lower_limit: f64,
    upper_limit: f64,
    precision: f64,
) -> f64 {
    let steps = ((upper_limit - lower_limit) / precision) as usize;
    let mut area = 0.0;

    for i in 0..steps {
        area = area + precision * f(&input_function, lower_limit + i as f64 * precision);
    }

    area
}

pub fn trapezoid_integration(
    input_function: &String,
    lower_limit: f64,
    upper_limit: f64,
    precision: f64,
) -> f64 {
    let mut area = 0.0;
    let mut prev_y = f(&input_function, lower_limit);

    let steps = ((upper_limit - lower_limit) / precision).ceil() as usize;

    for i in 0..steps {
        let x = lower_limit + i as f64 * precision;
        let y = f(&input_function, x);
        area += (y + prev_y) * precision / 2.0;
        prev_y = y;
    }

    area
}

pub fn parabola_integration(
    input_function: &str,
    lower_limit: f64,
    upper_limit: f64,
    precision: f64,
) -> f64 {
    let steps = ((upper_limit - lower_limit) * precision.recip()).ceil() as usize;
    let mut area = f(input_function, lower_limit) + f(input_function, upper_limit);

    for i in 0..steps {
        let x = lower_limit + (i as f64 + 0.5) * (upper_limit - lower_limit) / steps as f64;
        if i % 2 == 0 {
            area += 2.0 * f(input_function, x);
        } else {
            area += 4.0 * f(input_function, x);
        }
    }

    area * (upper_limit - lower_limit) / (3.0 * steps as f64)
}
