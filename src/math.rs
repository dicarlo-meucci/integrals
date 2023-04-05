use meval::eval_str;
use std::thread;
//use rayon::prelude::*;

pub fn f(function: &str, x: f64) -> f64 {
    let function_to_eval = function.replace("x", &x.to_string());
    eval_str(&function_to_eval).unwrap()
}

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

pub fn multithreaded_rectangles_integration(
    input_function: &String,
    lower_limit: f64,
    upper_limit: f64,
    precision: f64,
) -> f64 {
    let steps = ((upper_limit - lower_limit) / precision) as usize;
    let num_threads = num_cpus::get(); // get the number of available CPU cores
    let step_size = steps / num_threads; // divide the work evenly among threads
    let mut threads = vec![]; // vector to store the threads

    for i in 0..num_threads {
        let input_function = input_function.clone();
        let lower_limit = lower_limit + (i * step_size) as f64 * precision;
        let precision = precision;
        let thread = thread::spawn(move || {
            let mut area = 0.0;
            for i in 0..step_size {
                area += precision * f(&input_function, lower_limit + i as f64 * precision);
            }
            area
        });
        threads.push(thread);
    }

    let mut area = 0.0;
    for thread in threads {
        area += thread.join().unwrap();
    }

    area
}

pub fn multithreaded_trapezoid_integration(
    input_function: &String,
    lower_limit: f64,
    upper_limit: f64,
    precision: f64,
) -> f64 {
    let mut area = 0.0;

    let steps = ((upper_limit - lower_limit) / precision).ceil() as usize;
    let num_threads = num_cpus::get(); // get the number of available CPU cores
    let step_size = steps / num_threads; // divide the work evenly among threads
    let mut threads = vec![]; // vector to store the threads

    for i in 0..num_threads {
        let input_function = input_function.clone();
        let lower_limit = lower_limit + (i * step_size) as f64 * precision;
        let precision = precision;
        let thread = thread::spawn(move || {
            let mut area = 0.0;
            let mut prev_y = f(&input_function, lower_limit);
            for i in 0..step_size {
                let x = lower_limit + i as f64 * precision;
                let y = f(&input_function, x);
                area += (y + prev_y) * precision / 2.0;
                prev_y = y;
            }
            area
        });
        threads.push(thread);
    }

    for thread in threads {
        area += thread.join().unwrap();
    }

    area
}

pub fn multithreaded_parabola_integration(
    input_function: &str,
    lower_limit: f64,
    upper_limit: f64,
    precision: f64,
) -> f64 {
    let num_threads = num_cpus::get(); // get the number of available CPU cores
    let mut threads = vec![]; // vector to store the threads
    let mut partial_areas = vec![0.0; num_threads]; // vector to store the partial areas
    let mut subintervals = vec![]; // vector to store the subintervals

    // divide the integration interval into subintervals
    let steps = ((upper_limit - lower_limit) * precision.recip()).ceil() as usize;
    let subinterval_size = steps / num_threads;
    let mut start = lower_limit;
    for _ in 0..num_threads {
        let end = start + subinterval_size as f64 * (upper_limit - lower_limit) / steps as f64;
        subintervals.push((start, end));
        start = end;
    }
    subintervals[num_threads - 1].1 = upper_limit; // adjust the last subinterval

    // spawn a thread for each subinterval
    for i in 0..num_threads {
        let input_function = input_function.to_string();
        let (lower_limit, upper_limit) = subintervals[i];
        let thread = thread::spawn(move || {
            let mut area = f(&input_function, lower_limit) + f(&input_function, upper_limit);
            let substeps = ((upper_limit - lower_limit) * precision.recip()).ceil() as usize;
            for j in 0..substeps {
                let x =
                    lower_limit + (j as f64 + 0.5) * (upper_limit - lower_limit) / substeps as f64;
                if j % 2 == 0 {
                    area += 2.0 * f(&input_function, x);
                } else {
                    area += 4.0 * f(&input_function, x);
                }
            }
            area * (upper_limit - lower_limit) / (3.0 * substeps as f64)
        });
        threads.push(thread);
    }

    // collect the partial results from each thread
    for (i, thread) in threads.into_iter().enumerate() {
        partial_areas[i] = thread.join().unwrap();
    }

    // combine the partial results to obtain the final estimate of the integral
    partial_areas.iter().sum()
}
