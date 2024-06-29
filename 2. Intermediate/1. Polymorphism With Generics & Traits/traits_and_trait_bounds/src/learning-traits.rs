trait Park {
    fn park(&self); // Method signature, no implementation
}

trait Paint {
    fn paint(&self, color: String) {
        println!("painting object: {}", color); // default implementation
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

impl Park for Car {
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
        println!("unloading truck.");
    }
}

impl Park for Truck {
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

    let object = create_printable_object();

    pain_red(&car);
    pain_red(&house);
    pain_red(&object);

    pain_red3(car);
}

// 3 ways to specify trait bounds

// 1. Bounding T with
fn pain_red<T: Paint>(object: &T) {
    object.pain("red".to_owned())
}

// 2. Same as 1/ saying the object must be a reference that impliments Pait
fn pain_red2(object: &impl Paint) {
    object.pain("red".to_owned())
}

// where is usefull when you have miltiple trait bounds
// T must be any type that must impl both Paint and park
fn pain_red3<T>(object: &T) -> bool where T: Paint + Park {
    object.pain("red".to_owned())
}

fn create_printable_object() -> Paint {
    House {}
}