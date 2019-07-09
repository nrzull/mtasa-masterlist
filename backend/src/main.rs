use mtasa_masterlist::ase;
use mtasa_masterlist::server;

fn main() {
    ase::list::run();
    server::app::run("127.0.0.1:8080".to_owned());
}
