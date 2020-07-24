fn main() {
    /*
    Immutable Reference
    So, one can use a variable in a function without having to transfer ownership to that function
    One way to do is by passing reference
    */
    let s1 = String::from("EYYOO");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    /*
    Mutable Reference
    We can mutate a reference by first adding the mut keyword
    Then we pass our reference to the function using the mut keyword as well
    We also expect this reference with a mut keyword in the receiving function
    */
    let mut s2 = String::from("Hello");
    change(&mut s2);

    /*
    This would cause an error as two variables cannot mutate a reference
    Within the same scope, and I guessing this is to avoid data racing

    Data race can be caused by:
    1. Two or more pointers are trying to access same data at the same time
    2. At least one of the pointers is being used to write to data
    3. There’s no mechanism being used to synchronize access to the data.

    A solution to this error is to wrap one of them in a curly braces as it would be confined
    TO just that scope, see c0
    */
    let mut mutable = String::from("Yoo");
    {
        let c0 = &mut mutable;
    }
    let c1 = &mut mutable;
    let c2 = &mut mutable;

    println!("{} and {} ", c1, c2);

    /*
    Also, you cannot borrow as mutable and immutable at the same time
    Because we can't afford to have data change while the immutable reference is still in use
    */
    let mut str = String::from("Mee");
    let p1 = &str;
    let p2 = &str;
    // This code would only work if I use p1 and p2 then the ref is deleted
    println!("{} and {}", p1, p2);
    let p3 = &mut str;
    println!("{}", p3);

    /*
    Dangling Pointer/Reference
    This can be created when a pointer that references to a particular memory is given to someone else
    by freeing some memory while preserving the pointer. Rust prevents this on compile time.

    Sample of a dangling reference
    */
    let reference_to_string = dangle();
}

fn dangle() -> String {
    let s = String::from("Hello");
    s
}

// References passed into functions is called borrowing
fn calculate_length(some_string: &String) -> usize {
    some_string.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
