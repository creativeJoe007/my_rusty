fn main() {
    let _guess: u32 = "46".parse().expect("Please input a number");

    // This would fail because we didn't infer a data type to it
    let _guess_: i32 = "46".parse().expect("Please input a number");

    /*
        Rust has what is called Scalar types and they are 4 in number
        1. Integer types, 2. Boolean, 3. Character, 4. floating point number
     */
     // Integer types can be i8,u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
     let x: u8 = b'A';
     let u_bits_16: u16 = 3_000;
     let i_bits_16: i16 = 3_000;
     let u_bits_32: u32 = 300_000;
     let i_bits_32: i32 = 300_000;

     println!("{}", x);
     println!("{}", u_bits_16);
     println!("{}", i_bits_16);
     println!("{}", u_bits_32);
     println!("{}", i_bits_32);

     // Floating types can either be f32 or f64 depending on the CPU bits
     let float = 2.00;
     let float_32: f32 = 3.0;
     println!("{}", float);
     println!("{}", float_32);

     // Arithmetic operation
     let add = 5 + 10;
     let subtract = 10.5 - 5.3;
     let product = 4 * 30;
     let quotient = 54.6 / 32.1;
     let remainder = 50 % 3;

     println!("{} {} {} {} {}", add, subtract, product, quotient, remainder);

     // Boolean
     let is_rust = true;
     let is_js: bool = false;

     // Character literals
     let c = 'Z';
     let cc = 'Ω';
     let heart_eyed_cat = '😻';

     // Rust supports 2 compound types, tuples and arrays
     // Tuple is like python, once declared cannot be changed but can support multiple types
     let tup: (i32, f64, i8) = (500, 6.5, 1);

     // we can get all items from the tup in their position using
     // this is called destructuring
     let (xx, yy, zz) = tup;
     println!("The value of y is: {}", yy);
     // we can also access itmes in tup based on their index using
     let xxx = tup.0;
     let yyy = tup.1;

     println!("The value of yyy is: {}", yyy);
     println!("The value of xxx is: {}", xxx);

     // Unlike tuple, in Array all items must be same type
     let sample_array = [1,2,3,4];
     // If you want to pass a type while still stating maximum amount of items in the array use
     let array_2: [i32; 5] = [1,2,3,4,5];
     // If we want to fill the array with same data, a concise way to do it would be:
     let array_3 = [3; 5]; // meaning put number 3 in 5 places
     // Indexing in Array is just like other langauages [indx]
}
