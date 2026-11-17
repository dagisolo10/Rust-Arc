enum Access {
    User,
    Guest,
    Admin,
    Manager,
}

fn main() {
    let access_level = Access::Guest;

    // only admin can access
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };

    println!("can access file {}", can_access_file);
}
