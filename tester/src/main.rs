// mod object;
// use crate::object::car::Car;

// fn main() {
//     println!("Hello, world!");
//     let _car1 = Car(3);
//     println!("{:#?}", _car1.0);
// }

struct Circle {
    radius: f64,
    area: f64,
}

impl Circle {
    const PI: f64 = 3.14159;

    fn new(radius: f64) -> Circle {
        let area = Circle::calculate_area(radius);
        Circle { radius, area }
    }

    fn calculate_area(radius: f64) -> f64 {
        Circle::PI * radius * radius
    }
}

fn main() {
    let circle = Circle::new(5.0);
    println!("Bán kính: {}", circle.radius);
    println!("Diện tích: {}", circle.area);
    println!("Hằng số PI: {}", Circle::PI);
}
