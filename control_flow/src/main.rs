fn main() {
    let number = 8;
    let condition = true;
    let y = if condition { 4 } else { 7 };
    println!("Hello, world! see y {}", y);

    // if operations
    if number % 5 == 0 {
        println!("number is divisible by 5");
    } else if number % 4 == 0 {
        println!("number is divisible by 4");
    } else {
        println!("Conditions isn't divisible");
    }

    /*
        IT'S TIME FOR SOME LOOPS
    */
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 3;
        }
    };

    println!("\nResult is: {}", result);

    // While loop
    let mut countdown = 3;

    while countdown != 0 {
        println!("We are counting down in {} ", countdown);
        countdown -= 1;
    }
    println!("LIST OF\n");

    // Looping a collection
    let collection = [10,20,30,40,50];
    let mut index = 0;

    while index < collection.len() {
        println!("{}! ", collection[index]);
        index += 1;
    }

    // Making use of For loop
    for number in collection.iter() {
        println!("The value is: {}", number);
    }

    // Using For loop to loop a range
    for number in (1..4).rev() {
        println!("Reversed number is: {}", number);
    }
}
