/*
Stack memory stores data in order/sequence and pops in likewise manner
Heap memory is for data with unknown size, so it stores randomly and can be memory extensive
As every update requires the processor to search for space on the memory to store that peace of information
*/

fn main() {
    /*
    String literals are immutable
    For some reason this one could be updated
    Why can String be mutated but not literals
    */

    let mut c = String::from("Hello");
    c.push_str(", world");
    println!("{}", c);

    // Literals which are immutable
    // adding mut throws #[warn(unused_mut)] which means the compiler skips the mut command
    let s = "Hello, world";
    println!("{}", s);

    /*
    Literals are fixed and known at compile time, so the compiler knows the amount of memory needed
    Compiler then allocate needed space to the string literal on the stack memory, which makes it efficient and fast
    Unlike the String counter-part that can change it's data at any form, which means more data maybe allocated
    And this means data has to be stored in the heap memory.

    When a String looses its scope, it gets dropped. Once a curly brace is closed, all variables within that scope
    is dropped by a Rust command called drop
    */

    /*
    THis stores 5 in x, this is easier to do because the memory to allocate for scalar types are known on compilation
    So a copy of X value is made and assigned to Y
    */
    let x = 5;
    let _y = x;

    // THis is different for Strings
    /*
    String literals basially copies just the heap data but still allow c1 and c2 reference to their heap memory
    */
    let c1 = "Hello, world";
    let _c2 = c1;

    println!("C1: {}", c1);

    /*
    When we copy data from one String var to another, we are basically copying the stack data and duplicate it on the system still pointing to same heap data.
    When we copy a String variable, Rust removes the initially declared variable to avoid double free error, which means the variable is out of scope and destroyed but not it's heap data
    */
    let s1 = String::from("Hello, world");
    let s2 = s1;

    println!("S1: {}", s1);

    // Since Rust doesn't automatically do deep copy (copy heap and stack data), we can tell Rust to. Though this can be expensive
    // Using the clone method
    let s_1 = String::from("Hello, world");
    let s_2 = s_1.clone();
    println!("S1: {}, S2: {}", s_1, s_2);

    // COpy and Ownership
    let str = String::from("Hello");
    takes_ownership(str);
    println!("STR: {}", str);

    let xx = 5;
    makes_copy(xx);
    println!("XX: {}", xx);

    // Returns a Tuple
    let o = String::from("Hello");
    let (n, l) = calculate_length(o);
    println!("String is {}, length is {}", o, n);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // gets dropped, memory is freed

fn makes_copy(digit: i32) { // digit comes into scope
    println!("{}", digit);
} // Integers just goes out of scope, nothing special happens as this was just a copy

fn calculate_length(s: String) -> (String,usize) {
    let length s.len();

    (s, length)
}
