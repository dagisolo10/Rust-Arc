struct Temperature {
    degree_f: f64,
}

impl Temperature {
    fn show_temp(&self) {
        println!("{} degrees F", self.degree_f);
    }

    fn freezing() -> Self {
        Self {
            degree_f: 32.0,
        }
    }

    fn boiling() -> Self {
        Self {
            degree_f: 212.0,
        }
    }
}

fn main() {
    let hot = Temperature { degree_f: 99.9 };
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();

    let boiling = Temperature::boiling();
    boiling.show_temp();
}
