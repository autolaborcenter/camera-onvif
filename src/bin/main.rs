use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("192.168.188.88:8080")?;
    // let packet = camera_onvif::build_ptz((0.0,0.0,0.0));
    let packet = camera_onvif::build_reset();
    println!("{}", packet);
    stream.write_all(packet.as_bytes())?;

    let mut respond = String::new();
    stream.read_to_string(&mut respond)?;
    println!("{}", respond);
    Ok(())
}
