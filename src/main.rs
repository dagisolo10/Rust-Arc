enum Light {
    Dull,
    Bright,
}

fn main() {
    let light = Light::Dull;

    // ! without borrowing
    // display_light(light);
    // display_light(light);

    // * with borrowing
    display_light(&light);
    display_light(&light);
}

// ! taking ownership the value
fn display_light(light: &Light) {
    match light {
        Light::Dull => println!("Light is Dull"),
        Light::Bright => println!("Light is bright"),
    }
}

// * referencing the value
// fn display_light(light: Light) {
//     match light {
//         Light::Dull => println!("Light is Dull"),
//         Light::Bright => println!("Light is bright"),
//     }
// }
