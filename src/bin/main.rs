use std::{io::{Read, Write}, net::TcpStream};
use camera_onvif::{build_reset, build_ptz};

fn main() {
    let mut stream = TcpStream::connect("192.168.188.88:8080").unwrap();
    // let packet = build_ptz((0.0,0.0,0.0));
    let packet = build_reset();
    println!("{}", packet);
    println!("{:?}", stream.write_all(packet.as_bytes()));

    let mut respond = String::new();
    stream.read_to_string(&mut respond);
    println!("{}", respond);
}
