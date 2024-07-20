// main.rs

// import tools module
mod tools;

// import functions from tools
use crate::tools::method_1;
use crate::tools::method_2;

fn main() {
    println!("---------------");
    println!("Running main().");
    println!("---------------");
    // run functions from tools
    method_1();
    method_2();
    tools::method_3();
}

