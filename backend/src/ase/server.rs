use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream, UdpSocket};
use std::str;
use std::time::Duration;

#[derive(Debug)]
pub struct Server {
    port: String,
    name: String,
    gamemode: String,
    map: String,
    version: String,
    passworded: String,
    playercount: String,
    maxplayers: String,
    ip: String,
}

pub fn get_info(ip: &str, port: u16) -> Option<Server> {
    let (tcp, udp) = get_tcp_and_udp_addrs(ip, port);

    let buffer = match get_buffer(tcp, udp) {
        Some(v) => v,
        None => return None,
    };

    let buffer = buffer.as_slice();
    let buffer = &buffer[8..];

    let mut start = 0;
    let mut length = 0;
    let mut end = 0;

    let server = Server {
        ip: ip.to_owned(),
        port: process(&buffer, &mut start, &mut end, &mut length),
        name: process(&buffer, &mut start, &mut end, &mut length),
        gamemode: process(&buffer, &mut start, &mut end, &mut length),
        map: process(&buffer, &mut start, &mut end, &mut length),
        version: process(&buffer, &mut start, &mut end, &mut length),
        passworded: process(&buffer, &mut start, &mut end, &mut length),
        playercount: process(&buffer, &mut start, &mut end, &mut length),
        maxplayers: process(&buffer, &mut start, &mut end, &mut length),
    };

    Some(server)
}

fn get_tcp_and_udp_addrs(ip: &str, port: u16) -> (SocketAddr, SocketAddr) {
    let ip: Vec<u8> = ip.split('.').map(|v| v.parse::<u8>().unwrap()).collect();

    let ip = IpAddr::V4(Ipv4Addr::new(
        *ip.get(0).unwrap(),
        *ip.get(1).unwrap(),
        *ip.get(2).unwrap(),
        *ip.get(3).unwrap(),
    ));

    (SocketAddr::new(ip, 80), SocketAddr::new(ip, port + 123))
}

fn get_buffer(tcp: SocketAddr, udp: SocketAddr) -> Option<Vec<u8>> {
    let timeout = Duration::from_secs(1);
    let request_buffer = [115];

    let _streamer = match TcpStream::connect_timeout(&tcp, timeout) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("failed to connect to tcp remote server. {:?}", err);
            return None;
        }
    };

    let udp_socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(v) => v,
        Err(err) => {
            eprintln!("failed to connect to udp localhost server. {:?}", err);
            return None;
        }
    };

    match udp_socket.set_read_timeout(Some(timeout)) {
        Ok(_) => (),
        Err(_) => return None,
    };

    match udp_socket.set_write_timeout(Some(timeout)) {
        Ok(_) => (),
        Err(_) => return None,
    };

    match udp_socket.connect(udp) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("failed to connect to udp remote server. {:?}", err);
            return None;
        }
    };

    let mut response = [0; 1024 * 100];

    match udp_socket.send(&request_buffer) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("failed to send udp handshake {:?}", err);
            return None;
        }
    };

    match udp_socket.recv(&mut response) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("failed to receive from udp {:?}", err);
            return None;
        }
    };

    Some(
        response
            .iter()
            .filter(|v| if **v == 0u8 { false } else { true })
            .map(|v| *v)
            .collect::<Vec<u8>>(),
    )
}

fn process<'a>(buf: &'a [u8], start: &mut usize, end: &mut usize, length: &mut usize) -> String {
    let length_char_length = 1;

    *end = *start + length_char_length;
    *length = *(&buf[*start..*end][0]) as usize - length_char_length;
    *start = *end;
    *end = *start + *length;
    let slice = &buf[*start..*end];
    *start = *end;

    String::from_utf8_lossy(slice).into_owned()
}
