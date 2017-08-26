use std::thread;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, IpAddr};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

fn handle_connection(mut stream: TcpStream, dict: Arc<Mutex<HashMap<IpAddr, u32>>>) -> std::io::Result<()> {
	let mut s  = [0; 512];
	stream.read(&mut s)?;
	let s = String::from_utf8_lossy(&s);
	let mut s = s.lines();
	let mut ip = stream.peer_addr().unwrap().ip();
	while let Some(line) = s.next() {
		let mut header = line.split(":");
		let (key, val) = (header.next(), header.next());
		match (key, val) {
			(Some("X-Forwarded-For"), Some(addr)) => {
				ip = String::from(addr.trim_left().trim_right()).parse().unwrap_or(ip);
			},
			_ => (),
		};
	}

	let lock = dict.lock();
	if let Ok(mut map) = lock {
		*map.entry(ip).or_insert(1) += 1;

		let visits = map[&ip];

		write!(&mut stream, 
			"HTTP/1.1 200 OK\r\n\
			Server: rustHTTP/0.1.0\r\n\
			Content-Type: application/json; charset=utf-8\r\n\r\n")?;
		write!(&stream, "{{\"requests\": {}, \"address\": \"{}\"}}", visits, ip)?;
	}
    stream.flush()
}

fn main() {
    println!("Server listening on port 8080");
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
	let map = Arc::new(Mutex::new(HashMap::<IpAddr, u32>::new()));
    for stream in listener.incoming() {
		let m = map.clone();
        thread::spawn(move || handle_connection(stream.unwrap(), m).unwrap());
    }
}
