fn main() {
    let p = return_function();
    println!("Hello, world!, P is {}", p);

    another_function(3);
    an_expression_function();
}

fn another_function(x: i32) {
    println!("Eyoo x is: {}", x);
}

fn an_expression_function() {
    let x = 5;
    // an expression
    let y = {
        let x = 3;
        x + 1
    };

    println!("X is {}", x);
    println!("Y is {}", y);
}

fn return_function() -> i32 {
    // lets return something
    let y = {
        let x = 3;
        x + 1
    };

    y
}
