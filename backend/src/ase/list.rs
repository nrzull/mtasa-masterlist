use crate::utils;
use byteorder::{BigEndian, ReadBytesExt};
use bytes::Bytes;
use reqwest;
use serde::Serialize;
use std::io::Cursor;
use std::ops::Deref;
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

static mut LAST_MODIFIED_HEADER: Option<String> = None;
static mut LIST: Option<Arc<Mutex<Vec<Server>>>> = None;

#[derive(Debug, Clone, Serialize)]
pub struct Server {
    pub ip: Option<String>,
    pub port: Option<u16>,
    pub players: Option<u16>,
    pub maxplayers: Option<u16>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub password: Option<u8>,
}

impl Server {
    fn new() -> Server {
        Server {
            ip: None,
            port: None,
            players: None,
            maxplayers: None,
            name: None,
            version: None,
            password: None,
        }
    }
}

pub fn run() {
    unsafe {
        LIST = Some(Arc::new(Mutex::new(vec![])));
    }

    fetch().expect("can't fetch ase list");
    fetch_loop();
}

fn fetch_loop() {
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(30));

        match fetch() {
            Ok(_) => fetch_loop(),
            Err(err) => {
                eprintln!("{}", err);
                fetch_loop()
            }
        }
    });
}

pub fn get() -> Option<Vec<Server>> {
    unsafe {
        if let Some(v) = &LIST {
            Some(v.lock().unwrap().clone())
        } else {
            None
        }
    }
}

pub fn fetch() -> Result<(), String> {
    let head_client = reqwest::Client::new();
    let response = head_client.head(URI).send();

    if let Err(_) = response {
        return Err("[HEAD] can't fetch".to_owned());
    };

    unsafe {
        if let None = &LAST_MODIFIED_HEADER {
            LAST_MODIFIED_HEADER = Some(String::from(""));
        }
    }

    let response = response.unwrap();
    let headers = response.headers();
    let mut continue_fetch = true;

    if let Some(v) = headers.get("Last-Modified") {
        let v = v.to_owned().to_str().unwrap().to_owned();

        unsafe {
            if let Some(a) = &LAST_MODIFIED_HEADER {
                if a == &v {
                    continue_fetch = false
                } else {
                    LAST_MODIFIED_HEADER = Some(v);
                }
            }
        }
    } else {
        continue_fetch = false
    }

    if continue_fetch {
        match fetch_force() {
            Ok(_) => (),
            Err(text) => return Err(text),
        }
    }

    Ok(())
}

fn fetch_force() -> Result<(), String> {
    let response = reqwest::get(URI);

    if let Err(_) = response {
        return Err("[GET] can't fetch".to_owned());
    }

    let mut buffer = vec![];
    response.unwrap().copy_to(&mut buffer).unwrap();

    let servers = process(Bytes::from(buffer));

    unsafe {
        if let Some(v) = &LIST {
            let mut data = v.lock().unwrap();
            *data = servers;
        }
    }

    Ok(())
}

fn process(data: Bytes) -> Vec<Server> {
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
            get_string(&data, &mut offset);
        }

        if has_server_name != 0 {
            server.name = Some(get_string(&data, &mut offset));
        }

        if has_game_mode != 0 {
            get_string(&data, &mut offset);
        }

        if has_map_mame != 0 {
            get_string(&data, &mut offset);
        }

        if has_server_version != 0 {
            server.version = Some(get_string(&data, &mut offset));
        }

        if has_passworded_flag != 0 {
            server.password = Some(get_u8(&data, &mut offset));
        }

        if has_serials_slag != 0 {
            get_u8(&data, &mut offset);
        }

        if has_player_list != 0 {
            let count = get_u16(&data, &mut offset);

            for _ in 0..count {
                get_string(&data, &mut offset);
            }
        }

        if has_responding_flag != 0 {
            get_u8(&data, &mut offset);
        }

        if has_restriction_flags != 0 {
            get_u32(&data, &mut offset);
        }

        if has_search_ignore_sections != 0 {
            let count = get_u8(&data, &mut offset);

            for _ in 0..count {
                get_u8(&data, &mut offset);
                get_u8(&data, &mut offset);
            }
        }

        if has_keep_flag != 0 {
            get_u8(&data, &mut offset);
        }

        if has_http_port != 0 {
            get_u16(&data, &mut offset);
        }

        if has_special != 0 {
            get_u8(&data, &mut offset);
        }

        servers.push(server);
        offset = next_offset;
    }

    servers
}

fn get_u8(buffer: &Bytes, offset: &mut usize) -> u8 {
    let buf = buffer.slice(*offset, *offset + 1);
    *offset += buf.len();
    let raw = buf.deref();
    let mut cursor = Cursor::new(raw.to_owned());

    cursor.read_u8().unwrap()
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
