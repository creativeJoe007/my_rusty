// add an antonation here to make this struct printable in debug mode
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is: {} square pixels",
        area(width1, height1)
    );

    // refactoring to use tuple
    let rect = (30, 50);
    println!(
        "The area of the rectangle is: {} square pixels",
        area_from_tuple(rect)
    );

    // Using struct
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is: {} square pixels",
        area_from_struct(&rect1)
    );

    println!("rect1 is {:#?}", rect1);
}

fn area(width: i32, height: i32) -> i32 {
    width * height
}

fn area_from_tuple(dimension: (i32, i32)) -> i32 {
    dimension.0 * dimension.1
}

fn area_from_struct(dimension: &Rectangle) -> u32 {
    dimension.width * dimension.height
}
