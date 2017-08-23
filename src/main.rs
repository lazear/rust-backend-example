use std::thread;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, IpAddr};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

fn handle_connection(mut stream: TcpStream, dict: Arc<Mutex<HashMap<IpAddr, u32>>>) -> std::io::Result<()> {
	let mut s  = [0; 512];
	stream.read(&mut s)?;

	let ip = stream.peer_addr().unwrap().ip();
	let lock = dict.lock();
	if let Ok(mut map) = lock {
		*map.entry(ip).or_insert(1) += 1;

		let visits = map[&ip];

		write!(&mut stream, 
			"HTTP/1.1 200 OK\r\n\
			Server: rustHTTP/0.1.0\r\n\
			Content-Type: text/html; charset=utf-8\r\n\
			Access-Control-Allow-Origin: *\r\n\
			Access-Control-Allow-Credentials: true\r\n\r\n")?;
		write!(&stream, "\r\n{{\"requests\": {}}}", visits)?;
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
