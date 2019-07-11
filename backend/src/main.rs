use mtasa_masterlist::ase;
use mtasa_masterlist::server;

fn main() {
    ase::list::run();
    server::app::run("0.0.0.0:8081".to_owned());
}
