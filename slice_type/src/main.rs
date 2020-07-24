fn main() {
    /*
    Slice type
    Slices let you reference a contiguous sequence of elements in a collection/string rather than the whole collection
    */
    let mut s = String::from("Hello, world!");
    let first_word = first_word_position(&s);

    /*
    Should we try to value of first_word with s to extract the first word
    It would result in a bug because s is now empty
    */
    s.clear();

    /*
        A solution to this problem is Rust String slices
    */
    let hello = &s[0..5];
    let hello_ = &s[..5]; // same with hello just more idiomatic
    let world = &s[6..11];

    // Or
    let world = String::from("world");
    let c1 = &world[3..world.len()];
    let c2 = &world[3..];

    // Or
    let t1 = &world[0..world.len()];
    let t2 = &world[..];

    // DO you know that string literals are really slices
    let lit = "Hello World"; // type is &str
}

fn first_word_position(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return index;
        }
    }
    s.len()
}

fn rewrite_first_word_position(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
