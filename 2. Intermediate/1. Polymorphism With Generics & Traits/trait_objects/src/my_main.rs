trait Vehicle: Paint {
    fn park(&self);
    fn get_default_color() -> String {
        "black".to_owned()
    }
}

trait Paint {
    fn paint(&self, color: String) {
        println!("painting object: {}", color);
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16,
}

struct Car {
    info: VehicleInfo,
}

impl Vehicle for Car {
    fn park(&self) {
        println!("parking car!");
    }
}

impl Paint for Car {}

struct Truck {
    info: VehicleInfo,
}

impl Truck {
    fn unload(&self) {
        println!("unloading truck.")
    }
}

impl Vehicle for Truck {
    fn park(&self) {
        println!("parking truck!");
    }
}

impl Paint for Truck {}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("painting house: {}", color);
    }
}

fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "Honda".to_owned(),
            model: "Civic".to_owned(),
            year: 1995,
        }
    };
    let house = House {};
    let object = create_paintable_object(true);

    let printable_object: Vec<&dyn Paint> = vec![&car, &house, object.as_ref()];

    paint_red(&car);
    paint_red(&house);
    paint_red(object.as_ref());

    paint_vehicle_red(&car);
}

// fn paint_red<T: Paint>(object: &T) {
//     object.paint("red".to_owned());
// }

fn paint_red(object: &dyn Paint) {
    object.paint("red".to_owned());
}

fn paint_vehicle_red<T>(object: &T) where T: Vehicle {
    object.paint("red".to_owned());
}

fn create_paintable_object(vehile: bool) -> Box<dyn Paint> {
    if vehile {
        Box::new(Car {
            info: VehicleInfo {
                make: "Toyota".to_owned(),
                model: "Corolla".to_owned(),
                year: 2005,
            }
        })
    } else {
        Box::new(House {})
    }
}

// Static Dispatch vs Dynamic Dispatch
// Rust uses static dispatch by default, which means that the compiler knows at compile time which method to call.


