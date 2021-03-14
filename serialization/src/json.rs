use serde::{ Serialize, Deserialize };
use serde_json;
use serde_derive;

use std::net::{ TcpListener, TcpStream };
use std::io::{ stdin, BufRead, BufReader, Error, Write };
use std::{ env, str, thread };

#[derive(Serialize, Deserialize, Debug)]
struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}
// TCP Function handling a single client
fn handle_client(stream: TcpStream) {
    println!("Incoming connection from: {}", stream);
    let mut data = Vec::new()
    let mut stream = BufReader::new(stream);
    loop {
        data.clear()
        let bytes_read = stream.read_until(b'\n', &mut data)?;
        if bytes_read == 0 {
            return Ok(());
        }

        let input: Point3D = serde_json::from_slice(&data);
    }
}