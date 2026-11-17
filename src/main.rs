enum Flavor {
    Sweet,
    Fruity,
    Sparkling,
}

struct Drink {
    fluid_oz: f64,
    flavor: Flavor,
}

fn main() {
    print_drink(Drink {
        fluid_oz: 6.0,
        flavor: Flavor::Sweet,
    });

    print_drink(Drink {
        fluid_oz: 2.5,
        flavor: Flavor::Sparkling,
    });

    print_drink(Drink {
        fluid_oz: 1.0,
        flavor: Flavor::Fruity,
    });
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Sweet => println!("Flavor: Sweet"),
        Flavor::Fruity => println!("Flavor: Fruity"),
        Flavor::Sparkling => println!("Flavor: Sparkling"),
    }

    println!("oz: {}", drink.fluid_oz);
}
