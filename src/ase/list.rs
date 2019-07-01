use hyper::rt::{self, Future, Stream};
use hyper::Client;
use hyper_tls::HttpsConnector;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

const URI: &str = "https://master.multitheftauto.com/ase/mta/";
const ASE_HAS_PLAYER_COUNT: i32 = 0x0004;
const ASE_HAS_MAX_PLAYER_COUNT: i32 = 0x0008;
const ASE_HAS_GAME_NAME: i32 = 0x0010;
const ASE_HAS_SERVER_NAME: i32 = 0x0020;
const ASE_HAS_GAME_MODE: i32 = 0x0040;
const ASE_HAS_MAP_NAME: i32 = 0x0080;
const ASE_HAS_SERVER_VERSION: i32 = 0x0100;
const ASE_HAS_PASSWORDED_FLAG: i32 = 0x0200;
const ASE_HAS_SERIALS_FLAG: i32 = 0x0400;
const ASE_HAS_PLAYER_LIST: i32 = 0x0800;
const ASE_HAS_RESPONDING_FLAG: i32 = 0x1000;
const ASE_HAS_RESTRICTION_FLAGS: i32 = 0x2000;
const ASE_HAS_SEARCH_IGNORE_SECTIONS: i32 = 0x4000;
const ASE_HAS_KEEP_FLAG: i32 = 0x8000;
const ASE_HAS_HTTP_PORT: i32 = 0x080000;
const ASE_HAS_SPECIAL_FLAGS: i32 = 0x100000;

pub fn get() {
    rt::run(rt::lazy(|| {
        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let buffer: Arc<Mutex<String>> = Arc::new(Mutex::new(String::from("")));

        let link = Arc::clone(&buffer);
        let link2 = Arc::clone(&buffer);

        client
            .get(URI.parse().unwrap())
            .and_then(|res| {
                res.into_body().for_each(move |chunk| {
                    let mut data = link.lock().unwrap();
                    let chunk = format!("{:?}", chunk);
                    data.push_str(chunk.as_str());

                    Ok(())
                })
            })
            .and_then(move |_| {
                let data = link2.lock().unwrap();
                let data = data.deref().clone();
                process(data);

                Ok(())
            })
            .map_err(|e| panic!("{}", e))
    }));
}

// TODO
fn process(data: String) {
    println!("{:?}", data);
}
