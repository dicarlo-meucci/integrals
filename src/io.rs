use crate::{math, Algorithm};
use std::io::Write;
use textplots::{Chart, Plot, Shape};

pub fn get_string(message: &str, buffer: &mut String) {
    print!("{}", message);
    std::io::stdout().flush().expect("Failed to flush buffer");
    std::io::stdin().read_line(buffer).expect("Invalid input");
    *buffer = buffer.trim().to_string();
}

pub fn get_number(message: &str, buffer: &mut f64) {
    loop {
        let mut input = String::new();

        print!("{}", message);
        std::io::stdout().flush().unwrap();
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

pub fn validate_algo(algo: i32) -> Option<Algorithm> {
    match algo {
        1 => Some(Algorithm::Rectangles),
        2 => Some(Algorithm::Trapezoids),
        3 => Some(Algorithm::Parabolas),
        _ => None,
    }
}

pub fn get_algo() -> Option<Algorithm> {
    loop {
        println!();
        println!("[1] Rectangles (traditional)");
        println!("[2] Trapezoids (more accurate)");
        println!("[3] Parabolas (best)");
        print!("algorithm = ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Invalid input");

        if let Ok(choice) = input.trim().parse() {
            if (1..=3).contains(&choice) {
                return match choice {
                    1 => Some(Algorithm::Rectangles),
                    2 => Some(Algorithm::Trapezoids),
                    3 => Some(Algorithm::Parabolas),
                    _ => panic!("Invalid algorithm"),
                };
            }
        }

        println!("Invalid algorithm DEBUGGING");
    }
}

pub fn clear_console() {
    print!("\x1Bc")
}

pub fn print_header() {
    println!("  _   _ _   _ __  __ _____ ____  ___ ____    _    _       ___ _   _ _____ _____ ____ ____      _  _____ ___ ___  _   _ ");
    println!(" | \\ | | | | |  \\/  | ____|  _ \\|_ _/ ___|  / \\  | |     |_ _| \\ | |_   _| ____/ ___|  _ \\    / \\|_   _|_ _/ _ \\| \\ | |");
    println!(" |  \\| | | | | |\\/| |  _| | |_) || | |     / _ \\ | |      | ||  \\| | | | |  _|| |  _| |_) |  / _ \\ | |  | | | | |  \\| |");
    println!(" | |\\  | |_| | |  | | |___|  _ < | | |___ / ___ \\| |___   | || |\\  | | | | |__| |_| |  _ <  / ___ \\| |  | | |_| | |\\  |");
    println!(" |_| \\_|\\___/|_|  |_|_____|_| \\_\\___\\____/_/   \\_\\_____| |___|_| \\_| |_| |_____\\____|_| \\_\\/_/   \\_\\_| |___\\___/|_| \\_|");
    println!("\n")
}

pub fn pretty_print_integral(function: &String, start_value: f64, stop_value: f64) {
    println!("──────────────────────── COMPUTING ────────────────────────");
    println!("                         {}", stop_value);
    println!("                        ∫ {} dx", function);
    println!("                         {}", start_value);
}

pub fn print_algorithm(algorithm: &Option<Algorithm>) {
    println!("──────────────────────── ALGORITHM ────────────────────────");
    println!();
    println!(
        "                         {}",
        match algorithm {
            Some(Algorithm::Rectangles) => "Rectangles",
            Some(Algorithm::Trapezoids) => "Trapezoids",
            Some(Algorithm::Parabolas) => "Parabolas",
            None => "Unknown",
        }
    );
    println!();
}

pub fn print_graph(function: &String) {
    let eval_function = move |x: f32| math::f(&function, x as f64) as f32;
    println!("─────────────────────────── GRAPH ──────────────────────────");
    Chart::default()
        .lineplot(&Shape::Continuous(Box::new(eval_function)))
        .display();
}

pub fn pretty_print_result(result: f64, decimals: usize) {
    println!("───────────────────────── RESULT ─────────────────────────");
    println!("                         {:.decimals$}", result);
    println!("──────────────────────────────────────────────────────────");
}
