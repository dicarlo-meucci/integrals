use crate::{math, Algorithm};
use std::{io::Write, time::Duration};
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
        4 => Some(Algorithm::MultithreadedRectangles),
        5 => Some(Algorithm::MultithreadedTrapezoids),
        6 => Some(Algorithm::MultithreadedParabolas),
        _ => None,
    }
}

pub fn get_algo() -> Option<Algorithm> {
    loop {
        println!();
        println!("[1] Rectangles (traditional)");
        println!("[2] Trapezoids (more accurate)");
        println!("[3] Parabolas (best)");
        println!("[4] Multithreaded Rectangles");
        println!("[5] Multithreaded Trapezoids");
        println!("[6] Multithreaded Parabolas");
        print!("algorithm = ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Invalid input");

        if let Ok(choice) = input.trim().parse() {
            if (1..=6).contains(&choice) {
                return match choice {
                    1 => Some(Algorithm::Rectangles),
                    2 => Some(Algorithm::Trapezoids),
                    3 => Some(Algorithm::Parabolas),
                    4 => Some(Algorithm::MultithreadedRectangles),
                    5 => Some(Algorithm::MultithreadedTrapezoids),
                    6 => Some(Algorithm::MultithreadedParabolas),
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
    println!("──────────────────────── COMPUTING ────────────────────────\n");
    println!("{:^56}", stop_value);
    println!("{:^60}", format!("∫ {} dx", function));
    println!("{:^56}", start_value);
    println!();
}

pub fn print_algorithm(algorithm: &Option<Algorithm>) {
    println!("──────────────────────── ALGORITHM ────────────────────────\n");
    println!(
        "{:^60}",
        match algorithm {
            Some(Algorithm::Rectangles) => "Rectangles",
            Some(Algorithm::Trapezoids) => "Trapezoids",
            Some(Algorithm::Parabolas) => "Parabolas",
            Some(Algorithm::MultithreadedRectangles) => "Rectangles (Multithreaded)",
            Some(Algorithm::MultithreadedTrapezoids) => "Trapezoids (Multithreaded)",
            Some(Algorithm::MultithreadedParabolas) => "Parabolas (Multithreaded)",
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
    println!(
        "{:^58}",
        format!("{:.decimals$}", result, decimals = decimals)
    );
    println!("──────────────────────────────────────────────────────────");
}

pub fn pretty_print_time_elapsed(duration: Duration) {
    println!("────────────────────────── TIME ──────────────────────────");
    println!("{:^60}", format!("{:?}", duration));
    println!("──────────────────────────────────────────────────────────");
}
