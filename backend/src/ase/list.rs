use crate::utils;
use byteorder::{BigEndian, ReadBytesExt};
use bytes::Bytes;
use hyper::rt::{self, Future, Stream};
use hyper::Client;
use hyper_tls::HttpsConnector;
use std::io::Cursor;
use std::ops::Deref;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const URI: &str = "https://master.multitheftauto.com/ase/mta/";
const ASE_HAS_PLAYER_COUNT: u32 = 0x0004;
const ASE_HAS_MAX_PLAYER_COUNT: u32 = 0x0008;
const ASE_HAS_GAME_NAME: u32 = 0x0010;
const ASE_HAS_SERVER_NAME: u32 = 0x0020;
const ASE_HAS_GAME_MODE: u32 = 0x0040;
const ASE_HAS_MAP_NAME: u32 = 0x0080;
const ASE_HAS_SERVER_VERSION: u32 = 0x0100;
const ASE_HAS_PASSWORDED_FLAG: u32 = 0x0200;
const ASE_HAS_SERIALS_FLAG: u32 = 0x0400;
const ASE_HAS_PLAYER_LIST: u32 = 0x0800;
const ASE_HAS_RESPONDING_FLAG: u32 = 0x1000;
const ASE_HAS_RESTRICTION_FLAGS: u32 = 0x2000;
const ASE_HAS_SEARCH_IGNORE_SECTIONS: u32 = 0x4000;
const ASE_HAS_KEEP_FLAG: u32 = 0x8000;
const ASE_HAS_HTTP_PORT: u32 = 0x080000;
const ASE_HAS_SPECIAL_FLAGS: u32 = 0x100000;

static mut CAN_FETCH: bool = true;
static mut CACHED_LIST: Option<Vec<Server>> = None;

#[derive(Debug, Clone)]
pub struct Server {
    pub ip: Option<String>,
    pub port: Option<u16>,
    pub players: Option<u16>,
    pub maxplayers: Option<u16>,
    pub gamename: Option<String>,
    pub name: Option<String>,
    pub gamemode: Option<String>,
    pub map: Option<String>,
    pub version: Option<String>,
    pub password: Option<u8>,
    pub serials: Option<u8>,
    pub playerlist: Option<Vec<String>>,
    pub responding: Option<u8>,
    pub restriction: Option<u32>,
    pub searchignore: Option<Vec<(u8, u8)>>,
    pub keep: Option<u8>,
    pub http: Option<u16>,
    pub special: Option<u8>,
}

impl Server {
    fn new() -> Server {
        Server {
            ip: None,
            port: None,
            players: None,
            maxplayers: None,
            gamename: None,
            name: None,
            gamemode: None,
            map: None,
            version: None,
            password: None,
            serials: None,
            playerlist: None,
            responding: None,
            restriction: None,
            searchignore: None,
            keep: None,
            http: None,
            special: None,
        }
    }
}

pub fn get() -> Option<Vec<Server>> {
    unsafe {
        if !CAN_FETCH {
            if let Some(v) = &CACHED_LIST {
                return Some(v.clone());
            }
        }
    }

    let (tx, rx) = mpsc::channel::<Option<Vec<Server>>>();
    fetch(Arc::new(Mutex::new(tx)));

    if let Ok(Some(v)) = rx.recv() {
        unsafe {
            CACHED_LIST = Some(v.clone());
        }

        allow_fetch_after(Duration::from_secs(10));

        Some(v)
    } else {
        None
    }
}

fn fetch(tx: Arc<Mutex<Sender<Option<Vec<Server>>>>>) {
    rt::run(rt::lazy(move || {
        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let ok_tx = Arc::clone(&tx);
        let err_tx = Arc::clone(&tx);

        client
            .get(URI.parse().unwrap())
            .and_then(|res| res.into_body().concat2())
            .and_then(move |res| {
                process(ok_tx, res.into_bytes());
                Ok(())
            })
            .map_err(move |e| {
                eprintln!("{}", e);
                err_tx.lock().unwrap().send(None).unwrap();
            })
    }));
}

fn process(tx: Arc<Mutex<Sender<Option<Vec<Server>>>>>, data: Bytes) {
    let mut offset = 0usize;
    let _length = get_u16(&data, &mut offset);
    let _version = get_u16(&data, &mut offset);
    let flags = get_u32(&data, &mut offset);
    let _sequence = get_u32(&data, &mut offset);
    let count = get_u32(&data, &mut offset);

    let has_player_count = flags & ASE_HAS_PLAYER_COUNT;
    let has_max_player_count = flags & ASE_HAS_MAX_PLAYER_COUNT;
    let has_game_name = flags & ASE_HAS_GAME_NAME;
    let has_server_name = flags & ASE_HAS_SERVER_NAME;
    let has_game_mode = flags & ASE_HAS_GAME_MODE;
    let has_map_mame = flags & ASE_HAS_MAP_NAME;
    let has_server_version = flags & ASE_HAS_SERVER_VERSION;
    let has_passworded_flag = flags & ASE_HAS_PASSWORDED_FLAG;
    let has_serials_slag = flags & ASE_HAS_SERIALS_FLAG;
    let has_player_list = flags & ASE_HAS_PLAYER_LIST;
    let has_responding_flag = flags & ASE_HAS_RESPONDING_FLAG;
    let has_restriction_flags = flags & ASE_HAS_RESTRICTION_FLAGS;
    let has_search_ignore_sections = flags & ASE_HAS_SEARCH_IGNORE_SECTIONS;
    let has_keep_flag = flags & ASE_HAS_KEEP_FLAG;
    let has_http_port = flags & ASE_HAS_HTTP_PORT;
    let has_special = flags & ASE_HAS_SPECIAL_FLAGS;

    let mut servers: Vec<Server> = vec![];

    for _ in 0..count {
        let entry_length = get_u16(&data, &mut offset);
        let next_offset = offset + entry_length as usize - 2;
        let mut server = Server::new();

        let (ip1, ip2, ip3, ip4) = (
            get_u8(&data, &mut offset),
            get_u8(&data, &mut offset),
            get_u8(&data, &mut offset),
            get_u8(&data, &mut offset),
        );

        server.ip = Some(String::from(format!("{}.{}.{}.{}", ip4, ip3, ip2, ip1)));
        server.port = Some(get_u16(&data, &mut offset));

        if has_player_count != 0 {
            server.players = Some(get_u16(&data, &mut offset));
        }

        if has_max_player_count != 0 {
            server.maxplayers = Some(get_u16(&data, &mut offset));
        }

        if has_game_name != 0 {
            server.gamename = Some(get_string(&data, &mut offset));
        }

        if has_server_name != 0 {
            server.name = Some(get_string(&data, &mut offset));
        }

        if has_game_mode != 0 {
            server.gamemode = Some(get_string(&data, &mut offset));
        }

        if has_map_mame != 0 {
            server.map = Some(get_string(&data, &mut offset));
        }

        if has_server_version != 0 {
            server.version = Some(get_string(&data, &mut offset));
        }

        if has_passworded_flag != 0 {
            server.password = Some(get_u8(&data, &mut offset));
        }

        if has_serials_slag != 0 {
            server.serials = Some(get_u8(&data, &mut offset));
        }

        if has_player_list != 0 {
            let count = get_u16(&data, &mut offset);
            let mut temp_storage = vec![];

            for _ in 0..count {
                temp_storage.push(get_string(&data, &mut offset));
            }

            server.playerlist = Some(temp_storage);
        }

        if has_responding_flag != 0 {
            server.responding = Some(get_u8(&data, &mut offset));
        }

        if has_restriction_flags != 0 {
            server.restriction = Some(get_u32(&data, &mut offset));
        }

        if has_search_ignore_sections != 0 {
            let count = get_u8(&data, &mut offset);
            let mut temp_storage = vec![];

            for _ in 0..count {
                let offst = get_u8(&data, &mut offset);
                let lngth = get_u8(&data, &mut offset);
                temp_storage.push((offst, lngth));
            }

            server.searchignore = Some(temp_storage);
        }

        if has_keep_flag != 0 {
            server.keep = Some(get_u8(&data, &mut offset));
        }

        if has_http_port != 0 {
            server.http = Some(get_u16(&data, &mut offset));
        }

        if has_special != 0 {
            server.special = Some(get_u8(&data, &mut offset));
        }

        servers.push(server);
        offset = next_offset;
    }

    tx.lock().unwrap().send(Some(servers)).unwrap();
}

fn get_u8(buffer: &Bytes, offset: &mut usize) -> u8 {
    let buf = buffer.slice(*offset, *offset + 1);
    *offset += buf.len();
    let raw = buf.deref();
    let mut cursor = Cursor::new(raw.to_owned());

    cursor.read_u8().unwrap()
}

fn allow_fetch_after(time: std::time::Duration) {
    unsafe {
        CAN_FETCH = false;
    }

    thread::spawn(move || {
        thread::sleep(time);

        unsafe {
            CAN_FETCH = true;
        }
    });
}

fn get_u16(buffer: &Bytes, offset: &mut usize) -> u16 {
    let buf = buffer.slice(*offset, *offset + 2);
    *offset += buf.len();
    let raw = buf.deref();
    let mut cursor = Cursor::new(raw.to_owned());

    cursor.read_u16::<BigEndian>().unwrap()
}

fn get_u32(buffer: &Bytes, offset: &mut usize) -> u32 {
    let buf = buffer.slice(*offset, *offset + 4);
    *offset += buf.len();
    let raw = buf.deref();
    let mut cursor = Cursor::new(raw.to_owned());

    cursor.read_u32::<BigEndian>().unwrap()
}

fn get_string(buffer: &Bytes, offset: &mut usize) -> String {
    let length = get_u8(buffer, offset);
    let mut string = String::from("");

    let mut utf8_storage: Vec<u8> = vec![];
    for i in 0..length {
        let character = get_u8(buffer, offset);
        utf8_storage.push(character);

        if i + 1 == length {
            if let Ok(v) = std::str::from_utf8(&utf8_storage) {
                string.push_str(v);
            } else {
                string.push_str(&utils::get_safe_string(utf8_storage.clone()));
            }
        }
    }

    string
}
