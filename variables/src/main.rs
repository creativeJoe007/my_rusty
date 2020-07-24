fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);

    // variable Shadowing
    /*  */
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("Y is: {}", y);

    /*
        Do you know that when shadowing a variable, you can change the data type of the variable
        But once you make a variable mutable, you cannot change the data type because unlike the previous
        Shaowing, a new variable is not created but mutated
    */
    // This would work
    let p = "   ";
    let p = p.len();
    println!("P is: {}", p);

    // This would fail because I am changing data type for a mutable variable
    // let mut l = "  ";
    // l = l.len();
    // println!("L is: {}", l);
}
