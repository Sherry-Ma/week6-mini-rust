use std::io::{self, Write};

fn main() {
    let (base, height) = get_triangle_dimensions();

    let area = calculate_triangle_area(base, height);

    println!("The area of the triangle is: {}", area);
}

fn get_triangle_dimensions() -> (f32, f32) {
    let mut base_input = String::new();
    let mut height_input = String::new();

    print!("Enter the base of the triangle: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut base_input)
        .expect("Failed to read input");

    print!("Enter the height of the triangle: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut height_input)
        .expect("Failed to read input");

    let base = match base_input.trim().parse::<f32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for base. Please enter a valid number.");
            get_triangle_dimensions().0
        }
    };

    let height = match height_input.trim().parse::<f32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for height. Please enter a valid number.");
            get_triangle_dimensions().1
        }
    };

    (base, height)
}

fn calculate_triangle_area(base: f32, height: f32) -> f32 {
    0.5 * base * height
}
