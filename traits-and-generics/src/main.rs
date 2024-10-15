// Associated Types -> Associated Types is a helpful feature of Rust for improving the readability of code.
struct Index(i32);
trait Joining {
    // Associated types
    type A;
    type B;
    fn join_to_str(&self, _: &Self::A, _: &Self::B) -> String;
}
impl Joining for Index {
    // Define type of associated types
    type A = String;
    type B = String;
    fn join_to_str(&self, name: &Self::A, last_name: &Self::B) -> String {
        format!("{}. {} {}", self.0, name, last_name)
    }
}
fn get_joined_str<J: Joining>(joining: &J, name: &J::A, last_name: &J::B) -> String {
    format!("Person: {}", joining.join_to_str(name, last_name))
}
fn main() {
    let index = Index(10);
    println!(
        "{}",
        get_joined_str(&index, &"John".to_string(), &"Connor".to_string())
    );
}

// Generic Parameters
fn foo<T>(v: &T) {}
// it can't store references so we need to use a lifetime annotation.
struct Struct<'a, T> {
    value: &'a T,
}
// like to previous but the T must to implement the DerivedTrait
struct Struct<'a, T: DerivedTrait> {
    value: &'a T,
}
// T must to implement the DerivedTrait and the DerivedTrait2
struct Struct<'a, T: DerivedTrait + DerivedTrait2> {
    value: &'a T,
}
// T must implement the DerivedTrait and the DerivedTrait2.
struct Struct<'a, T>
where
    T: DerivedTrait + DerivedTrait2,
{
    value: &'a T,
}

// Supertraits -> Similar to inheritance in OOPS
trait Engine {
    fn start(&mut self);
    fn stop(&mut self);
    fn state(&self) -> bool;
}
trait Transmission {
    fn set_gear(&mut self, _: i32);
    fn gear(&self) -> i32;
}
trait Vehicle: Engine + Transmission {
    fn wheel_count(&self) -> u32;
}
trait Car: Vehicle {
    fn fuel_type(&self) -> FuelType;
}

// Creating trail objects -> The Rust doesn't have polymorphism, but there is a similar feature - dispatch. Dispatch may be static or dynamic.
trait Vehicle {
    fn wheel_count(&self) -> u32;
}
#[derive(Default)]
struct Car;
impl Vehicle for Car {
    fn wheel_count(&self) -> u32 {
        4
    }
}
#[derive(Default)]
struct Motorcycle;
impl Vehicle for Motorcycle {
    fn wheel_count(&self) -> u32 {
        2
    }
}
fn wheel_count_static<T: Vehicle>(obj: &T) -> u32 {
    obj.wheel_count()
}
fn wheel_count_dynamic(obj: &dyn Vehicle) -> u32 {
    obj.wheel_count()
}
fn main() {
    let car = Car::default();
    println!("{}", wheel_count_static(&car));
    println!("{}", wheel_count_static(&car));

    let motorcycle = Motorcycle::default();
    println!("{}", wheel_count_static(&motorcycle));
    println!("{}", wheel_count_static(&motorcycle));
}

// Trait Objects versus Enumerations -> Second method of polymorphism in rust is enum.
enum Vehicle {
    Car { wheel_count: u32 },
    Motorcycle { wheel_count: u32 },
}
impl Vehicle {
    fn new_car() -> Self {
        Self::Car { wheel_count: 4 }
    }
    fn new_motorcycle() -> Self {
        Self::Motorcycle { wheel_count: 2 }
    }
    fn wheel_count(&self) -> u32 {
        match self {
            Vehicle::Car { wheel_count, .. } => *wheel_count,
            Vehicle::Motorcycle { wheel_count, .. } => *wheel_count,
        }
    }
}
fn main() {
    let car = Vehicle::new_car();
    println!("{}", car.wheel_count());

    let motorcycle = Vehicle::new_motorcycle();
    println!("{}", motorcycle.wheel_count());
}
