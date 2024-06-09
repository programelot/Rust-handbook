//Struct defined in following ways.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//builder function
fn build1(username: String, email: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

//build2 is the same with build 1
//It requires the same name for variable and field name
fn build2(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn build3(user: User, username: String, email: String) -> User {
    User {
        active: user.active,
        username,
        email,
        sign_in_count: user.sign_in_count,
    }
}

//build4 is the same with build3
fn build4(user: User, username: String, email: String) -> User {
    User {
        username,
        email,
        ..user
    }
}

//tuple structs are defined as follows.
struct Point(i32, i32, i32);
struct Vector3(i32, i32, i32);

//There is empty struct.
struct emptyStruct;

//Rust program starts with main function
//Function starts with fn
//Function parameter exists in parenthesis.
//Block defined with {} brackets.
fn main() {
    //Struct can be defined as mutable or immutable.
    let user1: User = User {
        active: true,
        username: String::from("James"),
        email: String::from("email@homepage.com"),
        sign_in_count: 0,
    };

    let mut user2 = User {
        active: false,
        username: String::from("John"),
        email: String::from("John@homepage.com"),
        sign_in_count: 1,
    };

    user2.active = true;

    //struct can be generated from builder function
    let user = build1(String::from("Tom"), String::from("Tom@hoomepage.com"));

    //If field has the same name with variable, it can be shorten.
    let user = build2(String::from("Tom"), String::from("Tom@hoomepage.com"));

    //builder can use other struct's variable.
    //It takes ownership.
    let user = build3(
        user,
        String::from("Jack"),
        String::from("Jack@hoomepage.com"),
    );

    //If it takes most of fileds of other struct, it can be shorten.
    let user = build4(
        user,
        String::from("Risa"),
        String::from("Jack@hoomepage.com"),
    );

    //If struct didn't take ownership, struct can be reused.
    //Following two cases are the cases that is allowed and not allowed.
    let user1 = User {
        active: false,
        username: String::from("Lisa"),
        email: String::from("Lisa@homepage.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        username: String::from("Lee"),
        email: String::from("Lee@homepage.com"),
        ..user1
    };

    //user1 can be printed since it only takes variables with copy traits.
    println!(
        "{0} {1} {2} {3}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );
    println!(
        "{0} {1} {2} {3}",
        user2.active, user2.username, user2.email, user2.sign_in_count
    );
    println!("=====================================");
    // false Lisa Lisa@homepage.com 1
    // false Lee Lee@homepage.com 1
    // =====================================

    let user1 = User {
        active: false,
        username: String::from("Lisa"),
        email: String::from("Lisa@homepage.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        username: String::from("Lee"),
        ..user1
    };

    //It can't be printed since user1's email has been taken to the user 2.
    //println!(
    //    "{0} {1} {2} {3}", user1.active, user1.username, user1.email, user1.sign_in_count
    //);
    println!(
        "{0} {1} {2} {3}",
        user2.active, user2.username, user2.email, user2.sign_in_count
    );

    //two variables are defined as tuple structs.
    //It is used like tuples but with name.
    //It notices they are different types when using it.
    let a = Point(1, 1, 1);
    let b = Vector3(1, 1, 1);

    let c = emptyStruct;
}
