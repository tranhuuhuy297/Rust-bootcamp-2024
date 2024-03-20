mod object;
use crate::object::car::Car;

fn main() {
    println!("Hello, world!");
    let _car1 = Car(3);
    println!("{:#?}", _car1.0);
}
