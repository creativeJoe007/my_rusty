fn main() {
    // Convert celcius to fahrenheit
    let converted_celc = convert_to_fahrenheit(32);
    println!("Converted Celcius: {}\n", converted_celc);

    // Fibonacci algo
    solve_fibonacci_number(5);
}

fn convert_to_fahrenheit(celc: i32) -> i32 {
    return celc + 32;
}

fn solve_fibonacci_number(position: i32) {
    let mut previous_number = 0;
    let mut current_number = 1;
    let mut new_number: i32;

    for _ in 0..position {
        new_number = current_number + previous_number;
        previous_number = current_number;
        current_number = new_number;
    }
    println!("The {}th position of a fibinacci number is: {}", position, current_number);
}
