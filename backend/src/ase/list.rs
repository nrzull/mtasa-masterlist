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
        if let Some(list) = &LIST {
            Some(list.lock().unwrap().clone())
        } else {
            None
        }
    }
}

/// Fetches raw data from MTA masterlist.
/// It will not fetch if raw data from MTA masterlist wasn't changed since last fetch
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

    if let Some(header) = headers.get("Last-Modified") {
        let header = header.to_owned().to_str().unwrap().to_owned();

        unsafe {
            if let Some(previous_header) = &LAST_MODIFIED_HEADER {
                if previous_header == &header {
                    continue_fetch = false
                } else {
                    LAST_MODIFIED_HEADER = Some(header);
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

/// Forces fetching of raw data from MTA masterlist, processes and then stores it
fn fetch_force() -> Result<(), String> {
    let response = reqwest::get(URI);

    if let Err(_) = response {
        return Err("[GET] can't fetch".to_owned());
    }

    let mut buffer = vec![];
    response.unwrap().copy_to(&mut buffer).unwrap();

    let servers = process(Bytes::from(buffer));

    unsafe {
        if let Some(list) = &LIST {
            *list.lock().unwrap() = servers;
        }
    }

    Ok(())
}

/// Processes raw data from MTA masterlist and then returns vector of servers
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

    let mut servers: Vec<Server> = vec![];

    for _ in 0..count {
        // Get overall length of buffer that related to processing server
        let entry_length = get_u16(&data, &mut offset);

        // Get starting point for next server buffer
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

        // Skip gamename data
        if has_game_name != 0 {
            get_string(&data, &mut offset);
        }

        if has_server_name != 0 {
            server.name = Some(get_string(&data, &mut offset));
        }

        // Skip gamemode data
        if has_game_mode != 0 {
            get_string(&data, &mut offset);
        }

        // Skip mapname data
        if has_map_mame != 0 {
            get_string(&data, &mut offset);
        }

        if has_server_version != 0 {
            server.version = Some(get_string(&data, &mut offset));
        }

        if has_passworded_flag != 0 {
            server.password = Some(get_u8(&data, &mut offset));
        }

        servers.push(server);
        offset = next_offset;
    }

    servers
}

/// Takes 1 byte from buffer and then returns unsigned 8 bit integer
fn get_u8(buffer: &Bytes, offset: &mut usize) -> u8 {
    let buf = buffer.slice(*offset, *offset + 1);
    *offset += buf.len();
    let raw = buf.deref();
    let mut cursor = Cursor::new(raw.to_owned());

    cursor.read_u8().unwrap()
}

/// Takes 2 bytes from buffer, transforms it and then returns unsigned 16 bit integer
fn get_u16(buffer: &Bytes, offset: &mut usize) -> u16 {
    let buf = buffer.slice(*offset, *offset + 2);
    *offset += buf.len();
    let raw = buf.deref();
    let mut cursor = Cursor::new(raw.to_owned());

    cursor.read_u16::<BigEndian>().unwrap()
}

/// Takes 4 bytes from buffer, transforms it and then returns unsigned 32 bit integer
fn get_u32(buffer: &Bytes, offset: &mut usize) -> u32 {
    let buf = buffer.slice(*offset, *offset + 4);
    *offset += buf.len();
    let raw = buf.deref();
    let mut cursor = Cursor::new(raw.to_owned());

    cursor.read_u32::<BigEndian>().unwrap()
}

/// Takes length of string from buffer and then returns related string
fn get_string(buffer: &Bytes, offset: &mut usize) -> String {
    let length = get_u8(buffer, offset);
    let mut string = String::from("");
    let mut character_storage: Vec<u8> = vec![];

    for i in 0..length {
        character_storage.push(get_u8(buffer, offset));

        if i + 1 == length {
            string = String::from_utf8_lossy(&character_storage).into_owned();
        }
    }

    string
}
