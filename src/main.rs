struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
} // Struct

struct Color(i32, i32, i32); // Tuple struct

struct AlwaysEqual; // Unit struct

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("username"),
        email: String::from("fake@email.com"),
        sign_in_count: 5,
    };

    println!("Current info: {}, {}, {}, {}", user1.email, user1.active, user1.username, user1.sign_in_count);
    user1.email = String::from("new@email.com");
    println!("New email: {}", user1.email);

    let user2 = build_user("user2@email.com".to_string(), "abcdefg".to_string());
    println!("User created with function username: {}", user2.username);

    let user3 = User {
        email: String::from("user3@email.com"),
        ..user1
    };
    println!("User 3 info: {}, {}", user3.username, user2.email);

    let black = Color(0, 0, 0);
    print!("Color values: {}, {}, {}", black.0, black.1, black.2);

    let _unit = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active:true,
        username,
        email,
        sign_in_count: 1,
    }
}
