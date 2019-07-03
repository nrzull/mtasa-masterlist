use mtasa_masterlist::ase;
use std::thread;

fn main() {
    if ase::list::run() {
        match ase::list::get() {
            Some(v) => println!("{:?}", v.len()),
            None => println!("None"),
        };

        match ase::list::get() {
            Some(v) => println!("{:?}", v.len()),
            None => println!("None"),
        };
    }
}
