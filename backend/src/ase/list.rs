use byteorder::{BigEndian, ReadBytesExt};
use bytes::Bytes;
use hyper::rt::{self, Future, Stream};
use hyper::Client;
use hyper_tls::HttpsConnector;
use std::io::Cursor;
use std::ops::Deref;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;

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

#[derive(Debug)]
pub struct Server {
    pub ip: String,
    pub port: u16,
    pub players: u16,
    pub maxplayers: u16,
    pub gamename: String,
    pub name: String,
    pub gamemode: String,
    pub map: String,
    pub version: String,
    pub password: u8,
    pub serials: u8,
    pub playerlist: Vec<String>,
    pub responding: u8,
    pub restriction: u32,
    pub searchignore: Vec<(u8, u8)>,
    pub keep: u8,
    pub http: u16,
    pub special: u8,
}

impl Server {
    fn new() -> Server {
        Server {
            ip: String::from(""),
            port: 0,
            players: 0,
            maxplayers: 0,
            gamename: String::from(""),
            name: String::from(""),
            gamemode: String::from(""),
            map: String::from(""),
            version: String::from(""),
            password: 0,
            serials: 0,
            playerlist: vec![],
            responding: 0,
            restriction: 0,
            searchignore: vec![],
            keep: 0,
            http: 0,
            special: 0,
        }
    }
}

pub fn get(tx: Arc<Mutex<Sender<Option<Vec<Server>>>>>) {
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

        server.ip = String::from(format!("{}.{}.{}.{}", ip4, ip3, ip2, ip1));
        server.port = get_u16(&data, &mut offset);

        if has_player_count != 0 {
            server.players = get_u16(&data, &mut offset);
        }

        if has_max_player_count != 0 {
            server.maxplayers = get_u16(&data, &mut offset);
        }

        if has_game_name != 0 {
            server.gamename = get_string(&data, &mut offset);
        }

        if has_server_name != 0 {
            server.name = get_string(&data, &mut offset);
        }

        if has_game_mode != 0 {
            server.gamemode = get_string(&data, &mut offset);
        }

        if has_map_mame != 0 {
            server.map = get_string(&data, &mut offset);
        }

        if has_server_version != 0 {
            server.version = get_string(&data, &mut offset);
        }

        if has_passworded_flag != 0 {
            server.password = get_u8(&data, &mut offset);
        }

        if has_serials_slag != 0 {
            server.serials = get_u8(&data, &mut offset);
        }

        if has_player_list != 0 {
            let count = get_u16(&data, &mut offset);

            for _ in 0..count {
                server.playerlist.push(get_string(&data, &mut offset));
            }
        }

        if has_responding_flag != 0 {
            server.responding = get_u8(&data, &mut offset);
        }

        if has_restriction_flags != 0 {
            server.restriction = get_u32(&data, &mut offset);
        }

        if has_search_ignore_sections != 0 {
            let count = get_u8(&data, &mut offset);

            for _ in 0..count {
                let offst = get_u8(&data, &mut offset);
                let lngth = get_u8(&data, &mut offset);
                server.searchignore.push((offst, lngth));
            }
        }

        if has_keep_flag != 0 {
            server.keep = get_u8(&data, &mut offset);
        }

        if has_http_port != 0 {
            server.http = get_u16(&data, &mut offset);
        }

        if has_special != 0 {
            server.special = get_u8(&data, &mut offset);
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

    for _ in 0..length {
        let character = get_u8(buffer, offset);

        if let Ok(v) = std::str::from_utf8(&[character]) {
            string.push_str(v);
        }
    }

    string
}
