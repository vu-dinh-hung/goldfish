mod model;
mod controller;
mod utilities;
mod filesystem;
mod networking;
mod input;
mod display;


fn main() {
    println!("Hello Goldfishes!");
    controller::init();
    match model::add_track_file("README.md") {
        Some(e) => println!("{:?}", e),
        None => println!("Success"),
    }
    match controller::commit() {
        Some(e) => println!("{:?}", e),
        None => println!("Success"),
    }
    input::initialize();
}
