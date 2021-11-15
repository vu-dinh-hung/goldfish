mod model;
mod controller;
mod utilities;
mod filesystem;
mod networking;
mod input;

mod display;


fn main() {
    println!("Hello Goldfishes!");
    match model::add_track_file("README.md") {
        Some(e) => println!("{:?}", e),
        None => println!("Success"),
    }
    input::initialize();
}
