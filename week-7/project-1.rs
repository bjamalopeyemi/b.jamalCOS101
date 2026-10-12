
use std::f64::consts::PI;
use std::io;

// Reads a finite, positive number.
fn read_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("No input received. Exiting.");
                std::process::exit(0);
            }
            Ok(_) => {}
            Err(_) => {
                println!("Could not read input. Please try again.");
                continue;
            }
        }

        match input.trim().parse::<f64>() {
            Ok(number) if number.is_finite() && number > 0.0 => {
                return number;
            }
            _ => {
                println!(
                    "Please enter a finite number greater than zero."
                );
            }
        }
    }
}

fn trapezium_area() -> f64 {
    let height = read_number("Enter the height:");
    let base1 = read_number("Enter base 1:");
    let base2 = read_number("Enter base 2:");

    (height / 2.0) * (base1 + base2)
}

fn rhombus_area() -> f64 {
    let diagonal1 = read_number("Enter diagonal 1:");
    let diagonal2 = read_number("Enter diagonal 2:");

    0.5 * diagonal1 * diagonal2
}

fn parallelogram_area() -> f64 {
    let base = read_number("Enter the base:");
    let altitude = read_number("Enter the altitude:");

    base * altitude
}

fn cube_surface_area() -> f64 {
    let side = read_number("Enter the side length:");

    6.0 * side * side
}

fn cylinder_volume() -> f64 {
    let radius = read_number("Enter the radius:");
    let height = read_number("Enter the height:");

    PI * radius * radius * height
}

// Displays the result only if it is finite.
fn show_result(description: &str, result: f64) {
    if result.is_finite() {
        println!("{}: {:.2}", description, result);
    } else {
        println!("The result is too large to represent.");
    }
}

fn show_menu() {
    println!();
    println!("=== THE SHAPE CALCULATOR ===");
    println!("1. Trapezium (area)");
    println!("2. Rhombus (area)");
    println!("3. Parallelogram (area)");
    println!("4. Cube (surface area)");
    println!("5. Cylinder (volume)");
    println!("6. Quit");
    println!("Choose an option (1-6):");
}

fn main() {
    println!("Welcome to the MTH 101 Shape Calculator!");

    loop {
        show_menu();

        let mut choice = String::new();

        match io::stdin().read_line(&mut choice) {
            Ok(0) => {
                println!("No input received. Goodbye!");
                break;
            }
            Ok(_) => {}
            Err(_) => {
                println!("Could not read input. Please try again.");
                continue;
            }
        }

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please type a number from 1 to 6.");
                continue;
            }
        };

        match choice {
            1 => show_result(
                "The area of the trapezium",
                trapezium_area(),
            ),
            2 => show_result(
                "The area of the rhombus",
                rhombus_area(),
            ),
            3 => show_result(
                "The area of the parallelogram",
                parallelogram_area(),
            ),
            4 => show_result(
                "The surface area of the cube",
                cube_surface_area(),
            ),
            5 => show_result(
                "The volume of the cylinder",
                cylinder_volume(),
            ),
            6 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Option not recognised. Please choose 1 to 6."),
        }
    }
}