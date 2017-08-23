use std::thread;
use std::io::prelude::*;
use std::fs::File;
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
			Content-Type: application/json; charset=utf-8\r\n\
			Access-Control-Allow-Origin: *\r\n\
			Access-Control-Allow-Credentials: true\r\n\r\n")?;
		write!(&stream, "{{\"requests\": {}}}", visits)?;
	}
    stream.flush()
}

fn serve_static(mut stream: TcpStream) -> std::io::Result<()> {
	let mut s  = [0; 512];
	stream.read(&mut s)?;
	let s = String::from_utf8_lossy(&s[..]);
	
	let mut first = match s.lines().next() {
		Some(x) => x.split_whitespace(),
		None => return Ok(()),
	};

	let method = match first.next() {
		Some(x) => x,
		None => {
			write!(&stream, "HTTP/1.1 500 Internal Server Error\r\n\r\n").unwrap();
			stream.flush().unwrap();
			return Ok(());
		}
	};

	if method == "GET" {
		let mut path = match first.next() {
        	Some(x) => x.trim_left_matches("/"),
        	None => {
        		write!(&stream, "HTTP/1.1 500 Internal Server Error\r\n\r\n").unwrap();
    			stream.flush().unwrap();
    			return Ok(());
        	}
        };
		println!("path requested {}", path);
		if path == "" {
			path = "index.html";
		}

		let content_type = match path.to_lowercase().split(".").last() {
			Some("css") => "text/css",
			Some("html") => "text/html",
			_ => "text/plain",
		};


		if let Ok(mut file) = File::open(path) {
			
			let mut buf = String::new();
			if let Ok(len) = file.read_to_string(&mut buf) {
			
				write!(&mut stream, 
				"HTTP/1.1 200 OK\r\n\
				Server: rustHTTP/0.1.0\r\n\
				Content-Type: {}; charset=utf-8\r\n\
				Content-Length: {}\r\n\
				Access-Control-Allow-Origin: *\r\n\
				Access-Control-Allow-Credentials: true\r\n\r\n", content_type, len)?;
				stream.write(buf.as_bytes())?;
				//std::io::write(&mut stream, buf.as_bytes())?;
			} else {
				write!(&stream, "HTTP/1.1 500 Internal Server Error\r\n\r\n").unwrap();
			}
		} else {
			write!(&stream, "HTTP/1.1 404 File Not Found\r\n\r\n").unwrap();
		}
    	stream.flush()?;
	}
	Ok(())

}

fn main() {
	thread::spawn(move || {
		println!("Server listening on port 80");
		let listener = TcpListener::bind("0.0.0.0:80").unwrap();
		for stream in listener.incoming() {
			thread::spawn(move || serve_static(stream.unwrap()).unwrap());
		}
	});

    println!("Server listening on port 8080");
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
	let map = Arc::new(Mutex::new(HashMap::<IpAddr, u32>::new()));
    for stream in listener.incoming() {
		let m = map.clone();
        thread::spawn(move || handle_connection(stream.unwrap(), m).unwrap());
    }
}
