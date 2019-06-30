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
}

pub fn get_server_info(ip: &str, port: u16) -> Server {
    let (tcp, udp) = get_tcp_and_udp_addrs(ip, port);

    let buffer = get_server_info_buffer(tcp, udp);
    let buffer = buffer.as_slice();
    let buffer = &buffer[8..];

    let mut start = 0;
    let mut length = 0;
    let mut end = 0;

    let port = process(&buffer, &mut start, &mut end, &mut length);
    let name = process(&buffer, &mut start, &mut end, &mut length);
    let gamemode = process(&buffer, &mut start, &mut end, &mut length);
    let map = process(&buffer, &mut start, &mut end, &mut length);
    let version = process(&buffer, &mut start, &mut end, &mut length);
    let passworded = process(&buffer, &mut start, &mut end, &mut length);
    let playercount = process(&buffer, &mut start, &mut end, &mut length);
    let maxplayers = process(&buffer, &mut start, &mut end, &mut length);

    Server {
        port: port.to_owned(),
        name: name.to_owned(),
        gamemode: gamemode.to_owned(),
        map: map.to_owned(),
        version: version.to_owned(),
        passworded: passworded.to_owned(),
        playercount: playercount.to_owned(),
        maxplayers: maxplayers.to_owned(),
    }
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

fn get_server_info_buffer(tcp: SocketAddr, udp: SocketAddr) -> Vec<u8> {
    let timeout = Duration::from_secs(5);
    let request_buffer = [115];

    let _streamer = TcpStream::connect_timeout(&tcp, timeout)
        .unwrap_or_else(|err| panic!("failed to connect to tcp remote server. {:?}", err));

    let udp_socket = UdpSocket::bind("0.0.0.0:0")
        .unwrap_or_else(|err| panic!("failed to connect to udp localhost server. {:?}", err));

    udp_socket
        .connect(udp)
        .unwrap_or_else(|err| panic!("failed to connect to udp remote server. {:?}", err));

    let mut response = [0; 1024 * 100];

    udp_socket
        .send(&request_buffer)
        .unwrap_or_else(|err| panic!("failed to send udp handshake {:?}", err));

    udp_socket
        .recv(&mut response)
        .unwrap_or_else(|err| panic!("failed to receive from udp {:?}", err));

    response
        .iter()
        .filter(|v| if **v == 0u8 { false } else { true })
        .map(|v| *v)
        .collect::<Vec<u8>>()
}

fn process<'a>(buf: &'a [u8], start: &mut usize, end: &mut usize, length: &mut usize) -> &'a str {
    let length_char_length = 1;

    *end = *start + length_char_length;
    *length = *(&buf[*start..*end][0]) as usize - length_char_length;
    *start = *end;
    *end = *start + *length;
    let slice = &buf[*start..*end];
    *start = *end;

    str::from_utf8(&slice).unwrap()
}
