extern crate opencv;

use opencv::core;

fn main() {
    println!("{}", core::get_build_information().unwrap())
}
