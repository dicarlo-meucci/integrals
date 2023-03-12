use crate::math;
use std::io::{Write, self};
use textplots::{Chart, Plot, Shape};

pub fn get_string(message: &str, buffer: &mut String) {
    print!("{}", message);
    std::io::stdout().flush().expect("Failed to flush buffer");
    std::io::stdin().read_line(buffer).expect("Invalid input");
    *buffer = buffer.trim().to_string();
}

pub fn get_number(message: &str, buffer: &mut f64) {
    loop {
        let mut input = Default::default();

        print!("{}", message);
        std::io::stdout().flush().expect("Failed to flush buffer");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Invalid input");

        match input.trim().parse::<f64>() {
            Ok(v) => {
                *buffer = v;
                break;
            }
            Err(_) => {
                println!("Not a number");
                continue;
            }
        }
    }
}

pub fn pretty_print_integral(function: &String, start_value: f64, stop_value: f64) {
    println!("─────────────────────── COMPUTING ───────────────────────");
    println!("                         {}", stop_value);
    println!("                        ∫ {} dx", function);
    println!("                         {}", start_value);
}

pub fn pretty_print_result(result: f64) {
    println!("──────────────────────── RESULT ────────────────────────");
    println!("                         {:.2}", result);
    println!("────────────────────────────────────────────────────────");
}

pub fn print_graph(function: &String) {
    let eval_function = move |x: f32| math::f(&math::format_function(function, x as f64)) as f32;
    println!("───────────────────────── GRAPH ────────────────────────");
    Chart::new(100,100,-1.0,1.0)
        .lineplot(&Shape::Continuous(Box::new(eval_function)))
        .display();
}