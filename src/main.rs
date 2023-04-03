use crate::io::validate_algo;
use clap::Parser;
use core::panic;
mod io;
mod math;
mod tests;

pub enum Algorithm {
    Rectangles,
    Trapezoids,
    Parabolas,
    //MultithreadedRectangles,
    //MultithreadedTrapezoids,
    //MultithreadedParabolas
}

#[derive(Parser, Default, Debug)]
struct Config {
    #[arg(short, long)]
    function: Option<String>,
    #[arg(short, long)]
    upper: Option<f64>,
    #[arg(short, long)]
    lower: Option<f64>,
    #[arg(short, long)]
    precision: Option<f64>,
    #[arg(short, long)]
    algorithm: Option<i32>,
    #[arg(short, long)]
    decimals: Option<usize>,
}

fn main() {
    let config = Config::parse();

    println!("{:?}", config);

    let mut input_function = config.function.unwrap_or(String::new());
    let mut upper_limit: f64 = config.upper.unwrap_or(f64::default());
    let mut lower_limit: f64 = config.lower.unwrap_or(f64::default());
    let mut precision: f64 = config.precision.unwrap_or(f64::default());
    let mut algorithm: Option<Algorithm> =
        validate_algo(config.algorithm.unwrap_or(i32::default()));
    let decimals: usize = config.decimals.unwrap_or(6);

    io::print_header();
    if config.algorithm.is_none() {
        io::get_string("f(x) = ", &mut input_function);
    }

    if config.lower.is_none() {
        io::get_number("a = ", &mut lower_limit);
    }

    if config.upper.is_none() {
        io::get_number("b = ", &mut upper_limit);
    }

    if config.precision.is_none() {
        io::get_number("precision = ", &mut precision);
    }

    if config.algorithm.is_none() {
        algorithm = io::get_algo()
    }

    io::clear_console();

    io::pretty_print_integral(&input_function, lower_limit, upper_limit);
    io::print_algorithm(&algorithm);
    io::print_graph(&input_function);

    let result = match algorithm {
        Some(Algorithm::Rectangles) => {
            math::rectangles_integration(&input_function, lower_limit, upper_limit, precision)
        }
        Some(Algorithm::Trapezoids) => {
            math::trapezoid_integration(&input_function, lower_limit, upper_limit, precision)
        }
        Some(Algorithm::Parabolas) => {
            math::parabola_integration(&input_function, lower_limit, upper_limit, precision)
        }
        None => {
            panic!("Invalid algorithm");
        }
    };

    io::pretty_print_result(result, decimals);
}
