/*
    Struct:
    Are like tuples but they are more flexible and can be sorted using field names
    It's just like object in OOP languages
    */
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

/*
    Struct ownership, we have been clearing string variables as String, because we wanted the struct
    To own it's data.
    It is possible for struct to store reference to data owned by something else
    */
// This would fail
struct Person {
    user_name: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // Using Struct, they don't have to be declared in a particular structure
    let user1 = User {
        active: true,
        username: String::from("creativeJoe"),
        sign_in_count: 1,
        email: String::from("bla@gmail.com")
    };

    // Mutable struct
    let mut user2 = User {
        active: true,
        sign_in_count: 1,
        username: String::from("creativeJoe"),
        email: String::from("bla@gmail.com")
    };

    // We can change content in a field by calling the dot notation
    user2.email = String::from("hy@gmail.com");

    /*
    We can create a new struct instance from another one just like spread operation in JS
    And similar to splice
    */
    user3 = User {
        email: String::from("jj@gmail.com"),
        username: String::from("Ogochukwu"),
        ..user2
    };

    /*
    We can create what is called Tuple Struct, which is like tuple but with a struct type
    */
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(4,5,6);
    println!("Black is {}", black.0);

    // Reference of Struct ownership
    // THis would fail
    let person1 = Person {
        email: "ll@gmail.com",
        user_name: "ioo",
        active: true,
        sign_in_count: 2
    };
}

fn build_user(email: String, username: String) -> User {
    /*
     If field name and variable name is same, variable can be passed into the struct just
     Like JS
     */
    User {
        username,
        active: true,
        sign_in_count: 1,
        email
    }
}
