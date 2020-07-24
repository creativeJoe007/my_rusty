#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// Implementation block
// A struct can have multiple imp
impl Rectangle {
    fn area(&self) -> u32 {
        // if we wanted to change contents in this struct we can mut keyword
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

impl Rectangle {
    // associate functions are functions that do not take in self as a param
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    /*
    Methods are almost like functions, except they are tied to the struct they are declared in
    And act like Python methods, taking self as first parameter
    */
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45
    };

    // associate functions are not called with the struct instance
    let sq = Rectangle::square(30);

    println!(
        "Rect can hold Rect2: {}",
        rect.can_hold(&rect2)
    );

    println!(
        "Rec1 can hold Rect3: {}",
        rect.can_hold(&rect3)
    );

}
