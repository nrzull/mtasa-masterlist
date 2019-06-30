use hyper::rt::{self, Future, Stream};
use hyper::Client;
use hyper_tls::HttpsConnector;

use std::ops::Deref;
use std::sync::{Arc, Mutex};
const URI: &str = "https://master.multitheftauto.com/ase/mta/";

pub fn get_info() {
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
                process(data.deref());
                Ok(())
            })
            .map_err(|e| panic!("{}", e))
    }));
}

// TODO
fn process(data: &String) {
    println!("{:?}", data);
}
