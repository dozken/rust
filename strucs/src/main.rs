#[derive(Debug)]
enum CarModel {
    Toyota,
    Honda,
    Hyundai,
}
struct Car {
    name: String,
    number: u8,
    model: CarModel,
}

impl Car {
    fn drive(self, speed: u16) {
        if speed > 0 {
            println!("{} Riders in the storm!!! 🏎️", self.name);
        } else {
            println!("{} rider, start the engine 😠 ", self.name)
        }
    }

    fn borrowed_drive(&self, speed: u16) {
        if speed > 0 {
            println!("{} Riders in the storm!!! 🏎️", self.name);
        } else {
            println!("{} rider, start the engine 😠 ", self.name)
        }
    }

    fn newToyota(name: String) -> Car {
        Car {
            name: name,
            number: 123,
            model: CarModel::Toyota,
        }
    }
}

fn main() {
    let car = Car {
        name: String::from("Corola"),
        number: 123,
        model: CarModel::Toyota,
    };

    println!("Car model {:?}, and it is: {}", car.model, car.name);

    car.drive(60);
    // drive(car, 60);

    let car2 = Car::newToyota(String::from("Prius"));
    car2.borrowed_drive(0);
    car2.borrowed_drive(40);
}
