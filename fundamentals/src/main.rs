fn main() {
    let some_user = fundamentals::User {
        name: String::from("Name"),
        surname: String::from("Name"),
        job_location: String::from("Name"),
    };

    println!("{}", some_user);
}