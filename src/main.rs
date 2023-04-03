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
    let mut config = Config::parse();

    println!("{:?}", config);

    let mut input_function = String::new();
    let mut upper_limit: f64 = 0.0;
    let mut lower_limit: f64 = 0.0;
    let mut precision: f64 = 0.0;
    let algorithm: Option<Algorithm>;
    let decimals: usize = config.decimals.unwrap_or(6);

    io::print_header();
    if config.function.is_none() {
        io::get_string("f(x) = ", &mut input_function);
    } else {
        input_function = config.function.take().unwrap();
    }

    if config.lower.is_none() {
        io::get_number("a = ", &mut lower_limit);
    } else {
        upper_limit = config.upper.unwrap();
    }

    if config.upper.is_none() {
        io::get_number("b = ", &mut upper_limit);
    } else {
        lower_limit = config.lower.unwrap();
    }

    if config.precision.is_none() {
        io::get_number("precision = ", &mut precision);
    } else {
        precision = config.precision.unwrap();
    }

    if config.algorithm.is_none() {
        algorithm = io::get_algo()
    } else {
        algorithm = validate_algo(config.algorithm.unwrap());
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
