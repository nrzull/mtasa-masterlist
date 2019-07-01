use mtasa_masterlist::ase::list::{self, Server};
use std::sync::{mpsc, Arc, Mutex};

fn main() {
    let (tx, rx) = mpsc::channel::<Option<Vec<Server>>>();
    list::get(Arc::new(Mutex::new(tx)));

    if let Ok(Some(v)) = rx.recv() {
        println!("{:?}", v);
    }
}
