struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("user1"),
        email: String::from("user1@nowhere.com"),
        sign_in_count: 1,
        is_active: true,
    };
    // user1.email = String::from("can we do this?"); // we can't do this because email is immutable
    let mut user2 = User {
        username: String::from("user1"),
        email: String::from("user1@nowhere.com"),
        sign_in_count: 1,
        is_active: true,
    };
    user2.username = String::from("user2");
}
